use ink::storage::traits::StorageLayout;
use openbrush::storage::Mapping;
use openbrush::traits::AccountId;
use crate::traits::governance::{ProposalId, ProposalVote};

#[openbrush::storage_item]
pub struct Data {
    proposal_votes: Mapping<ProposalId, ProposalVote>,
}

