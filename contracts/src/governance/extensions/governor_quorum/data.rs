use crate::checkpoint::Checkpoints;

#[derive(Debug, Default)]
#[openbrush::storage_item]
pub struct Data {
    #[lazy]
    pub quorum_numerator_history: Checkpoints,
}
