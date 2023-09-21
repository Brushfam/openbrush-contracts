// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::traits::errors::UpgradeableError;
use openbrush::traits::Hash;

#[openbrush::wrapper]
pub type UpgradeableRef = dyn Upgradeable;

/// A common trait that exposes ink!'s `set_code_hash` function as a mean to upgrade the contract
#[openbrush::trait_definition]
pub trait Upgradeable {
    #[ink(message)]
    fn set_code_hash(&mut self, new_code_hash: Hash) -> Result<(), UpgradeableError>;
}
