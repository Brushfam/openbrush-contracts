---
sidebar_position: 5
title: PSP22 FlashMint
---

This example shows how you can reuse the implementation of [PSP22](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/token/psp22) token with [PSP22FlashMint](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/token/psp22/extensions/flashmint.rs) extension, which allows the user to perform a flash loan on the token by minting the borrowed amount and then burning it along with fees for the loan.

## 1. Implement the FlashMint extension

First, you should implement basic version of [PSP22](../psp22.md).

After you can just add implementation of PSP22Flashmint via `#[openbrush::implementation(PSP22Flashmint)]` attribute.

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP22, Flashmint)]
#[openbrush::contract]
pub mod my_psp22_flashmint {
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp22: psp22::Data,
    }

    /// Override `get_fee` function to add 1% fee to the borrowed `amount`
    #[overrider(flashmint::Internal)]
    fn _get_fee(&self, amount: Balance) -> Balance {
        amount / 100
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

And that's it! Your `PSP22` is now extended by the `PSP22FlashMint` extension and ready to use its functions!
You can check the full example of the implementation of this extension [here](https://github.com/Brushfam/openbrush-contracts/tree/main/examples/psp22_extensions/flashmint).
