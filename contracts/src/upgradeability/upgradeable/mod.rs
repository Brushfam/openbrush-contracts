pub use crate::{
    traits::upgradeable::*,
    upgradeable,
};
use openbrush::traits::{
    DefaultEnv,
    Hash,
};
pub use upgradeable::UpgradeableImpl as _;

pub trait UpgradeableImpl: Sized {
    fn set_code_hash(&mut self, new_code_hash: Hash) -> Result<(), UpgradeableError> {
        Self::env()
            .set_code_hash(&new_code_hash)
            .map_err(|_| UpgradeableError::SetCodeHashFailed)
    }
}
