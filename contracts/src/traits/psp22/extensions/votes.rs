// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

use crate::traits::{
    errors::GovernanceError,
    governance::utils::votes::*,
};
use openbrush::traits::AccountId;
pub use openbrush::utils::checkpoints::Checkpoint;

/// Extension of ERC20 to support Compound-like voting and delegation.
///
/// This extension keeps a history (checkpoints) of each account's vote power. Vote power can be delegated either
/// by calling the `delegate` function directly, or by providing a signature to be used with `delegate_by_sig`. Voting
/// power can be queried through the public accessors `get_votes` and `get_past_votes`.
///
/// By default, token balance does not account for voting power. This makes transfers cheaper. The downside is that it
/// requires users to delegate to themselves in order to activate checkpoints and have their voting power tracked.
#[openbrush::trait_definition]
pub trait PSP22Votes {
    /// Get number of checkpoints for `account`.
    #[ink(message)]
    fn num_checkpoints(&self, account: AccountId) -> u32;

    /// Get the `pos`-th checkpoint for `account`.
    #[ink(message)]
    fn checkpoints(&self, account: AccountId, pos: u32) -> Result<Checkpoint, GovernanceError>;
}

#[openbrush::wrapper]
pub type PSP22VotesWrapper = dyn PSP22Votes + Votes;
