// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::traits::{
    errors::GovernanceError,
    governance::{
        ProposalId,
        ProposalVote,
    },
};
use openbrush::traits::AccountId;

/// Extension of `Governor` for simple, 3 options, vote counting.
#[openbrush::trait_definition]
pub trait GovernorCounting {
    /// Returns `true` if the account has voted for the proposal, `false` otherwise
    #[ink(message)]
    fn has_voted(&self, proposal_id: ProposalId, account: AccountId) -> bool;

    /// Returns the tuple (for, against, abstain) votes for a proposal, where `for` is the total
    /// number of votes for the proposal, `against` is the total number of votes against the
    /// proposal, and `abstain` is the total number of abstained votes.
    #[ink(message)]
    fn proposal_votes(&self, proposal_id: ProposalId) -> Result<ProposalVote, GovernanceError>;
}

#[openbrush::wrapper]
pub type GovernorCountingRef = dyn GovernorCounting;
