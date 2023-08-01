use crate::traits::governance::{
    ProposalCore,
    ProposalId,
    Transaction,
};
use ink::prelude::vec::Vec;
use openbrush::storage::Mapping;

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    pub proposals: Mapping<ProposalId, ProposalCore>,
    #[lazy]
    pub governance_call: Vec<Transaction>,
}
