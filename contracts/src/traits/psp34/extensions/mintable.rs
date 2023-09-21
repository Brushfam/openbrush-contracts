// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

/// Extension of [`PSP34`] that exposes the mint function
pub use crate::traits::errors::PSP34Error;
pub use crate::traits::psp34::Id;
use openbrush::traits::AccountId;

#[openbrush::wrapper]
pub type PSP34MintableRef = dyn PSP34Mintable;

#[openbrush::trait_definition]
pub trait PSP34Mintable {
    /// Mints a new token with `id`.
    ///
    /// See [`PSP34::_mint_to`].
    #[ink(message)]
    fn mint(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error>;
}
