// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::{
    psp22_pallet,
    psp22_pallet::extensions::metadata,
    traits::psp22::{
        extensions::metadata::*,
        *,
    },
};
pub use ink::env::DefaultEnvironment;
use openbrush::traits::Storage;
use ink::prelude::vec::*;
pub use openbrush::traits::String;
pub use pallet_assets_chain_extension::traits::{
    Error,
    Origin,
    PalletAssets,
};
pub use psp22_pallet::{
    Internal as _,
    InternalImpl as _,
    PSP22PalletImpl,
};

pub trait PSP22PalletMetadataImpl: PSP22PalletMetadataInternal {
    fn token_name(&self) -> Option<String> {
        let name = self._name();

        if name.is_empty() {
            None
        } else {
            Some(String::from_utf8(name).expect("Invalid UTF-8 string for token"))
        }
    }

    fn token_symbol(&self) -> Option<String> {
        let symbol = self._symbol();

        if symbol.is_empty() {
            None
        } else {
            Some(String::from_utf8(symbol).expect("Invalid UTF-8 string for token"))
        }
    }

    fn token_decimals(&self) -> u8 {
        self._decimals()
    }
}

pub trait PSP22PalletMetadataInternal: Storage<psp22_pallet::Data> {
    fn _name(&self) -> Vec<u8> {
        self.data()
            .pallet_assets
            .get_or_default()
            .metadata_name(self.data().asset_id.get_or_default())
    }

    fn _symbol(&self) -> Vec<u8> {
        self.data()
            .pallet_assets
            .get_or_default()
            .metadata_symbol(self.data().asset_id.get_or_default())
    }

    fn _decimals(&self) -> u8 {
        self.data()
            .pallet_assets
            .get_or_default()
            .metadata_decimals(self.data().asset_id.get_or_default())
    }
}
