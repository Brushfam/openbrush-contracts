use openbrush::storage::Mapping;
use openbrush::traits::AccountId;
use crate::utils::checkpoint::Checkpoints;

#[openbrush::storage_item]
pub struct Data {
    delegation: Mapping<AccountId, AccountId>,
    delegate_checkpoints: Mapping<AccountId, Checkpoints>,
    total_checkpoints: Checkpoints,
}