use openbrush::traits::{AccountId, Balance, Storage};
use ink::prelude::vec::Vec;
use crate::governance::extensions::governor_counting::Data;
use crate::traits::governance::ProposalId;


pub trait CountingInternal: Storage<Data> {
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