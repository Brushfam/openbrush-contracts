use crate::traits::governance::{
    ProposalId,
    ProposalVote,
};
use openbrush::{
    storage::Mapping,
    traits::AccountId,
};
use openbrush::traits::Timestamp;

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    #[lazy]
    pub proposal_threshold: u128,
    #[lazy]
    pub voting_delay: Timestamp,
    #[lazy]
    pub voting_period: Timestamp,
}
