// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

/// Extension of [`PSP34`] that allows token holders to destroy their tokens
pub use crate::traits::errors::PSP34Error;
pub use crate::traits::psp34::Id;
use openbrush::traits::AccountId;

#[openbrush::wrapper]
pub type PSP34BurnableRef = dyn PSP34Burnable;

#[openbrush::trait_definition]
pub trait PSP34Burnable {
    /// Destroys token with id equal to `id` from `account`
    ///
    /// Caller must be approved to transfer tokens from `account`
    /// or to transfer token with `id`
    #[ink(message)]
    fn burn(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error>;
}
