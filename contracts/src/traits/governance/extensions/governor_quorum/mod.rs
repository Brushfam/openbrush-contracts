use crate::traits::errors::GovernanceError;
use openbrush::traits::Timestamp;

#[openbrush::trait_definition]
pub trait Quorum {
    #[ink(message)]
    fn quorum_numerator(&self) -> u128;

    #[ink(message)]
    fn quorum_numerator_at(&self, time_point: Timestamp) -> u128;

    #[ink(message)]
    fn quorum_denominator(&self) -> u128;

    #[ink(message)]
    fn quorum(&self, time_point: Timestamp) -> Result<u128, GovernanceError>;

    #[ink(message)]
    fn update_quorum_numerator(&mut self, numerator: u128) -> Result<(), GovernanceError>;
}

#[openbrush::wrapper]
pub type QuorumRef = dyn Quorum;
