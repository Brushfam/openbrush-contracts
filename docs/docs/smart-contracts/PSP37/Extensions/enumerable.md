---
sidebar_position: 1
title: PSP37 Enumerable
---

This example shows how you can reuse the implementation of [PSP37](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/token/psp37) token with [PSP37Enumerable](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/token/psp37/extensions/enumerable.rs) extension.

First, you should implement basic version of [PSP37](../psp37.md).

## Step 1: Implement PSP37Enumerable

Import **everything** from `openbrush::contracts::psp37::extensions::enumerable`.

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::contract]
#[openbrush::implementation(..., PSP37, PSP37Enumerable, ...)]
pub mod my_psp37 {
...
```

## Step 2: Define storage

```rust
#[derive(Default, Storage)]
#[ink(storage)]
pub struct Contract {
    #[storage_field]
    psp37: psp37::Data,
    #[storage_field]
    enumerable: enumerable::Data,
}
```

## Final code

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP37, PSP37Batch, PSP37Burnable, PSP37Mintable, PSP37Enumerable)]
#[openbrush::contract]
pub mod my_psp37_enumerable {
    use openbrush::traits::Storage;

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp37: psp37::Data,
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

And that's it! Your `PSP37` is now extended by the `PSP37Enumerable` extension and ready to use its functions!
You can check an example of the usage of [PSP37 Enumerable](https://github.com/Brushfam/openbrush-contracts/tree/main/examples/psp37_extensions/enumerable).
