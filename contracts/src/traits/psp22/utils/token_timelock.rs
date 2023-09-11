// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::traits::errors::PSP22TokenTimelockError;
use openbrush::traits::{
    AccountId,
    Timestamp,
};

#[openbrush::wrapper]
pub type PSP22TokenTimelockRef = dyn PSP22TokenTimelock;

#[openbrush::trait_definition]
pub trait PSP22TokenTimelock {
    /// Returns the token address
    #[ink(message)]
    fn token(&self) -> Option<AccountId>;

    /// Returns the beneficiary of the tokens
    #[ink(message)]
    fn beneficiary(&self) -> Option<AccountId>;

    /// Returns the timestamp when the tokens are released
    #[ink(message)]
    fn release_time(&self) -> Timestamp;

    /// Transfers the tokens held by timelock to the beneficairy
    #[ink(message)]
    fn release(&mut self) -> Result<(), PSP22TokenTimelockError>;
}
