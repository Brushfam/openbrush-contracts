use crate::traits::{
    errors::GovernanceError,
    governance::ProposalId,
};
use openbrush::traits::{
    AccountId,
    Balance,
    String,
};

#[openbrush::trait_definition]
pub trait GovernorCounting {
    #[ink(message)]
    fn counting_mode(&self) -> String;

    #[ink(message)]
    fn has_voted(&self, proposal_id: ProposalId, account: AccountId) -> bool;

    #[ink(message)]
    fn proposal_votes(&self, proposal_id: ProposalId) -> Result<(Balance, Balance, Balance), GovernanceError>;
}

#[openbrush::wrapper]
pub type GovernorCountingRef = dyn GovernorCounting;
