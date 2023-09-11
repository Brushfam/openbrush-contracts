// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::{
    ownable::*,
    traits::upgradeable::*,
    upgradeable,
};
use openbrush::{
    modifiers,
    traits::{
        Hash,
        Storage,
    },
};
pub use upgradeable::UpgradeableImpl as _;

pub trait UpgradeableImpl: Storage<ownable::Data> {
    #[modifiers(ownable::only_owner)]
    fn set_code_hash(&mut self, new_code_hash: Hash) -> Result<(), UpgradeableError> {
        Self::env()
            .set_code_hash(&new_code_hash)
            .map_err(|_| UpgradeableError::SetCodeHashFailed)
    }
}
