---
sidebar_position: 1
title: AccessControl Enumerable
---

This example shows how you can reuse the implementation of [AccessControl](https://github.com/Brushfam/openbrush-contracts/blob/main/contracts/src/access/access_control/access_control.rs) with [AccessControlEnumerable](https://github.com/Brushfam/openbrush-contracts/blob/main/contracts/src/access/access_control/extensions/enumerable.rs) extension, which enables an easier overview of access control roles.

First, you should implement basic version of [AccessControl](../access-control.md).

## Step 1: Add implemenation of AccessControlEnumerable

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::contract]
#[openbrush::implementation(AccessControl, AccessControlEnumerable)]
pub mod my_access_control {
    ...
```

## Step 2: Define storage

```rust
#[ink(storage)]
#[derive(Default, Storage)]
pub struct Contract {
    #[storage_field]
    enumerable: enumerable::Data,
}
```

## Final code

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(AccessControl, AccessControlEnumerable)]
#[openbrush::contract]
pub mod my_access_control {
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        enumerable: enumerable::Data,
    }

    // You can manually set the number for the role.
    // But better to use a hash of the variable name.
    // It will generate a unique identifier of this role.
    // And will reduce the chance to have overlapping roles.
    const MINTER: RoleType = ink::selector_id!("MINTER");

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();

            let caller = Self::env().caller();
            access_control::Internal::_init_with_admin(&mut instance, Some(caller));
            // We grant minter role to caller in constructor, so he can mint/burn tokens
            AccessControl::grant_role(&mut instance, MINTER, Some(caller)).expect("Should grant MINTER role");
            assert_eq!(AccessControlEnumerable::get_role_member_count(&instance, MINTER), 1);

            instance
        }
    }
}
```

And that's it! Your `AccessControl` is now extended by the `AccessControlEnumerable` extension and ready to use its functions!
You can check an example of the usage of [AccessControl Enumerable](https://github.com/Brushfam/openbrush-contracts/blob/main/contracts/src/access/access_control/extensions/enumerable.rs).

You can also check the documentation for the basic implementation of [AccessControl](../access-control.md).