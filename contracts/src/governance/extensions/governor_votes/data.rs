use crate::checkpoint::Checkpoints;
use openbrush::traits::AccountId;

#[derive(Debug, Default)]
#[openbrush::storage_item]
pub struct Data {
    #[lazy]
    pub token: AccountId,
}
