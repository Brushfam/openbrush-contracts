use crate::traits::governance::{ProposalCore, ProposalId, Selector};
use openbrush::storage::Mapping;
use ink::prelude::vec::Vec;

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    pub proposals: Mapping<ProposalId, ProposalCore>,
    #[lazy]
    governance_call: Vec<Selector>,
}
