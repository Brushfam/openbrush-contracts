---
sidebar_position: 1
title: PSP34 Metadata
---

This example shows how you can reuse the implementation of [PSP34](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/token/psp34) token with [PSP34Metadata](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/token/psp34/extensions/metadata.rs) extension.

First, you should implement basic version of [PSP34](/smart-contracts/PSP34).

## Step 1: Implement features

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP34, PSP34Metadata)]
#[openbrush::contract]
pub mod my_psp34_metadata {
...
```

## Step 2: Define storage

Declare storage struct and declare the field related to the metadata module data structure. 
Then you need to derive the `Storage` trait and mark the corresponding field with 
the `#[storage_field]` attribute. Deriving this trait allows you to reuse the 
`PSP34Metadata` extension in your `PSP34` implementation.

```rust
#[derive(Default, Storage)]
#[ink(storage)]
pub struct Contract {
    #[storage_field]
    psp34: psp34::Data,
    #[storage_field]
    metadata: metadata::Data,
}
```

## Step 3: Define constructor

Define constructor. Your `PSP34Metadata` contract is ready!

```rust
impl Contract {
    /// A constructor which mints the first token to the owner
    #[ink(constructor)]
    pub fn new(id: Id, name: String, symbol: String) -> Self {
        let mut instance = Self::default();

        let name_key = String::from("name");
        let symbol_key = String::from("symbol");
        metadata::Internal::_set_attribute(&mut instance, id.clone(), name_key, name);
        metadata::Internal::_set_attribute(&mut instance, id, symbol_key, symbol);

        instance
    }
}
```

## Final code

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP34, PSP34Metadata)]
#[openbrush::contract]
pub mod my_psp34_metadata {
    use openbrush::traits::Storage;

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    impl Contract {
        /// A constructor which mints the first token to the owner
        #[ink(constructor)]
        pub fn new(id: Id, name: String, symbol: String) -> Self {
            let mut instance = Self::default();

            let name_key = String::from("name");
            let symbol_key = String::from("symbol");
            metadata::Internal::_set_attribute(&mut instance, id.clone(), name_key, name);
            metadata::Internal::_set_attribute(&mut instance, id, symbol_key, symbol);

            instance
        }
    }
}
```

You can check an example of the usage of [PSP34 Metadata](https://github.com/Brushfam/openbrush-contracts/tree/main/examples/psp34_extensions/metadata).

You can also check the documentation for the basic implementation of [PSP34](/smart-contracts/PSP34).
