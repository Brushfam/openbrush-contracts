// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::traits::errors::NoncesError;
use openbrush::traits::AccountId;

/// Provides tracking nonces for addresses. Nonces will only increment.
#[openbrush::trait_definition]
pub trait Nonces {
    /// Returns the nonce of `account`.
    #[ink(message)]
    fn nonces(&self, account: AccountId) -> u64;
}
