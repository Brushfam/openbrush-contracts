use crate::utils::checkpoint::Checkpoints;
use openbrush::{
    storage::Mapping,
    traits::AccountId,
};

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    #[lazy]
    pub quorum_numerator_history: Checkpoints,
}
