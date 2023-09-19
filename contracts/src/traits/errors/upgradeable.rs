// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

use super::{
    AccessControlError,
    OwnableError,
};
use openbrush::traits::String;

/// The PSP34 error type. Contract will throw one of this errors.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum UpgradeableError {
    /// Custom error type for cases if writer of traits added own restrictions
    Custom(String),
    /// Returned if the upgrade failed
    SetCodeHashFailed,
    OwnableError(OwnableError),
    AccessControlError(AccessControlError),
}

impl From<OwnableError> for UpgradeableError {
    fn from(error: OwnableError) -> Self {
        UpgradeableError::OwnableError(error)
    }
}

impl From<AccessControlError> for UpgradeableError {
    fn from(error: AccessControlError) -> Self {
        UpgradeableError::AccessControlError(error)
    }
}
