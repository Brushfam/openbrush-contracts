use crate::utils::checkpoint::Checkpoints;
use openbrush::{
    storage::Mapping,
    traits::AccountId,
};

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    pub delegation: Mapping<Option<AccountId>, AccountId>,
    pub delegate_checkpoints: Mapping<AccountId, Checkpoints>,
    #[lazy]
    pub total_checkpoints: Checkpoints,
}
