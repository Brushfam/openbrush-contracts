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

pub trait PSP22PalletMetadataImpl: Storage<psp22_pallet::Data> {
    fn token_name(&self) -> Option<String> {
        let self_ = self.data();
        let name = self_
            .pallet_assets
            .get_or_default()
            .metadata_name(self_.asset_id.get_or_default());

        if name.is_empty() {
            None
        } else {
            Some(String::from_utf8(name).expect("Invalid UTF-8 string for token"))
        }
    }

    fn token_symbol(&self) -> Option<String> {
        let self_ = self.data();
        let symbol = self_
            .pallet_assets
            .get_or_default()
            .metadata_symbol(self_.asset_id.get_or_default());

        if symbol.is_empty() {
            None
        } else {
            Some(String::from_utf8(symbol).expect("Invalid UTF-8 string for token"))
        }
    }

    fn token_decimals(&self) -> u8 {
        let self_ = self.data();
        self_
            .pallet_assets
            .get_or_default()
            .metadata_decimals(self_.asset_id.get_or_default())
    }
}
