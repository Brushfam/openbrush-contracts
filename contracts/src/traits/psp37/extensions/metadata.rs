// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

/// Metadata for PSP37
pub use crate::traits::psp37::Id;
use openbrush::traits::String;

#[openbrush::wrapper]
pub type PSP37MetadataRef = dyn PSP37Metadata;

#[openbrush::trait_definition]
pub trait PSP37Metadata {
    #[ink(message)]
    fn get_attribute(&self, id: Id, key: String) -> Option<String>;
}
