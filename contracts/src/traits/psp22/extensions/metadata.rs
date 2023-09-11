// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

use openbrush::traits::String;

#[openbrush::wrapper]
pub type PSP22MetadataRef = dyn PSP22Metadata;

/// Trait that contains metadata
#[openbrush::trait_definition]
pub trait PSP22Metadata {
    /// Returns the token name.
    #[ink(message)]
    fn token_name(&self) -> Option<String>;

    /// Returns the token symbol.
    #[ink(message)]
    fn token_symbol(&self) -> Option<String>;

    /// Returns the token decimals.
    #[ink(message)]
    fn token_decimals(&self) -> u8;
}
