use crate::traits::governance::ProposalId;
use openbrush::traits::String;
use openbrush::traits::{AccountId, Balance, Timestamp};

pub trait GovernorEvents {
    fn emit_proposal_created(
        &self,
        proposal_id: ProposalId,
        proposer: AccountId,
        targets: Vec<AccountId>,
        values: Vec<Balance>,
        signatures: Vec<String>,
        calldatas: Vec<Vec<u8>>,
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
