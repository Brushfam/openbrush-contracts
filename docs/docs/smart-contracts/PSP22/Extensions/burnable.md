---
sidebar_position: 3
title: PSP22 Burnable
---

This example shows how you can reuse the implementation of
[PSP22](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/token/psp22) token with [PSP22Burnable](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/token/psp22/extensions/burnable.rs) extension.

## How to use this extension

First, you should implement basic version of [PSP22](../psp22.md).

After you can just add implementation of PSP22Burnable via `#[openbrush::implementation(PSP22Burnable)]` attribute.

## Final code 

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP22, PSP22Burnable)]
#[openbrush::contract]
pub mod my_psp22_burnable {
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

        #[ink(message)]
        pub fn burn_from_many(&mut self, accounts: Vec<(AccountId, Balance)>) -> Result<(), PSP22Error> {
            for account in accounts.iter() {
                PSP22Burnable::burn(self, account.0, account.1)?;
            }
            Ok(())
        }
    }
}
```

And that's it! Your `PSP22` is now extended by the `PSP22Burnable` extension and ready to use its functions!
You can check an example of the usage of [PSP22 Burnable](https://github.com/Brushfam/openbrush-contracts/tree/main/examples/psp22_extensions/burnable).
