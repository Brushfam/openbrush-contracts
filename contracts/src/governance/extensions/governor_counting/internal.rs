use openbrush::traits::{AccountId, Balance};
use ink::prelude::vec::Vec;
use crate::traits::governance::ProposalId;


pub trait CountingInternal {
    fn _quorum_reached(&self, proposal_id: ProposalId) -> bool;

    fn _vote_succeeded(&self, proposal_id: ProposalId) -> bool;

    fn _count_vote(
        &mut self,
        proposal_id: ProposalId,
        account: AccountId,
        support: u8,
        weight: Balance,
        params: Vec<u8>,
    );
}