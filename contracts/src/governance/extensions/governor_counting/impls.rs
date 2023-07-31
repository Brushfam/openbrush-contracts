use openbrush::traits::{Storage, String};
use crate::traits::governance::ProposalId;
use openbrush::traits::{AccountId, Balance};
use crate::governance::extensions::governor_counting::Data;
use crate::governance::governor::GovernorImpl;

pub trait GovernorCountingImpl: Storage<Data> + GovernorImpl{
    fn counting_mode(&self) -> String;

    fn has_voted(&self, proposal_id: ProposalId, account: AccountId) -> bool;

    fn proposal_votes(&self, proposal_id: ProposalId) -> (Balance, Balance);
}
