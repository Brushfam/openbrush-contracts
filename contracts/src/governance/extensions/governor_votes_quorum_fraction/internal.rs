use crate::traits::errors::GovernanceError;

pub trait QuorumInternal {
    fn _update_quorum_numerator(&mut self, numerator: u128) -> Result<(), GovernanceError>;
}
