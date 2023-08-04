---
sidebar_position: 1
title: PSP22 Token Timelock
---

This example shows how you can reuse the implementation of [PSP22 Token Timelock](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/token/psp22/src/utils/token_timelock.rs) utility for [PSP22](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/token/psp22). This contract will lock user's `PSP22` tokens until the time specified, when they can withdraw them.

## Step 1: Implement features

- Use `openbrush::contract` macro instead of `ink::contract`.
- Use `openbrush::implementation` macro to inherit the implementation of the `PSP22TokenTimelock` trait.

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::contract]
#[openbrush::implementation(PSP22TokenTimelock)]
pub mod my_psp22_token_timelock {
...
```

## Step 2: Define storage

Declare storage struct and declare the field related to the timelock module data structure.
Then you need to derive the `Storage` trait and mark the corresponding field with
the `#[storage_field]` attribute. Deriving this trait allows you to reuse the
`PSP22TokenTimelock`.

```rust
#[ink(storage)]
#[derive(Default, Storage)]
pub struct Contract {
    #[storage_field]
    timelock: token_timelock::Data,
}
```

## Step 3: Inherit logic

You can customize (override) methods using `#[openbrush::overrider]` macro.

## Step 4: Define constructor

Define constructor. Your implementation of `PSP22TokenTimelock` contract is ready!

```rust
impl Contract {
    #[ink(constructor)]
    pub fn new(token_address: AccountId, beneficiary: AccountId, release_time: Timestamp) -> Self {
        let mut instance = Self::default();

        assert!(instance._init(token_address, beneficiary, release_time).is_ok());
        
        instance
    }
}
```

## Final code
```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP22TokenTimelock)]
#[openbrush::contract]
pub mod my_psp22_token_timelock {
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        timelock: token_timelock::Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(token_address: AccountId, beneficiary: AccountId, release_time: Timestamp) -> Self {
            let mut instance = Self::default();

            token_timelock::Internal::_init(&mut instance, token_address, beneficiary, release_time)
                .expect("Should init");

            instance
        }
    }
}

```

You can check an example of the usage of [PSP22 Token Timelock](https://github.com/Brushfam/openbrush-contracts/tree/main/examples/psp22_utils/token_timelock).

You can also check the documentation for the basic implementation of [PSP22](../psp22.md).