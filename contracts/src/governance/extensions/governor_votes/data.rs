use crate::utils::checkpoint::Checkpoints;
use openbrush::{
    storage::Mapping,
    traits::AccountId,
};

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    delegation: Mapping<AccountId, AccountId>,
    delegate_checkpoints: Mapping<AccountId, Checkpoints>,
    #[lazy]
    total_checkpoints: Checkpoints,
}
