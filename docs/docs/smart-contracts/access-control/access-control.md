---
sidebar_position: 1
title: Access Control
---

This example shows how you can use the implementation of [access-control](https://github.com/727-Ventures/openbrush-contracts/tree/main/contracts/src/access/access_control) to provide rights for usage of specific smart contract functions.

## Step 1: Import default implementation

With [default `Cargo.toml`](/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush),
you need to enable `access-control` feature, embed modules data structures and implement them via `#[openbrush::implementation]` macro
as described in [that section](/smart-contracts/overview#reuse-implementation-of-traits-from-openbrush).

The main trait is `AccessControl`.

## Step 2: Define constructor

Define constructor where you grant `MINTER` role(or any another role) to the caller.

```rust
// You can manually set the number for the role.
// But better to use a hash of the variable name.
// It will generate a unique identifier of this role.
// And will reduce the chance to have overlapping roles.
const MINTER: RoleType = ink::selector_id!("MINTER");

impl Contract {
    #[ink(constructor)]
    pub fn new() -> Self {
        let mut instance = Self::default();

        let caller = instance.env().caller();
        access_control::Internal::_init_with_admin(&mut instance, Some(caller));
        // We grant minter role to caller in constructor, so he can mint/burn tokens
        AccessControl::grant_role(&mut instance, MINTER, Some(caller)).expect("Should grant MINTER role");

        instance
    }
}
```

## Step 3: Customize your contract

Customize it by adding access control logic. We will add a `restricted_function` to `Contract` implementation, 
which will use the `only_role` modifier with `MINTER` parameter, which verifies that the caller has the `MINTER` role. 

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP34, PSP34Burnable, PSP34Mintable, AccessControl)]
#[openbrush::contract]
pub mod my_access_control {
    use openbrush::{
        modifiers,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        access: access_control::Data,
    }

    // You can manually set the number for the role.
    // But better to use a hash of the variable name.
    // It will generate a unique identifier of this role.
    // And will reduce the chance to have overlapping roles.
    const MINTER: RoleType = ink::selector_id!("MINTER");

    #[default_impl(PSP34Burnable)]
    #[modifiers(only_role(MINTER))]
    fn burn() {}

    #[default_impl(PSP34Mintable)]
    #[modifiers(only_role(MINTER))]
    fn mint() {}

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();

            let caller = instance.env().caller();
            access_control::Internal::_init_with_admin(&mut instance, Some(caller));
            // We grant minter role to caller in constructor, so he can mint/burn tokens
            AccessControl::grant_role(&mut instance, MINTER, Some(caller)).expect("Should grant MINTER role");

            instance
        }
    }
}

```

You can check an example of the usage of [Access Control](https://github.com/727-Ventures/openbrush-contracts/tree/main/examples/access_control).