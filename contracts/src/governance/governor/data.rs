use crate::traits::{
    errors::GovernanceError,
    governance::{
        ProposalCore,
        ProposalId,
        Transaction,
    },
};
use ink::prelude::collections::VecDeque;
use openbrush::{
    storage::Mapping,
    traits::{
        Storage,
        Timestamp,
    },
};

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    pub proposals: Mapping<ProposalId, ProposalCore>,
    #[lazy]
    pub governance_call: VecDeque<Transaction>,
}

pub trait GovernorStorageGetters: Storage<Data> {
    fn _proposal_snapshot(&self, proposal_id: ProposalId) -> Result<Timestamp, GovernanceError> {
        Ok(self
            .data::<Data>()
            .proposals
            .get(&proposal_id)
            .ok_or(GovernanceError::ProposalNotFound)?
            .vote_start)
    }
}
