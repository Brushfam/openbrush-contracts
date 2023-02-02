# Updating ink! smart-contracts' storage from version ink! v3.4.0 to ink! v4.0.0-beta.1

## Update ink! dependencies

In your `Cargo.toml` file, remove `ink_lang`, `ink_storage`, and similar dependencies,
add `ink = { version = "4.0.0-rc", default-features = false }` instead. Also, you need to
use `ink::env`, `ink::storage` etc. instead of `ink_env`, `ink_storage` etc.

## Storage refactoring

In ink! v4.0.0-beta, the way storage works was refactored.

Previously each field was stored in its storage cell under its storage key.
The storage key was calculated in runtime over fields iteration. In the new version, all packed fields are stored in one
storage cell under one storage key. All non-packed fields know their storage key during compilation time.
During compilation time, own storage keys are generated for types, so, for instance, if you have `Mapping<u128, 128>`
type for your storage field, after code generation, the type of this field will be changed to `Mapping<u128, u128, ManualKey<123>>`.

### Traits

The traits `SpreadLayout` and `PackedLayout` were substituted by the traits `Storable` and `Packed`.
Methods `pull_spread` and `push_spread` were replaced with `ink::env::set_contract_storage` and `ink::env::get_contract_storage`.
You can use the macro attribute `#[ink::storage_item]` to implement these traits.

 - `Storable` is a trait derived in types that can be read and written into the contract's storage.
Types that implement `scale::Decode` and `scale::Encode` are storable by default.

You can derive the `Storable` trait for your type as in the following example:
```rust
use ink::storage::traits::Storable;

#[derive(Storable)]
struct MyStruct {
    first_field: u32,
    second_field: Vec<u8>,
}
```

 - `Packed` is a trait that is created for representing types that can be read and written into the contract's storage, 
and all of its fields occupy a single storage cell.

You can derive the `Packed` trait for your type as in the following example:

```rust
use ink::storage::traits::Packed;

#[derive(Packed)]
struct MyPackedStruct {
    first_field: u32,
    second_field: Vec<u8>,
}
```

A type will be considered non-packed if any of its fields occupy its single storage cell. Example of non-packed type:

```rust
#[ink::storage_item]
struct MyNonPackedStruct {
    first_field: u32,
    second_field: Mapping<u32, u32>,
}
```
The type is considered non-packed as `Mapping` always occupies its own storage cell.

 - `#[ink::storage_item]` macro attribute prepares your type to be fully compatible and usable with storage.
It implements all necessary traits and calculates the storage key for types. You can set `#[ink::storage_item(derive = false)]`,
which will indicate that the auto-deriving of all required traits will be disabled. Also, it will be implemented via blanket
implementation for every type that implements `scale::Encode` and `scale::Decode`. The following examples show how to create
type using `ink::storage_item`, `scale::Encode` and `scale::Decode`.

```rust
#[ink::storage_item]
struct MyNonPackedStruct {
    first_field: u32,
    second_field: Mapping<u32, u32>,
}

#[ink::storage_item(derive = false)]
#[derive(Storable, StorableHint, StorageKey)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
struct MyAnotherNonPackedStruct {
    first_field: Mapping<u128, Vec<u8>>,
    second_field: Mapping<u32, u32>,
}

#[derive(scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
struct MyPackedStruct {
    first_field: u32,
    second_field: Vec<u8>,
}
```

Example of nested storage types:
```rust
#[ink::storage_item]
struct NonPacked {
    s1: Mapping<u32, u128>,
    s2: Lazy<u128>,
}

#[derive(scale::Decode, scale::Encode)]
#[cfg_attr(
feature = "std",
derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
struct Packed {
    s1: u128,
    s2: Vec<u128>,
}

#[ink::storage_item]
struct NonPackedComplex<KEY: StorageKey> {
    s1: (String, u128, Packed),
    s2: Mapping<u128, u128>,
    s3: Lazy<u128>,
    s4: Mapping<u128, Packed>,
    s5: Lazy<NonPacked>,
    s6: PackedGeneric<Packed>,
    s7: NonPackedGeneric<Packed>,
}
```

- `StorableHint` is a trait that describes the type that should be used for storing the value and preferred storage key. 
It is implemented automatically for all types that implement `Packed` types.
- `ManualKey` is a generic struct used to set the storage key manually. You can use it as `ManualKey<123>`.
- `AutoKey` is a struct that automatically sets the storage key. You can use it as `AutoKey`.
In this case, the storage key will be calculated while compilation and set as `ManualKey<>` with the calculated storage key.

For example, if you want to use the mapping and you want to set the storage key manually, you can take a look at the following example:
```rust
#[ink::storage_item]
struct MyStruct {
    first_field: u32,
    second_field: Mapping<u32, u32, ManualKey<123>>,
}
```

### Problems
There is a problem with generic fields in non-packed structs. Example:
```rust
#[ink::storage_item]
struct MyNonPackedStruct<D: MyTrait = OtherStruct> {
    first_field: u32,
    second_field: D,
}

struct OtherStruct {
    other_first_field: Mapping<u128, u128>,
    other_second_field: Mapping<u32, Vec<u8>>,
}

trait MyTrait {
    fn do_something(&self);
}

impl MyTrait for OtherStruct {
    fn do_something(&self) {
        // do something
    }
}
```

In this case, the contract cannot be built because it cannot calculate the storage key for the field `second_field` of type `MyTrait`.
You can use packed structs for it or, as a temporary solution, set `ManualKey` as another trait for the field:
```rust
struct MyNonPackedStruct<D: MyTrait + ManualKey<123> = OtherStruct>
```

But instead of `ManualKey<123>`, you should use the key generated during compilation. Packed generics work okay, you can
use them like this:
```rust
#[ink::storage_item]
struct MyNonPackedStruct<D: Packed> {
    first_field: u32,
    second_field: D,
}
```

## Initialization of contract

In ink! v4.0.0-beta.1, `ink_lang::codegen::initialize_contract` was removed. Instead, you can use
`Default` trait. Example:

```rust
#[ink(storage)]
pub struct MyContract {
    value: u32,
    balances: Mapping<u128, u128>,
}

impl MyContract {
    #[ink(constructor)]
    pub fn new(value: u32) -> Self {
        let mut instance = Self::default();
        
        instance.value = value;
        
        instance
    }
}
```

## Removed methods

- `ink::env::random`

Function `ink::env::random` was removed. Right now, there is no known possibility of providing random entropy on-chain.
You can either use a rand chain extension, or create pseudo-randomness with something like

```rust
let salt = (<Self as DefaultEnv>::env().block_timestamp(), some_value).encode();
let hash = xxh32(&salt, 0).to_le_bytes();
```

## Removed types

- `ink_primitives::KeyPtr`

You can't use `KeyPtr` now. You should use just `Key` instead.
Also, you can't make `Key::new` from bytes. You can use the new method `KeyComposer::from_bytes` instead.

## Other changes

- `AccountId` [does not implement](https://github.com/paritytech/ink/pull/1255) `Default` anymore.
- `fire` method in `CallBuilder` was renamed to `invoke`, and now it but it is better to use `try_invoke` instead, so you can handle errors.
- `instantiate` now doesn't require to do `unwrap`, as 

## Problems

 - `DelegateCall` is not supported yet because it [was marked](https://github.com/paritytech/ink/pull/1331#discussion_r953736863)
as a possible attack vector.