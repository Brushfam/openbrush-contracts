---
sidebar_position: 2
title: PSP22 Pallet Mintable
---

This example shows how you can reuse the implementation of
[PSP22 Pallet](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/token/psp22_pallet) token with [PSP22Mintable](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/token/psp22_pallet/extensions/mintable.rs) extension via `pallet-assets` chain extension.

## How to use this extension

First, you should implement basic version of [PSP22 Pallet](/smart-contracts/PSP22-Pallet).

After you can just add implementation of PSP22PalletMintable via `#[openbrush::implementation(PSP22PalletMintable)]` attribute.

# Final code

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP22Pallet, PSP22PalletMintable)]
#[openbrush::contract]
pub mod my_psp22_pallet_mintable {
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        pallet: psp22_pallet::Data,
    }

    impl Contract {
        /// During instantiation of the contract, you need to pass native tokens as a deposit
        /// for asset creation.
        #[ink(constructor)]
        #[ink(payable)]
        pub fn new(asset_id: u32, min_balance: Balance, total_supply: Balance) -> Self {
            let mut instance = Self::default();
            let caller = instance.env().caller();

            psp22_pallet::Internal::_create(&mut instance, asset_id, Self::env().account_id(), min_balance)
                .expect("Should create an asset");
            instance.pallet.asset_id.set(&asset_id);
            instance.pallet.origin.set(&Origin::Caller);
            psp22_pallet::Internal::_mint_to(&mut instance, caller, total_supply).expect("Should mint_to");

            instance
        }
    }
}
```

And that's it! Your `PSP22 Pallet` is now extended by the `PSP22Mintable` extension and ready to use its functions!
