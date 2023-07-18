---
sidebar_position: 3
title: PSP34 Enumerable
---

This example shows how you can reuse the implementation of [PSP34](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/token/psp34) token with [PSP34Enumerable](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/token/psp34/extensions/enumerable.rs) extension.

First, you should implement basic version of [PSP34](../psp34.md).

## Step 1: Implement features

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP34, PSP34Mintable, PSP34Burnable, PSP34Enumerable)]
#[openbrush::contract]
pub mod my_psp34 {
...
```

## Step 2: Define storage

Pass `enumerable::Balances` into `psp34::Data` to be able to use `PSP34Enumerable` extension 
in your `PSP34` implementation.

```rust
#[derive(Default, Storage)]
#[ink(storage)]
pub struct Contract {
    #[storage_field]
    psp34: psp34::Data,
    #[storage_field]
    enumerable: enumerable::Data,
}
```

## Final code

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP34, PSP34Mintable, PSP34Burnable, PSP34Enumerable)]
#[openbrush::contract]
pub mod my_psp34_enumerable {
    use openbrush::traits::Storage;

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        enumerable: enumerable::Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }
}
```

And that's it! Your `PSP34` is now extended by the `PSP34Enumerable` extension and ready to use its functions!
You can check an example of the usage of [PSP34 Enumerable](https://github.com/Brushfam/openbrush-contracts/tree/main/examples/psp34_extensions/enumerable).
