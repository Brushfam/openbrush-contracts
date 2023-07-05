---
sidebar_position: 1
title: PSP22 Pallet Metadata
---

This example shows how you can reuse the implementation of [PSP22](https://github.com/727-Ventures/openbrush-contracts/tree/main/contracts/src/token/psp22_pallet) token with the [PSP22Metadata](https://github.com/727-Ventures/openbrush-contracts/tree/main/contracts/src/token/psp22_pallet/extensions/metadata.rs) extension via `pallet-assets` chain extension.

First, you should implement basic version of [PSP22 Pallet](/smart-contracts/PSP22-Pallet).

## Step 1: Implement features

- Use `openbrush::contract` macro instead of `ink::contract`. 
- Implement `PSP22PalletMetadata` via `#[openbrush::implementation].

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::contract]
#[openbrush::implementation(PSP22Pallet, PSP22PalletMetadata)]
pub mod my_psp22_pallet {
    ...
}
```

## Step 2: Define storage

Declare storage struct and declare the field related to the metadata module data structure.
Then you need to derive the `Storage` trait and mark the corresponding field with
the `#[storage_field]` attribute. Deriving this trait allows you to reuse the
`PSP22Metadata` extension in your `PSP22 Pallet` implementation.

```rust
#[ink(storage)]
#[derive(Default, Storage)]
pub struct Contract {
    #[storage_field]
    pallet: psp22_pallet::Data,
}
```

## Step 3: Define constructor

Define constructor. Your `PSP22Metadata` contract is ready!

```rust
impl Contract {
    /// During instantiation of the contract, you need to pass native tokens as a deposit
    /// for asset creation.
    #[ink(constructor)]
    #[ink(payable)]
    pub fn new(
        asset_id: u32,
        min_balance: Balance,
        total_supply: Balance,
        name: String,
        symbol: String,
        decimal: u8,
    ) -> Self {
        let mut instance = Self::default();

        psp22_pallet::Internal::_create(&mut instance, asset_id, Self::env().account_id(), min_balance)
            .expect("Should create an asset");
        instance.pallet.asset_id.set(&asset_id);
        instance.pallet.origin.set(&Origin::Caller);
        instance
            .pallet
            .pallet_assets
            .get_or_default()
            .set_metadata(asset_id, name.into(), symbol.into(), decimal)
            .expect("Should set metadata");
        psp22_pallet::Internal::_mint_to(&mut instance, Self::env().caller(), total_supply).expect("Should mint");

        instance
    }
}
```

## Final code

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP22Pallet, PSP22PalletMetadata)]
#[openbrush::contract]
pub mod my_psp22_pallet_metadata {
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
        pub fn new(
            asset_id: u32,
            min_balance: Balance,
            total_supply: Balance,
            name: String,
            symbol: String,
            decimal: u8,
        ) -> Self {
            let mut instance = Self::default();

            psp22_pallet::Internal::_create(&mut instance, asset_id, Self::env().account_id(), min_balance)
                .expect("Should create an asset");
            instance.pallet.asset_id.set(&asset_id);
            instance.pallet.origin.set(&Origin::Caller);
            instance
                .pallet
                .pallet_assets
                .get_or_default()
                .set_metadata(asset_id, name.into(), symbol.into(), decimal)
                .expect("Should set metadata");
            psp22_pallet::Internal::_mint_to(&mut instance, Self::env().caller(), total_supply).expect("Should mint");

            instance
        }
    }
}
```

You can also check the documentation for the basic implementation of [PSP22 Pallet](/smart-contracts/PSP22-Pallet).
