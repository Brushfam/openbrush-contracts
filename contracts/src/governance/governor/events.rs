use crate::traits::governance::{
    ProposalId,
    Transaction,
    VoteType,
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
        _proposal_id: ProposalId,
        _proposer: AccountId,
        _transactions: Vec<Transaction>,
        _vote_start: Timestamp,
        _vote_end: Timestamp,
        _description: String,
    ) {
        ()
    }

    fn emit_proposal_cancelled(&self, _proposal_id: ProposalId) {
        ()
    }

    fn emit_proposal_executed(&self, _proposal_id: ProposalId) {
        ()
    }

    fn emit_vote_cast(
        &self,
        _proposal_id: ProposalId,
        _voter: AccountId,
        _support: VoteType,
        _weight: Balance,
        _reason: String,
    ) {
        ()
    }

    fn emit_vote_cast_with_params(
        &self,
        _proposal_id: ProposalId,
        _voter: AccountId,
        _support: VoteType,
        _weight: Balance,
        _reason: String,
        _params: Vec<u8>,
    ) {
        ()
    }
}
