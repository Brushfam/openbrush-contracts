---
sidebar_position: 8
title: Timelock Controller
---

This example shows how you can reuse the implementation of
[timelock-controller](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/governance/timelock_controller).

## Step 1: Import default implementation

With [default `Cargo.toml`](overview.md/#the-default-toml-of-your-project-with-openbrush),
you need to enable corresponding features, embed modules data structures and implement them via `#[openbrush::implementation]` macro
as described in [that section](overview.md/#reuse-implementation-of-traits-from-openbrush).

The main traits are `AccessControl` and `TimelockController`.

## Step 2: Define constructor

Define constructor where you init admin of the contract.

```rust
impl Contract {
    #[ink(constructor)]
    pub fn new(min_delay: Timestamp, proposers: Vec<AccountId>, executors: Vec<AccountId>) -> Self {
        let mut instance = Self::default();

        let caller = Self::env().caller();
        // `TimelockController` and `AccessControl` have `_init_with_admin` methods.
        // You need to call it for each trait separately, to initialize everything for these traits.
        access_control::Internal::_init_with_admin(instance, caller);
        timelock_controller::Internal::_init_with_admin(instance, caller, min_delay, proposers, executors);
        
        instance
    }
}
```

## Final code

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(AccessControl, TimelockController)]
#[openbrush::contract]
pub mod my_timelock_controller {
    use ink::prelude::vec::Vec;
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        access_control: access_control::Data,
        #[storage_field]
        timelock: timelock_controller::Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(min_delay: Timestamp, proposers: Vec<AccountId>, executors: Vec<AccountId>) -> Self {
            let mut instance = Self::default();

            let caller = Self::env().caller();
            // `TimelockController` and `AccessControl` have `_init_with_admin` methods.
            // You need to call it for each trait separately, to initialize everything for these traits.
            access_control::Internal::_init_with_admin(&mut instance, Some(caller));
            timelock_controller::Internal::_init_with_admin(
                &mut instance,
                Some(caller),
                min_delay,
                proposers,
                executors,
            );

            instance
        }
    }
}

```

You can check an example of the usage of [TimelockController](https://github.com/Brushfam/openbrush-contracts/tree/main/examples/timelock_controller).