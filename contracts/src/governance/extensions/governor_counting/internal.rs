// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

use crate::{
    governance::{
        extensions::{
            governor_counting::Data,
            governor_quorum::QuorumImpl,
        },
        governor::GovernorStorageGetters,
    },
    traits::{
        errors::GovernanceError,
        governance::{
            ProposalId,
            VoteType,
        },
    },
};
use openbrush::traits::{
    AccountId,
    Balance,
    Storage,
};

pub trait CountingInternal: Storage<Data> + QuorumImpl + GovernorStorageGetters {
    /// Returns true if the quorum is reached for the given proposal, false otherwise
    fn _quorum_reached(&self, proposal_id: ProposalId) -> Result<bool, GovernanceError> {
        let proposal_vote = self.data::<Data>().proposal_votes.get(&proposal_id).unwrap_or_default();
        let num_votes = proposal_vote
            .for_votes
            .checked_add(proposal_vote.abstain_votes)
            .ok_or(GovernanceError::Overflow)?;

        Ok(self.quorum(self._proposal_snapshot(proposal_id)?)? <= num_votes)
    }

    /// Returns true if the proposal has succeeded, false otherwise
    fn _vote_succeeded(&self, proposal_id: ProposalId) -> bool {
        self.data::<Data>()
            .proposal_votes
            .get(&proposal_id)
            .map(|proposal_vote| proposal_vote.for_votes > proposal_vote.against_votes)
            .unwrap_or_default()
    }

    /// Adds a `account`'s vote to `proposal_id` with `weight` votes, to the `support` side.
    fn _count_vote(
        &mut self,
        proposal_id: ProposalId,
        account: AccountId,
        support: VoteType,
        weight: Balance,
        // params: Vec<u8>,
    ) -> Result<(), GovernanceError> {
        let mut proposal_vote = self.data::<Data>().proposal_votes.get(&proposal_id).unwrap_or_default();

        if self.data::<Data>().has_votes.get(&(proposal_id, account)).is_some() {
            return Err(GovernanceError::AlreadyCastVote)?
        }

        self.data::<Data>().has_votes.insert(&(proposal_id, account), &());

        match support {
            VoteType::Against => {
                proposal_vote.against_votes = proposal_vote
                    .against_votes
                    .checked_add(weight)
                    .ok_or(GovernanceError::Overflow)?;
            }
            VoteType::For => {
                proposal_vote.for_votes = proposal_vote
                    .for_votes
                    .checked_add(weight)
                    .ok_or(GovernanceError::Overflow)?;
            }
            VoteType::Abstain => {
                proposal_vote.abstain_votes = proposal_vote
                    .abstain_votes
                    .checked_add(weight)
                    .ok_or(GovernanceError::Overflow)?;
            }
        }

        self.data::<Data>().proposal_votes.insert(&proposal_id, &proposal_vote);

        Ok(())
    }
}
