// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

/// Metadata for PSP34
pub use crate::traits::errors::PSP34Error;
pub use crate::traits::psp34::Id;
use openbrush::traits::String;

#[openbrush::wrapper]
pub type PSP34MetadataRef = dyn PSP34Metadata;

#[openbrush::trait_definition]
pub trait PSP34Metadata {
    /// Returns the attribute of `id` for the given `key`.
    ///
    /// If `id` is a collection id of the token, it returns attributes for collection.
    #[ink(message)]
    fn get_attribute(&self, id: Id, key: String) -> Option<String>;
}
