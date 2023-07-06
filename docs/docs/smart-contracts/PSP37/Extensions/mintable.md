---
sidebar_position: 2
title: PSP37 Mintable
---

This example shows how you can reuse the implementation of [PSP37](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/token/psp37) token with [PSP37Mintable](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/token/psp37/extensions/mintable.rs) extension.

## How to use this extension

First, you should implement basic version of [PSP37](/smart-contracts/PSP37).

For your smart contract to use this extension, you only need to implement the 
`PSP37Mintable` via `#[openbrush::implementation(PSP37Mintable)]` attribute.

## Final code

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP37, PSP37Mintable)]
#[openbrush::contract]
pub mod my_psp37 {
    use openbrush::traits::Storage;

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp37: psp37::Data,
    }

    impl Contract {
        /// contract constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }
}
```

And that's it! Your `PSP37` is now extended by the `PSP37Mintable` extension and ready to use its functions!
You can check an example of the usage of [PSP37 Mintable](https://github.com/Brushfam/openbrush-contracts/tree/main/examples/psp37_extensions/mintable).
