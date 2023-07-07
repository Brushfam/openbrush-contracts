---
sidebar_position: 1
title: Overview
---

This doc contains description of how the OpenBrush library can be imported and used.

The OpenBrush is using ink! version `4.2.1` at the moment.
You will need to use the same version of ink! in your project.
If you use a different version of ink, you need to use a different version of OpenBrush which uses the same version of ink!.
OpenBrush had several significant changes in API, so you check the [Wizard](https://openbrush.io)
to study how to use different versions of OpenBrush.

The documentation describes the latest available OpenBrush and how to use it.
It doesn't contain [versioning](https://github.com/supercolony-net/openbrush-contracts/issues/127) yet.

#### The default `toml` of your project with OpenBrush:

```toml
[dependencies]
# Import ink!
ink = { version = "4.2.1", default-features = false}

scale = { package = "parity-scale-codec", version = "3", default-features = false, features = ["derive"] }
scale-info = { version = "2.6", default-features = false, features = ["derive"], optional = true }

# OpenBrush dependency
openbrush = { git = "https://github.com/Brushfam/openbrush-contracts", branch = "feature/stable-rust", default-features = false }

[features]
default = ["std"]
std = [
  "ink/std",
  "scale/std",
  "scale-info/std",
  # OpenBrush dependency
  "openbrush/std",
]
ink-as-dependency = []
```

By default, the `openbrush` crate provides [macros](https://github.com/Brushfam/openbrush-contracts/blob/main/lang/macro/src/lib.rs)
for simplification of the development and [traits](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/traits) of
contracts (you can implement them by yourself, and you can use them for cross-contract calls).

OpenBrush also provides the default implementation of traits that can be enabled via crate features.
A list of all available features you can find [here](https://github.com/Brushfam/openbrush-contracts/blob/main/Cargo.toml#L51).
The default features are implemented by a `#[openbrush::implentation]` macro, by providing the trait name you want to implement, and functions from the default implementation can be overriden using the `#[overrider]` attribute. If you want to use the default implementation of a function, while adding some modifier to the function, you can do so with the `#[default_impl]` attribute. Both of these attribute take the name of the trait we are overriding the method in as argument. Some default implementations come with several traits containing methods that can be overriden. We can override any function in any trait with these attributes. An example PSP22 with some overriden functions would look like this: 

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

// This will add the default implementation of PSP22 and PSP22Mintable
#[openbrush::implementation(PSP22, PSP22Mintable, Ownable)] 
// This macro will collect the traits and override them. Make sure it comes after the implementation macro!
#[openbrush::contract] 
pub mod psp22_example {
  // derive macro which implements traits needed for a proper Storage manipulation within OB standards
  use openbrush::traits::Storage; 
  use ink::storage::traits::ManualKey;
  use ink::storage::traits::Lazy;
  #[ink(storage)] // needed for the ink! contract storage struct
  // this will implement traits needed for OB standards to work with the contract storage struct
  #[derive(Storage, Default)] 
  pub struct PSP22Example {
    // we have to add the data structs needed to work with the implemented traits to the storage
    // the fields need to be marked with this attribute in order for the contract to implement neede traits
    #[storage_field] 
    psp22: psp22::Data,
    #[storage_field]
    ownable: ownable::Data,
    // here we can add any other fields needed for our contract
    // we will add logic which bans a user from transferring the token
    // we will make it lazy and set it a manual storage key so we can upgrade this contract in future
    banned_account: Lazy<AccountId, ManualKey<123>>
  }

  #[default_impl(PSP22Mintable)] // we will add some attributes to the mint function in PSP22Mintable
  #[modifiers(ownable::only_owner)] // this will be moved to the PSP22Mintable::mint along with any other attributes
  fn mint() { 
    // the default_impl attribute only cares about the function name and the trait name
    // in which we want to override the method, therefore we can omit all parameters and return types. 
    // default_impl macro will use the original body of the function, so here we can keep it empty as well.
  }

  #[overrider(psp22::Internal)] // we want to override psp22::Internal::_before_token_transfer method
  fn _before_token_transfer(
    &mut self,
    from: Option<&AccountId>,
    _to: Option<&AccountId>,
    _amount: &Balance,
  ) -> Result<(), PSP22Error> {
    if from == self.banned_account.get() {
      return Err(PSP22Error::InsufficientAllowance)
    }
    Ok(())
  }

  impl Contract {
    #[ink(constructor)]
    pub fn new(total_supply: Balance) -> Self {
      let mut instance = Self::default();

      psp22::Internal::_mint_to(&mut instance, Self::env().caller(), total_supply).expect("Should mint");
      ownable::Internal::_init_with_owner(&mut instance, Self::env().caller());
      // private key of 0x0 is known, so we ban transfers from this account and users can safely use it as burn address!
      self.banned_account.set([0u8; 32]); 

      instance
    }
  }
}
```

:::note

ink! requires to put `#![cfg_attr(not(feature = "std"), no_std, no_main)]` at the top of root crate.

:::

:::note

The standards implemented in OpenBrush support events, but user has to specify this in their contract. ink! events have to be defined in the contract mod, meaning you have to emit them in the contract mod. You can do this by overriding the default _emit_xxx_event methods in different standards. There is currently a [PR in ink!](https://github.com/paritytech/ink/pull/1827) which will allow us to define events anywhere, and we will reflect this change in OB as well!

:::

#### Reuse implementation of traits from OpenBrush

The doc contains links to the examples of how to reuse and customize the default implementation of traits.

All default implementations of the traits provided by OpenBrush have the same pattern.
Consequently, the re-usage of each implementation in your contract also has the same pattern.

Each implementation of the contract has its module and its feature that enables that
module. A list of available modules you can find [here](https://github.com/Brushfam/openbrush-contracts/blob/main/contracts/src/lib.rs#L33),
a list of available features [here](https://github.com/Brushfam/openbrush-contracts/blob/main/Cargo.toml#L51).
Each module can be reached via the `openbrush::contracts::` namespace. For example,
to use the `psp22` module, you need to import `openbrush::contracts::psp22`;
to use the `ownable` module, you need to import `openbrush::contracts::ownable`. It is not needed to import the modules when using the `implementation` macro, the macro will do it for you.

Before importing each module (manually or with the macro), first you need to enable the corresponding feature in your `Cargo.toml`.
The name of the feature is the same as the name of the module. For example:

To enable `psp22`:

```toml
openbrush = { git = "https://github.com/Brushfam/openbrush-contracts", branch = "feature/stable-rust", default-features = false, features = ["psp22"] }
```

To enable `ownable`:

```toml
openbrush = { git = "https://github.com/Brushfam/openbrush-contracts", branch = "feature/stable-rust", default-features = false, features = ["ownable"] }
```

To enable both:

```toml
openbrush = { git = "https://github.com/Brushfam/openbrush-contracts", branch = "feature/stable-rust", default-features = false, features = ["psp22, ownable"] }
```

After enabling the feature and importing the corresponding module, you need to embed the module
data structure into your contract as a field and implement the `openbrush::traits::Storage`
trait for that field. In most cases, the data structure of each module is named `Data`.
If importing several modules, you can specify which data you want to use via namespaces like
`psp22::Data` or `ownable::Data`.

Embedding of data structures looks like:

```rust
use openbrush::contracts::ownable::*;
use openbrush::contracts::psp22::*;

#[ink(storage)]
pub struct Contract {
    foo: psp22::Data,
    bar: ownable::Data,
}
```

Each contract that wants to reuse implementation should implement the
`openbrush::traits::Storage` with the corresponding data structure.
The easiest way to implement that trait is via the derive macro by adding
`#[derive(Storage)]` and marking the corresponding fields with the `#[storage_field]`
attribute.

```rust
use openbrush::contracts::ownable::*;
use openbrush::contracts::psp22::*;
use openbrush::traits::Storage;

#[ink(storage)]
#[derive(Storage)]
pub struct Contract {
    #[storage_field]
    foo: psp22::Data,
    #[storage_field]
    bar: ownable::Data,
}
```

Now your contract has access to default implementation on the Rust level.
It is on the Rust level so you can call methods only inside your contract
(in the example, it is methods of `PSP22`, `psp22::Internal`, `Ownable`, and
`ownable::Internal` traits). You can implement the traits yourself if you need
a custom behavior of OpenBrush-defined standard. In most cases, you want to
inherit the behavior of OpenBrush standard and do some modifications in its behavior.
You can do it with the `#[openbrush::implementation]` macro. We can omit the imports
for ownable and psp22 as they will be imported with the macro.

```rust
#[openbrush::implementation(PSP22, Ownable)]
#[openbrush::contract]
pub mod my_psp22 {
  use openbrush::traits::Storage;

  #[ink(storage)]
  #[derive(Storage)]
  pub struct Contract {
      #[storage_field]
      foo: psp22::Data,
      #[storage_field]
      bar: ownable::Data,
  }
```

Remember, only traits with `#[ink(message)]` methods can be public. `psp22::Internal`
and `ownable::Internal` can't be exposed. It is for internal usage only.

The implementation in OpenBrush is called "default" because you can customize (override) it.
You can override any method from any trait with the `#[overrider]` macro, by passing the
name of trait the method belongs to. For example:

```rust
#[openbrush::implementation(PSP22, Ownable)]
#[openbrush::contract]
pub mod my_psp22 {
  use openbrush::traits::Storage;

  #[ink(storage)]
  #[derive(Storage)]
  pub struct Contract {
      #[storage_field]
      foo: psp22::Data,
      #[storage_field]
      bar: ownable::Data,
  }

  #[overrider(PSP22)]
  fn balance_of(&self, owner: AccountId) -> Balance {
    // For example you can break `balance_of` method and return always zero
    return 0
  }

  #[overrider(Ownable)]
  fn owner(&self) -> Option<AccountId> {
    // For example you can return always zero owner
    None
  }

  #[overrider(psp22::Internal)]
  fn _mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
    return Err(PSP22Error::Custom("I don't want to mint anything".to_string()));
  }

  #[overrider(ownable::Internal)]
  fn _init_with_owner(&mut self, owner: AccountId) {
    // Maybe you want to change something during initialization of the owner
  }
```

Work with each module has the same pattern. The difference is only in the naming of
the module and main trait. Some contract extensions require additional steps, so below,
you can find instructions on how to work with them:

- [PSP22](PSP22/psp22.md) is an example of how you can reuse the implementation of
  [psp22](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/token/psp22). You also can find examples of how to reuse extensions.
  - [PSP22Metadata](PSP22/Extensions/metadata.md): metadata for PSP22.
  - [PSP22Mintable](PSP22/Extensions/mintable.md): creation of new tokens.
  - [PSP22Burnable](PSP22/Extensions/burnable.md): destruction of own tokens.
  - [PSP22Wrapper](PSP22/Extensions/wrapper.md): wrapper for PSP22 token (useful for governance tokens etc.).
  - [PSP22FlashMint](PSP22/Extensions/flashmint.md): extension which allows performing flashloans of the token by minting and burning the token.
  - [PSP22Pausable](PSP22/Extensions/pausable.md): example of using pausable extension in the PSP22 contract.
  - [PSP22TokenTimelock](PSP22/Utils/token-timelock.md): Utility which allows token holders to lock their tokens for a specified amount of time.
- [PSP34](PSP34/psp34.md) is an example of how you can reuse the implementation of
  [psp34](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/token/psp34). You also can find examples of how to reuse extensions.
  - [PSP34Metadata](PSP34/Extensions/metadata.md): metadata for PSP34.
  - [PSP34Mintable](PSP34/Extensions/mintable.md): creation of new tokens.
  - [PSP34Burnable](PSP34/Extensions/burnable.md): destruction of own tokens.
  - [PSP34Enumerable](PSP34/Extensions/enumerable.md): iterating over contract's tokens.
- [PSP37](PSP37/psp37.md) is an example of how you can reuse the implementation of
  [psp37](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/token/psp37). You also can find examples of how to reuse extensions.
  - [PSP37Metadata](PSP37/Extensions/metadata.md): metadata for PSP37.
  - [PSP37Mintable](PSP37/Extensions/mintable.md): creation of new tokens.
  - [PSP37Burnable](PSP37/Extensions/burnable.md): destruction of own tokens.
  - [PSP37Batch](PSP37/Extensions/batch.md): batch transferring of tokens.
  - [PSP37Enumerable](PSP37/Extensions/enumerable.md): iterating over contract's tokens.
- [Access Control](access-control/access-control.md) shows how you can use the implementation of
  [access-control](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/access/access_control) and
  [psp34](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/token/psp34) together to provide rights to mint and burn NFT tokens.
  - [AccessControlEnumerable](access-control/Extensions/enumerable.md): iterating over contract's roles.
- [Ownable](ownable.md) shows how you can use the implementation of
  [ownable](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/access/ownable) and
  [psp37](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/token/psp37) together to provide rights to mint and burn tokens.
- [ReentrancyGuard](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/security/reentrancy_guard)
  modifier to prevent reentrancy during certain functions.
- [Pausable](pausable.md) shows how you can use the implementation of
  [pausable](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/security/pausable)
  contract and modifiers.
- [TimelockController](timelock-controller.md) shows how you can use the implementation of
  [timelock-controller](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/governance/timelock_controller)
  to execute a transaction with some delay via governance.
- [PaymentSplitter](payment-splitter.md) shows how you can use the implementation of
  [payment-splitter](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/finance/payment_splitter)
  to split received native tokens between participants of the contract.
