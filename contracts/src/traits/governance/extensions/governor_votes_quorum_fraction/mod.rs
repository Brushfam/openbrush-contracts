use crate::traits::{
    governance::GovernanceError,
};
use openbrush::traits::Timestamp;

#[openbrush::trait_definition]
pub trait GovernorVotesQuorumFraction {
    #[ink(message)]
    fn quorum_numerator(&self) -> u128;

    #[ink(message)]
    fn quorum_numerator_by_timestamp(&self, timestamp: Timestamp) -> u128;

    #[ink(message)]
    fn quorum_denominator(&self) -> u128;

    #[ink(message)]
    fn quorum(&self, timestamp: Timestamp) -> u128;

    #[ink(message)]
    // todo: #[openbrush::modifiers(only_governance)]
    fn update_quorum_numerator(&mut self, new_quorum_numerator: u128) -> Result<(), GovernanceError>;
}
