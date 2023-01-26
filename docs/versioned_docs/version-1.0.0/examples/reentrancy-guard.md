## Overview

This example shows how you can use the [non_reentrant](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/security/reentrancy-guard)
modifier to prevent reentrancy into certain functions. In this example we will create two contracts:

- `my_flipper_guard` - this contract is the simple version of [flipper](https://github.com/paritytech/ink/tree/master/examples/flipper),
  but method `flip` will be marked with `non_reentrant` modifier, and we will add additional method, also marked
  with `non_reentrant`, which will ask another contract to call `flip` of our `flipper`.
- `flip_on_me` is a contract which has the only one method `flip_on_me`. This method will try to call `flip` on the caller
  (it means that caller must be a contract with method `flip`).

## MyFlipper

### Steps

1. Include dependencies to `reentrancy-guard` and `brush` in the cargo file.

```markdown
[dependencies]
ink_primitives = { tag = "v3.0.0-rc4", git = "https://github.com/Supercolony-net/ink", default-features = false }
ink_metadata = { tag = "v3.0.0-rc4", git = "https://github.com/Supercolony-net/ink", default-features = false, features = ["derive"], optional = true }
ink_env = { tag = "v3.0.0-rc4", git = "https://github.com/Supercolony-net/ink", default-features = false }
ink_storage = { tag = "v3.0.0-rc4", git = "https://github.com/Supercolony-net/ink", default-features = false }
ink_lang = { tag = "v3.0.0-rc4", git = "https://github.com/Supercolony-net/ink", default-features = false }
ink_prelude = { tag = "v3.0.0-rc4", git = "https://github.com/Supercolony-net/ink", default-features = false }

scale = { package = "parity-scale-codec", version = "2.1", default-features = false, features = ["derive"] }
scale-info = { version = "0.6.0", default-features = false, features = ["derive"], optional = true }

# These dependencies
reentrancy-guard = { tag = "v1.0.0", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }
brush = { tag = "v1.0.0", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }

crate-type = [
    "cdylib",
    # This contract will be imported by FlipOnMe contract, so we need build this crate also like a `rlib`
    "rlib",
]

[features]
default = ["std"]
std = [
    "ink_primitives/std",
    "ink_metadata",
    "ink_metadata/std",
    "ink_env/std",
    "ink_storage/std",
    "ink_lang/std",
    "scale/std",
    "scale-info",
    "scale-info/std",

    # These dependencies
    "brush/std",
    "reentrancy_guard/std",
]

ink-as-dependency = []
```

2. To declare the contract, you need to use `brush::contract` macro instead of `ink::contract`. Import **everything**
   from `reentrancy-guard` trait module.

```rust
#[brush::contract]
pub mod my_flipper_guard {
    use reentrancy_guard::traits::*;
    use brush::modifiers;
    use ink_env::call::FromAccountId;
    use crate::flip_on_me::CallerOfFlip;
```

3. Declare storage struct and declare the field for `ReentrancyGuardStorage` trait. Then you need to
   derive `ReentrancyGuardStorage` trait and mark the field with `#[ReentrancyGuardStorageField]` attribute. Deriving
   this trait allows you to use `non_reentrant` modifier.

```rust
#[ink(storage)]
#[derive(Default, ReentrancyGuardStorage)]
pub struct MyFlipper {
    #[ReentrancyGuardStorageField]
    guard: ReentrancyGuardData,
    value: bool,
}
```

4. After that you can add `non_reentrant` modifier to `flip` and `call_flip_on_me` methods.

```rust
impl MyFlipper {
    #[ink(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    #[ink(message)]
    pub fn get_value(&self) -> bool {
        self.value
    }

    #[ink(message)]
    #[brush::modifiers(non_reentrant)]
    pub fn flip(&mut self) {
        self.value = !self.value;
    }

    #[ink(message)]
    #[modifiers(non_reentrant)]
    pub fn call_flip_on_me(&mut self, callee: AccountId) {
        // This method will do a cross-contract call to callee account. It calls method `flip_on_me`.
        // Callee contract during execution of `flip_on_me` will call `flip` of this contract.
        // `call_flip_on_me` and `flip` are marked with `non_reentrant` modifier. It means,
        // that call of `flip` after `call_flip_on_me` must fail.
        let mut flipper: CallerOfFlip = FromAccountId::from_account_id(callee);
        flipper.flip_on_me();
    }
}
```

5. To simplify cross contract call to `FlipOnMe` contract let's create a wrapper around the contract's account id.
   For that, we will define another contract in this crate with `#[ink_lang::contract(compile_as_dependency = true)]`
   and empty methods but with the same signature as in the original contract.

```rust
/// This is a stub implementation of contract with method `flip_on_me`.
/// We need this implementation to create a wrapper around account id of contract.
/// With this wrapper, we can easily call methods of some contract.
/// Example:
/// ```
/// let mut flipper: CallerOfFlip = FromAccountId::from_account_id(callee);
/// flipper.flip_on_me();
/// ```
#[ink_lang::contract(compile_as_dependency = true)]
pub mod flip_on_me {
    #[ink(storage)]
    pub struct CallerOfFlip {}

    impl CallerOfFlip {
        #[ink(constructor)]
        pub fn new() -> Self {
            unimplemented!()
        }
    }

    impl CallerOfFlip {
        #[ink(message)]
        pub fn flip_on_me(&mut self) {
            unimplemented!()
        }
    }
}
```

## FlipOnMe

It's a simple contract that doesn't use any logic from the OpenBrush, so you can use simple ink! here.

### Steps

1. Define `FlipOnMe` contract. It has the only method `flip_on_me`, which will call `flip` on caller.

```rust
#[ink_lang::contract]
pub mod flip_on_me {
    use ink_env::call::FromAccountId;
    use my_flipper_guard::my_flipper_guard::MyFlipper;

    #[ink(storage)]
    #[derive(Default)]
    pub struct FlipOnMe {}

    impl FlipOnMe {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn flip_on_me(&mut self) {
            let caller = self.env().caller();
            // This method does a cross-contract call to caller contract and calls the `flip` method.
            let mut flipper: MyFlipper = FromAccountId::from_account_id(caller);
            flipper.flip();
        }
    }
}
```

2. To simplify cross-contract call to `MyFlipper` you need to import the contract with `ink-as-dependency` feature.

```rust
[dependencies]
ink_primitives = { tag = "v3.0.0-rc4", git = "https://github.com/Supercolony-net/ink", default-features = false }
ink_metadata = { tag = "v3.0.0-rc4", git = "https://github.com/Supercolony-net/ink", default-features = false, features = ["derive"], optional = true }
ink_env = { tag = "v3.0.0-rc4", git = "https://github.com/Supercolony-net/ink", default-features = false }
ink_storage = { tag = "v3.0.0-rc4", git = "https://github.com/Supercolony-net/ink", default-features = false }
ink_lang = { tag = "v3.0.0-rc4", git = "https://github.com/Supercolony-net/ink", default-features = false }
ink_prelude = { tag = "v3.0.0-rc4", git = "https://github.com/Supercolony-net/ink", default-features = false }

scale = { package = "parity-scale-codec", version = "2.1", default-features = false, features = ["derive"] }
scale-info = { version = "0.6.0", default-features = false, features = ["derive"], optional = true }

# This dependencies
my_flipper_guard = { path = "../flipper", default - features = false, features = ["ink-as-dependency"] }

[features]
default = ["std"]
std = [
    "ink_primitives/std",
    "ink_metadata",
    "ink_metadata/std",
    "ink_env/std",
    "ink_storage/std",
    "ink_lang/std",
    "scale/std",
    "scale-info",
    "scale-info/std",
    
    # This dependencies
    "my_flipper_guard/std",
]
```