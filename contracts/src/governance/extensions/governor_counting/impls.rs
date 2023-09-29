// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

use crate::governance::extensions::governor_counting::{
    CountingInternal,
    Data,
};
pub use crate::{
    governance::extensions::governor_counting,
    traits::governance::extensions::governor_counting::*,
};
use openbrush::traits::{
    AccountId,
    Storage,
};

/// Extension of `Governor` for simple, 3 options, vote counting.
pub trait GovernorCountingImpl: Storage<Data> + CountingInternal {
    /// Returns `true` if the account has voted for the proposal, `false` otherwise
    fn has_voted(&self, proposal_id: ProposalId, account: AccountId) -> bool {
        self.data::<Data>().has_votes.get(&(proposal_id, account)).is_some()
    }
    /// Returns the tuple (for, against, abstain) votes for a proposal, where `for` is the total
    /// number of votes for the proposal, `against` is the total number of votes against the
    /// proposal, and `abstain` is the total number of abstained votes.
    fn proposal_votes(&self, proposal_id: ProposalId) -> Result<ProposalVote, GovernanceError> {
        let proposal_vote = self.data::<Data>().proposal_votes.get(&proposal_id).unwrap_or_default();
        Ok(ProposalVote { ..proposal_vote })
    }
}
