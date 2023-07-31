use openbrush::traits::String;
use crate::traits::governance::ProposalId;
use openbrush::traits::{AccountId, Balance};

pub trait GovernorCountingImpl {
    fn counting_mode(&self) -> String;

    fn has_voted(&self, proposal_id: ProposalId, account: AccountId) -> bool;

    fn proposal_votes(&self, proposal_id: ProposalId) -> (Balance, Balance);
}
