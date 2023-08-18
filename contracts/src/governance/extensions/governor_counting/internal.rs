// Copyright (c) 2023 Brushfam
// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use crate::{
    governance::extensions::{
        governor_quorum::QuorumImpl,
        governor_counting::Data,
    },
    governance::governor::GovernorStorageGetters,
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
            return Err(GovernanceError::AlreadyCastVote(account))?
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
