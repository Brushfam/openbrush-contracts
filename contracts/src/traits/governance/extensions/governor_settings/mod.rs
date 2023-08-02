use crate::traits::errors::GovernanceError;

#[openbrush::trait_definition]
pub trait GovernorSettings {
    #[ink(message)]
    fn set_voting_delay(&mut self, new_voting_delay: u128) -> Result<(), GovernanceError>;

    #[ink(message)]
    fn set_voting_period(&mut self, new_voting_period: u128) -> Result<(), GovernanceError>;

    #[ink(message)]
    fn set_proposal_threshold(&mut self, new_proposal_threshold: u128) -> Result<(), GovernanceError>;

    #[ink(message)]
    fn voting_delay(&self) -> u128;

    #[ink(message)]
    fn voting_period(&self) -> u128;

    #[ink(message)]
    fn proposal_threshold(&self) -> u128;
}
