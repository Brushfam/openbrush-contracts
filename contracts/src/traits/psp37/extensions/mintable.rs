// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

/// Extension of [`PSP37`] that allows minting of new tokens
use crate::traits::psp37::{
    Id,
    PSP37Error,
};
use ink::prelude::vec::Vec;
use openbrush::traits::{
    AccountId,
    Balance,
};

#[openbrush::wrapper]
pub type PSP37MintableRef = dyn PSP37Mintable;

#[openbrush::trait_definition]
pub trait PSP37Mintable {
    /// Mints `amount` tokens of token type `id` to `to`
    ///
    /// See [`PSP37::_mint_to`].
    #[ink(message)]
    fn mint(&mut self, to: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP37Error>;
}
