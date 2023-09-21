// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::traits::{
    errors::GovernanceError,
    types::Signature,
};
use openbrush::traits::{
    AccountId,
    Balance,
    Timestamp,
};

/// Common interface for `PSP22Votes`, and other `Votes`-enabled contracts.
#[openbrush::trait_definition]
pub trait Votes {
    /// The amount of votes owned by `account`.
    #[ink(message)]
    fn get_votes(&self, account: AccountId) -> Balance;

    /// The amount of votes delegated to `account` at the time `timestamp`.
    #[ink(message)]
    fn get_past_votes(&self, account: AccountId, timestamp: Timestamp) -> Result<Balance, GovernanceError>;

    /// The total amount of votes at the time `timestamp`.
    #[ink(message)]
    fn get_past_total_supply(&self, timestamp: Timestamp) -> Result<Balance, GovernanceError>;

    /// Returns the address delegated to by `delegator`.
    #[ink(message)]
    fn delegates(&mut self, delegator: AccountId) -> Option<AccountId>;

    /// Delegate votes from `signer` to `delegatee`.
    #[ink(message)]
    fn delegate(&mut self, delegatee: AccountId) -> Result<(), GovernanceError>;

    /// Delegate votes from `signer` to `delegatee` using a signature.
    #[ink(message)]
    fn delegate_by_signature(
        &mut self,
        signer: AccountId,
        delegatee: AccountId,
        nonce: u64,
        expiry: Timestamp,
        signature: Signature,
    ) -> Result<(), GovernanceError>;
}

#[openbrush::wrapper]
pub type VotesRef = dyn Votes;
