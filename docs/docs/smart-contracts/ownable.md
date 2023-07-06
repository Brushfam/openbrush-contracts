---
sidebar_position: 3
title: Ownable
---

This example shows how you can use the implementation of [ownable](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/access/ownable) to provide `only owner` rights for contract's functions.

## Step 1: Import default implementation

With [default `Cargo.toml`](/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush),
you need to enable `ownable` feature, embed modules data structures and implement them via `#[openbrush::implementation]` macro
as described in [that section](/smart-contracts/overview#reuse-implementation-of-traits-from-openbrush).

The main trait is `Ownable`.

## Step 2: Define constructor

Define the constructor and initialize the owner with the contract initiator.

```rust
impl Contract {
    #[ink(constructor)]
    pub fn new() -> Self {
        let mut instance = Self::default();
        ownable::Internal::_init_with_owner(&mut instance, Self::env().caller());
        instance
    }
}
```

## Step 3: Customize your contract

Customize it by adding ownable logic. We will add a `owner_function` to `MyOwnable` implementation 
and add the `only_owner` modifier, which will verify that the caller of the function is the owner.

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(Ownable, PSP37, PSP37Burnable, PSP37Mintable)]
#[openbrush::contract]
pub mod ownable {
    use openbrush::{
        modifiers,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp37: psp37::Data,
        #[storage_field]
        ownable: ownable::Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            ownable::Internal::_init_with_owner(&mut instance, Self::env().caller());
            instance
        }
    }

    #[default_impl(PSP37Mintable)]
    #[modifiers(only_owner)]
    fn mint(&mut self) {}

    #[default_impl(PSP37Burnable)]
    #[modifiers(only_owner)]
    fn burn(&mut self) {}
}

```

You can check an example of the usage of [Ownable](https://github.com/Brushfam/openbrush-contracts/tree/main/examples/ownable).
