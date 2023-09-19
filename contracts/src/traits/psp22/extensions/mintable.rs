// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

/// Extension of [`PSP22`] that allows create `amount` tokens
/// and assigns them to `account`, increasing the total supply
pub use crate::traits::errors::PSP22Error;
use openbrush::traits::{
    AccountId,
    Balance,
};

#[openbrush::wrapper]
pub type PSP22MintableRef = dyn PSP22Mintable;

#[openbrush::trait_definition]
pub trait PSP22Mintable {
    /// Minting `amount` tokens to the account.
    ///
    /// See [`PSP22::_mint_to`].
    #[ink(message)]
    fn mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error>;
}
