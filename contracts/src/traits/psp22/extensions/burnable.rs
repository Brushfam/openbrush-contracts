// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

/// Extension of [`PSP22`] that allows token holders to destroy both their own
/// tokens and those that they have an allowance for.
pub use crate::traits::errors::PSP22Error;
use openbrush::traits::{
    AccountId,
    Balance,
};

#[openbrush::wrapper]
pub type PSP22BurnableRef = dyn PSP22Burnable;

#[openbrush::trait_definition]
pub trait PSP22Burnable {
    /// Destroys `amount` tokens from `account`, deducting from the caller's
    /// allowance.
    ///
    /// See [`PSP22::_burn_from`].
    #[ink(message)]
    fn burn(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error>;
}
