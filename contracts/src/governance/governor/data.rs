use crate::traits::governance::{ProposalCore, ProposalId};
use ink::env::call::Selector;
use openbrush::storage::Mapping;

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    pub proposals: Mapping<ProposalId, ProposalCore>,
    #[lazy]
    governance_call: Vec<Selector>,
}
