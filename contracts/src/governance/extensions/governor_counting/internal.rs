use openbrush::traits::{AccountId, Balance, Storage};
use crate::governance::extensions::governor_counting::Data;
use crate::governance::governor::GovernorImpl;
use crate::traits::errors::GovernanceError;
use crate::traits::governance::{ProposalId, VoteType};


pub trait CountingInternal: Storage<Data> + GovernorImpl{
    fn _quorum_reached(&self, proposal_id: ProposalId) -> Result<bool, GovernanceError> {
        let proposal_vote = self
            .data::<Data>()
            .proposal_votes
            .get(&proposal_id)
            .ok_or(GovernanceError::ProposalNotFound)?;
        let num_votes = proposal_vote
            .for_votes
            .checked_add(proposal_vote.against_votes)
            .ok_or(GovernanceError::Overflow)?;
        Ok(self.quorum(self.proposal_snapshot(proposal_id)?) <= num_votes)
    }

    fn _vote_succeeded(&self, proposal_id: ProposalId) -> Result<bool, GovernanceError> {
        let proposal_vote = self
            .data::<Data>()
            .proposal_votes
            .get(&proposal_id)
            .ok_or(GovernanceError::ProposalNotFound)?;
        Ok(proposal_vote.for_votes > proposal_vote.against_votes)
    }

    fn _count_vote(
        &mut self,
        proposal_id: ProposalId,
        account: AccountId,
        support: VoteType,
        weight: Balance,
        //params: Vec<u8>,
    ) -> Result<(), GovernanceError> {
        let mut proposal_vote = self
            .data::<Data>()
            .proposal_votes
            .get(&proposal_id)
            .ok_or(GovernanceError::ProposalNotFound)?;

        if self
            .data::<Data>()
            .has_votes
            .get(&(proposal_id, account))
            .unwrap_or_default() {
            return Err(GovernanceError::AlreadyCastVote(account))?;
        }

        self
            .data::<Data>()
            .has_votes
            .insert(&(proposal_id, account), &true);

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

        Ok(())
    }
}