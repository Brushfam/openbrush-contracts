#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    #[lazy]
    pub proposal_threshold: u128,
    #[lazy]
    pub voting_delay: u64,
    #[lazy]
    pub voting_period: u64,
}
