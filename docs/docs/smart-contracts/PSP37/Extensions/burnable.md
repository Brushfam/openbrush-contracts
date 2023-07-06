---
sidebar_position: 3
title: PSP37 Burnable
---

This example shows how you can reuse the implementation of [PSP37](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/token/psp37) token with [PSP37Burnable](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/token/psp37/extensions/burnable.rs) extension.

## How to use this extension

First, you should implement basic version of [PSP37](/smart-contracts/PSP37). 
After you can just add implementation of PSP37Burnable via `#[openbrush::implementation(PSP37Burnable)]` attribute.

## Final code

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP37, PSP37Burnable)]
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

        #[ink(message)]
        pub fn mint_to(&mut self, to: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP37Error> {
            psp37::Internal::_mint_to(self, to, ids_amounts)
        }
    }
}

```

And that's it! Your `PSP37` is now extended by the `PSP37Burnable` extension and ready to use its functions!
You can check an example of the usage of [PSP37 Burnable](https://github.com/Brushfam/openbrush-contracts/tree/main/examples/psp37_extensions/burnable).
 the documentation for the basic implementation of [PSP37](/smart-contracts/PSP37).