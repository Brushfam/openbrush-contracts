use crate::traits::governance::{
    ProposalId,
    ProposalVote,
};
use openbrush::{
    storage::Mapping,
    traits::AccountId,
};

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    pub proposal_votes: Mapping<ProposalId, ProposalVote>,

    pub has_votes: Mapping<(ProposalId, AccountId), bool>,
}
