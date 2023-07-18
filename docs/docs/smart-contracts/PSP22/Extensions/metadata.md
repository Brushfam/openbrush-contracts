---
sidebar_position: 1
title: PSP22 Metadata
---

This example shows how you can reuse the implementation of [PSP22](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/token/psp22) token with the [PSP22Metadata](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/token/psp22/extensions/metadata.rs) extension.

First, you should implement basic version of [PSP22](../psp22.md).

## Step 1: Add imports and enable unstable feature

Use `openbrush::contract` macro instead of `ink::contract`. Import **everything** from `openbrush::contracts::psp22::extensions::metadata`.

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP22, PSP22Metadata)]
#[openbrush::contract]
pub mod my_psp22 {
    ...
```

## Step 2: Define storage

Declare storage struct and declare the field related to the metadata module data structure.
Then you need to derive the `Storage` trait and mark the corresponding field with
the `#[storage_field]` attribute. Deriving this trait allows you to reuse the
`PSP22Metadata` extension in your `PSP22` implementation.

```rust
#[ink(storage)]
#[derive(Default, Storage)]
pub struct Contract {
    #[storage_field]
    psp22: psp22::Data,
    #[storage_field]
    metadata: metadata::Data,
}
```

## Step 3: Define constructor

Define constructor. Your `PSP22Metadata` contract is ready!

```rust
impl Contract {
    #[ink(constructor)]
    pub fn new(total_supply: Balance, name: Option<String>, symbol: Option<String>, decimal: u8) -> Self {
        let mut instance = Self::default();
        let caller = instance.env().caller();

        instance.metadata.name.set(&name);
        instance.metadata.symbol.set(&symbol);
        instance.metadata.decimals.set(&decimal);

        psp22::Internal::_mint_to(&mut instance, caller, total_supply).expect("Should mint total_supply");

        instance
    }
}
```

## Final code

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP22, PSP22Metadata)]
#[openbrush::contract]
pub mod my_psp22 {
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(total_supply: Balance, name: Option<String>, symbol: Option<String>, decimal: u8) -> Self {
            let mut instance = Self::default();
            let caller = instance.env().caller();

            instance.metadata.name.set(&name);
            instance.metadata.symbol.set(&symbol);
            instance.metadata.decimals.set(&decimal);

            psp22::Internal::_mint_to(&mut instance, caller, total_supply).expect("Should mint total_supply");

            instance
        }
    }
}
```

You can check an example of the usage of [PSP22 Metadata](https://github.com/Brushfam/openbrush-contracts/tree/main/examples/psp22_extensions/metadata).
