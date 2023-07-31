use crate::governance::governor::{Data, GovernorEvents};
use crate::traits::governance::{HashType, ProposalId, Transaction};
use ink::prelude::vec::Vec;
use openbrush::traits::String;
use openbrush::traits::{AccountId, Balance, Storage, Timestamp};
use crate::traits::errors::governance::GovernanceError;

pub trait GovernorInternal: Storage<Data> + GovernorEvents {
    fn _quorum_reached(&self, proposal_id: ProposalId) -> bool;

    fn _vote_succeeded(&self, proposal_id: ProposalId) -> bool;

    fn _get_votes(&self, account: AccountId, time_point: Timestamp, params: Vec<u8>) -> u128;

    fn _count_vote(
        &mut self,
        proposal_id: ProposalId,
        account: AccountId,
        support: u8,
        weight: u128,
        params: Vec<u8>,
    ) -> Result<(), GovernanceError>;

    fn _default_params(&self) -> Vec<u8> {
        Vec::new()
    }

    fn _execute(&mut self, transactions: Vec<Transaction>, description_hash: HashType) -> Result<(), GovernanceError>;

    fn _before_execute(
        &mut self,
        transactions: Vec<Transaction>,
        description_hash: HashType,
    ) -> Result<(), GovernanceError>;

    fn _after_execute(
        &mut self,
        transactions: Vec<Transaction>,
        description_hash: HashType,
    ) -> Result<(), GovernanceError>;

    fn _cancel(&mut self, transactions: Vec<Transaction>, description_hash: HashType) -> Result<(), GovernanceError>;

    fn _cast_vote(&mut self, proposal_id: ProposalId, support: u8, reason: String) -> Result<Balance, GovernanceError> {
        self._cast_vote_with_params(proposal_id, support, reason, self._default_params())
    }

    fn _cast_vote_with_params(
        &mut self,
        proposal_id: ProposalId,
        support: u8,
        reason: String,
        params: Vec<u8>,
    ) -> Result<Balance, GovernanceError>;

    fn _executor(&self) -> AccountId;

    fn _is_valid_description_for_proposer(&self, proposer: AccountId, description: String) -> bool;
}
