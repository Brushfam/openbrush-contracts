---
sidebar_position: 6
title: PSP22 Capped
---

This example shows how you can reuse the implementation of [PSP22](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/token/psp22) token with the [PSP22Capped](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/token/psp22/extensions/capped.rs) extension.

First, you should implement basic version of [PSP22](/smart-contracts/PSP22).

## Step 1: Add imports and enable unstable feature

- Use `openbrush::contract` macro instead of `ink::contract`.
- Use `openbrush::implementation` macro to inherit implementations of `PSP22` and `PSP22Capped` traits.

## Step 2: Define storage

Declare storage struct and declare the field related to the capped module data structure.
Then you need to derive the `Storage` trait and mark the corresponding field with
the `#[storage_field]` attribute. Deriving this trait allows you to reuse the
`PSP22Capped` extension in your `PSP22` implementation.

```rust
#[ink(storage)]
#[derive(Default, Storage)]
pub struct Contract {
    ...
    #[storage_field]
    cap: capped::Data,
}
```

## Step 3: Inherit logic

You can customize (override) methods using `#[overrider]` attribute.

Override `psp22::Transfer` to check is the cap exceeded before minting.

```rust 
#[overrider(psp22::Internal)]
fn _before_token_transfer(
    &mut self,
    from: Option<&AccountId>,
    _: Option<&AccountId>,
    amount: &Balance,
) -> Result<(), PSP22Error> {
    // `is_none` means that it is minting
    if from.is_none() && capped::Internal::_is_cap_exceeded(self, amount) {
        return Err(PSP22Error::Custom(String::from("Cap exceeded")))
    }
    Ok(())
}
```

## Step 4: Define constructor

Define constructor. Your `PSP22Capped` contract is ready!

```rust
impl Contract {
    /// Constructor which mints `initial_supply` of the token to sender
    /// Will set the token's cap to `cap`
    #[ink(constructor)]
    pub fn new(inital_supply: Balance, cap: Balance) -> Self {
        let mut instance = Self::default();

        assert!(capped::Internal::_init_cap(&mut instance, cap).is_ok());
        assert!(PSP22Mintable::mint(&mut instance, Self::env().caller(), inital_supply).is_ok());

        instance
    }
}
```

## Final code

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP22, PSP22Capped, PSP22Mintable)]
#[openbrush::contract]
pub mod my_psp22_capped {
    use openbrush::traits::{
        Storage,
        String,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        cap: capped::Data,
    }

    #[overrider(psp22::Internal)]
    fn _before_token_transfer(
        &mut self,
        from: Option<&AccountId>,
        _: Option<&AccountId>,
        amount: &Balance,
    ) -> Result<(), PSP22Error> {
        // `is_none` means that it is minting
        if from.is_none() && capped::Internal::_is_cap_exceeded(self, amount) {
            return Err(PSP22Error::Custom(String::from("Cap exceeded")))
        }
        Ok(())
    }

    impl Contract {
        /// Constructor which mints `initial_supply` of the token to sender
        /// Will set the token's cap to `cap`
        #[ink(constructor)]
        pub fn new(inital_supply: Balance, cap: Balance) -> Self {
            let mut instance = Self::default();

            assert!(capped::Internal::_init_cap(&mut instance, cap).is_ok());
            assert!(PSP22Mintable::mint(&mut instance, Self::env().caller(), inital_supply).is_ok());

            instance
        }
    }
}
```

You can check an implementation example of [PSP22 Capped](https://github.com/Brushfam/openbrush-contracts/tree/main/examples/psp22_extensions/capped).

You can also check the documentation for the basic implementation of [PSP22](/smart-contracts/PSP22).
