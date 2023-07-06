---
sidebar_position: 2
title: PSP22 Mintable
---

This example shows how you can reuse the implementation of
[PSP22](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/token/psp22) token with [PSP22Mintable](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/token/psp22/extensions/mintable.rs) extension.

## How to use this extension

First, you should implement basic version of [PSP22](/smart-contracts/PSP22).

After you can just add implementation of PSP22Mintable via `#[openbrush::implementation(PSP22Mintable)]` attribute.

## Final implementation

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP22, PSP22Mintable)]
#[openbrush::contract]
pub mod my_psp22_mintable {
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp22: psp22::Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self::default();

            psp22::Internal::_mint_to(&mut instance, Self::env().caller(), total_supply).expect("Should mint");

            instance
        }
    }
}

```

You can check an example of the usage of [PSP22 Mintable](https://github.com/Brushfam/openbrush-contracts/tree/main/examples/psp22_extensions/mintable).

And that's it! Your `PSP22` is now extended by the `PSP22Mintable` extension and ready to use its functions!
