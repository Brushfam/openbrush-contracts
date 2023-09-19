// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::{
    psp22,
    psp22::extensions::metadata,
    traits::psp22::{
        extensions::metadata::*,
        *,
    },
};
use openbrush::traits::Storage;
pub use openbrush::traits::String;
pub use psp22::{
    Internal as _,
    InternalImpl as _,
    PSP22Impl,
};

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    #[lazy]
    pub name: Option<String>,
    #[lazy]
    pub symbol: Option<String>,
    #[lazy]
    pub decimals: u8,
}

pub trait PSP22MetadataImpl: Storage<Data> {
    fn token_name(&self) -> Option<String> {
        self.data().name.get_or_default()
    }

    fn token_symbol(&self) -> Option<String> {
        self.data().symbol.get_or_default()
    }

    fn token_decimals(&self) -> u8 {
        self.data().decimals.get_or_default()
    }
}
