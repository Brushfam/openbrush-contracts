---
sidebar_position: 4
title: PSP22 Wrapper
---

This example shows how you can reuse the implementation of [PSP22](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/token/psp22) token with [PSP22 Wrapper](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/token/psp22/extensions/wrapper.rs) extension, which allows you to wrap your `PSP22` token in a `PSP22Wrapper` token which can be used for example for governance.

First, you should implement basic version of [PSP22](/smart-contracts/PSP22).

## Step 1: Add imports and enable unstable feature

- Use `openbrush::contract` macro instead of `ink::contract`. 
- Use `openbrush::implementation` macro to inherit implementations of `PSP22` and `PSP22Wrapper` traits.

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP22, PSP22Wrapper)]
#[openbrush::contract]
pub mod my_psp22_wrapper {
    ...
}
```

## Step 2: Define storage

Declare storage struct and declare the field related to the wrapper module data structure.
Then you need to derive the `Storage` trait and mark the corresponding field with
the `#[storage_field]` attribute. Deriving this trait allows you to reuse the
`PSP22Wrapper` extension in your `PSP22` implementation.

```rust
#[ink(storage)]
#[derive(Default, Storage)]
pub struct Contract {
    #[storage_field]
    psp22: psp22::Data,
    #[storage_field]
    wrapper: wrapper::Data,
}
```

## Step 3: Define constructor

Define constructor where you init address of wrapper fungible token(PSP22) and `recover` message.

```rust
impl Contract {
    #[ink(constructor)]
    pub fn new(token_address: AccountId) -> Self {
        let mut instance = Self::default();

        Internal::_init(&mut instance, token_address);

        instance
    }

    /// Exposes the `_recover` function for message caller
    #[ink(message)]
    pub fn recover(&mut self) -> Result<Balance, PSP22Error> {
        Internal::_recover(self, Self::env().caller())
    }
}
```

## Final code

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP22, PSP22Wrapper)]
#[openbrush::contract]
pub mod my_psp22_wrapper {
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        wrapper: wrapper::Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(token_address: AccountId) -> Self {
            let mut instance = Self::default();

            Internal::_init(&mut instance, token_address);

            instance
        }

        /// Exposes the `_recover` function for message caller
        #[ink(message)]
        pub fn recover(&mut self) -> Result<Balance, PSP22Error> {
            Internal::_recover(self, Self::env().caller())
        }
    }
}

```

You can check an example of the usage of [PSP22 Wrapper](https://github.com/Brushfam/openbrush-contracts/tree/main/examples/psp22_extensions/wrapper).

You can also check the documentation for the basic implementation of [PSP22](/smart-contracts/PSP22).