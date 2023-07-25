// Copyright (c) 2023 727.ventures
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

pub use crate::{
    governance::governance::*,
    traits::governance::{
        extensions::counting::*,
        *,
    },
};
use openbrush::traits::{AccountId, Balance, StorageAsRef};
pub use governance::governance::{
    Internal as _,
    InternalImpl as _,
    GovernorImpl,
};
use openbrush::storage::Mapping;
use openbrush::traits::Storage;

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    proposal_votes: Mapping<u128, ProposalVote>
}

#[openbrush::storage_item]
pub struct ProposalVote {
    against_votes: u128,
    for_votes: u128,
    abstain_votes: u128,
    has_voted: Mapping<AccountId, bool>,
}

pub trait GovernorCountingImpl: governor::Internal + Internal + Storage<Data> + GovernorImpl{
    fn counting_mode(&self) -> String {
        "support=bravo&quorum=for,abstain".to_string()
    }

    fn has_voted(&self, proposal_id: u128,voter: AccountId) -> bool {
        self.data().proposal_votes.get(&proposal_id).unwrap_or_default().has_voted.get(&voter).unwrap_or_default()
    }

    fn proposal_votes(&self, proposal_id: u128) -> (u128, u128, u128) {
        let proposal_vote = self.data().proposal_votes.get(&proposal_id).unwrap_or_default();
        (proposal_vote.for_votes, proposal_vote.against_votes, proposal_vote.abstain_votes)
    }
}

pub trait Internal {
    fn _quorum_reached(&self, proposal_id: u128) -> Result<bool, GovernorError>;

    fn _vote_succeeded(&self, proposal_id: u128) -> bool;

    fn _count_vote(
        &self,
        proposal_id: u128,
        account: AccountId,
        support: u8,
        weight: Balance
    ) -> Result<(), GovernorError>;
}

pub trait InternalImpl: Internal + Storage<Data> + GovernorCountingImpl + GovernorImpl {
    fn _quorum_reached(&self, proposal_id: u128) -> Result<bool, GovernorError> {
        let proposal_vote = self.data().proposal_votes.get(&proposal_id).unwrap_or_default();
        let quorum = self.quorum(self.proposal_snapshot(&proposal_id));
        let sum_of_votes = proposal_vote.for_votes.checked_add(proposal_vote.abstain_votes).ok_or(GovernorError::Overflow)?;
        Ok(sum_of_votes >= quorum)
    }

    fn _vote_succeeded(&self, proposal_id: u128) -> bool {
        let proposal_vote = self.data().proposal_votes.get(&proposal_id).unwrap_or_default();
        proposal_vote.for_votes > proposal_vote.against_votes
    }

    fn _count_vote(
        &mut self,
        proposal_id: u128,
        account: AccountId,
        support: u8,
        weight: Balance
    ) -> Result<(), GovernorError> {
        let mut proposal_vote = self.data().proposal_votes.get(&proposal_id).unwrap_or_default();
        if proposal_vote.has_voted.get(&account).unwrap_or_default() {
            return Err(GovernorError::AlreadyCastVote(account.clone()));
        }
        proposal_vote.has_voted.insert(&account, &true);
        match support {
            0 => proposal_vote.against_votes = proposal_vote.against_votes.checked_add(weight).ok_or(GovernorError::Overflow)?,
            1 => proposal_vote.for_votes = proposal_vote.for_votes.checked_add(weight).ok_or(GovernorError::Overflow)?,
            2 => proposal_vote.abstain_votes = proposal_vote.abstain_votes.checked_add(weight).ok_or(GovernorError::Overflow)?,
            _ => return Err(GovernorError::InvalidVoteType),
        }
        self.data().proposal_votes.insert(&proposal_id, &proposal_vote);
        Ok(())
    }
}