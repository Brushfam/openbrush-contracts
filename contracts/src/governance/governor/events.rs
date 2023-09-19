// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::traits::governance::{
    ProposalId,
    Transaction,
    VoteType,
};
pub use ink::prelude::vec::Vec;
pub use openbrush::traits::{
    AccountId,
    Balance,
    String,
    Timestamp,
};

pub trait GovernorEvents {
    /// Emitted when a proposal is created
    fn emit_proposal_created(
        &self,
        _proposal_id: ProposalId,
        _proposer: AccountId,
        _transactions: Vec<Transaction>,
        _vote_start: Timestamp,
        _vote_end: Timestamp,
        _description: String,
    ) {
    }

    /// Emitted when a proposal is canceled
    fn emit_proposal_canceled(&self, _proposal_id: ProposalId) {}

    /// Emitted when a proposal is executed
    fn emit_proposal_executed(&self, _proposal_id: ProposalId) {}

    /// Emitted when the vote is casted
    fn emit_vote_cast(
        &self,
        _proposal_id: ProposalId,
        _voter: AccountId,
        _support: VoteType,
        _weight: Balance,
        _reason: String,
    ) {
    }

    /// Emitted when the vote is casted with params
    fn emit_vote_cast_with_params(
        &self,
        _proposal_id: ProposalId,
        _voter: AccountId,
        _support: VoteType,
        _weight: Balance,
        _reason: String,
        _params: Vec<u8>,
    ) {
    }
}
