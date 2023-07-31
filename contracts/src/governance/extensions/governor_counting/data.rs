use crate::traits::governance::{ProposalId, ProposalVote};
use openbrush::storage::Mapping;
use openbrush::traits::AccountId;

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    proposal_votes: Mapping<ProposalId, ProposalVote>,

    has_votes: Mapping<(ProposalId, AccountId), bool>,
}
