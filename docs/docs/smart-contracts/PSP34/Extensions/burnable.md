---
sidebar_position: 3
title: PSP34 Burnable
---

This example shows how you can reuse the implementation of [PSP34](https://github.com/727-Ventures/openbrush-contracts/tree/main/contracts/src/token/psp34) token with [PSP34Burnable](https://github.com/727-Ventures/openbrush-contracts/tree/main/contracts/src/token/psp34/extensions/burnable.rs) extension.

## How to use this extension

First, you should implement basic version of [PSP34](/smart-contracts/PSP34).

After you can just add implementation of PSP34Burnable via `#[openbrush::implementation(PSP34Burnable)]` attribute.

## Final code

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP34, PSP34Burnable)]
#[openbrush::contract]
pub mod my_psp34_burnable {
    use openbrush::traits::Storage;

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp34: psp34::Data,
    }

    impl Contract {
        /// The constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();

            psp34::Internal::_mint_to(&mut instance, Self::env().caller(), Id::U8(0u8))
                .expect("Should mint token with id 0");
            psp34::Internal::_mint_to(&mut instance, Self::env().caller(), Id::U8(1u8))
                .expect("Should mint token with id 1");
            psp34::Internal::_mint_to(&mut instance, Self::env().caller(), Id::U8(2u8))
                .expect("Should mint token with id 2");

            instance
        }
    }
}

```

And that's it! Your `PSP34` is now extended by the `PSP34Burnable` extension and ready to use its functions!
You can check an example of the usage of [PSP34 Burnable](https://github.com/727-Ventures/openbrush-contracts/tree/main/examples/psp34_extensions/burnable).