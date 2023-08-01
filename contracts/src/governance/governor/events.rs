use crate::traits::governance::{
    ProposalId,
    Transaction,
};
use ink::prelude::vec::Vec;
use openbrush::traits::{
    AccountId,
    Balance,
    String,
    Timestamp,
};

pub trait GovernorEvents {
    fn emit_proposal_created(
        &self,
        proposal_id: ProposalId,
        proposer: AccountId,
        transactions: Vec<Transaction>,
        vote_start: Timestamp,
        vote_end: Timestamp,
        description: String,
    );

    fn emit_proposal_cancelled(&self, proposal_id: ProposalId);

    fn emit_proposal_executed(&self, proposal_id: ProposalId);

    fn emit_vote_cast(&self, proposal_id: ProposalId, voter: AccountId, support: u8, weight: Balance, reason: String);

    fn emit_vote_cast_with_params(
        &self,
        proposal_id: ProposalId,
        voter: AccountId,
        support: u8,
        weight: Balance,
        reason: String,
        params: Vec<u8>,
    );
}
