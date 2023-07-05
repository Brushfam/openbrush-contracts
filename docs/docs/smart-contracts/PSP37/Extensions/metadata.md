---
sidebar_position: 1
title: PSP37 Metadata
---

This example shows how you can reuse the implementation of [PSP37](https://github.com/727-Ventures/openbrush-contracts/tree/main/contracts/token/psp37) token with [PSP37Metadata](https://github.com/727-Ventures/openbrush-contracts/tree/main/contracts/token/psp37/extensions/metadata.rs) extension.

First, you should implement basic version of [PSP37](/smart-contracts/PSP37).

## Step 1: Implement PSP37Metadata

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(..., PSP37, PSP37Metadata, ...)]
#[openbrush::contract]
pub mod my_psp37 {
...
```

## Step 2: Define storage

Declare storage struct and declare the field related to the metadata module. 
Then you need to derive the `Storage` trait and mark the corresponding field with 
the `#[storage_field]` attribute. 
Deriving this trait allows you to reuse the `PSP37Metadata` extension in your 
`PSP37` implementation.

```rust
#[derive(Default, Storage)]
#[ink(storage)]
pub struct Contract {
    ...
    #[storage_field]
    metadata: metadata::Data,
}
```

## Step 3: Define constructor

Define constructor. Your `PSP37Metadata` contract is ready!

```rust
impl Contract {
    #[ink(constructor)]
    pub fn new() -> Self {
        Self::default()
    }
}
```

## Final code

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP37, PSP37Metadata)]
#[openbrush::contract]
pub mod my_psp37 {
    use openbrush::traits::{
        Storage,
        String,
    };

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp37: psp37::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    impl Contract {
        /// contract constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn set_attribute(&mut self, id: Id, key: String, data: String) -> Result<(), PSP37Error> {
            metadata::Internal::_set_attribute(self, &id, &key, &data)
        }
    }
}

```

You can check an example of the usage of [PSP37 Metadata](https://github.com/727-Ventures/openbrush-contracts/tree/main/examples/psp37_extensions/metadata).

You can also check the documentation for the basic implementation of [PSP37](/smart-contracts/PSP37).
