// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

use super::OwnableError;
use openbrush::traits::Hash;

/// The Diamond error type. Contract will throw one of this errors.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum DiamondError {
    OwnableError(OwnableError),
    FunctionDoesNotExist,
    EmptyCodeHash,
    ReplaceExisting(Hash),
}

impl From<OwnableError> for DiamondError {
    fn from(error: OwnableError) -> Self {
        DiamondError::OwnableError(error)
    }
}
