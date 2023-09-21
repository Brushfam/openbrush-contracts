// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

/// Extension of [`PSP37`] that allows token holders to destroy their tokens
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
pub type PSP37BurnableRef = dyn PSP37Burnable;

#[openbrush::trait_definition]
pub trait PSP37Burnable {
    /// Destroys `amount` tokens of token type `id` from `from`
    ///
    /// See [`PSP37::_burn_from`].
    #[ink(message)]
    fn burn(&mut self, from: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP37Error>;
}
