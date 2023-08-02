use crate::{
    extensions::governor_votes_quorum_fraction::{
        Data,
        QuorumInternal,
    },
    traits::errors::GovernanceError,
};
use openbrush::traits::{
    Storage,
    Timestamp,
};

pub trait QuorumImpl: Storage<Data> + QuorumInternal {
    fn quorum_numerator(&self) -> u128;

    fn quorum_numerator_at(&self, time_point: Timestamp) -> u128;

    /// may be overriden by the contract
    fn quorum_denominator(&self) -> u128;

    fn quorum(&self, time_point: Timestamp) -> Result<u128, GovernanceError>;

    // TODO: [only-governance]
    fn update_quorum_numerator(&mut self, numerator: u128) -> Result<(), GovernanceError>;
}
