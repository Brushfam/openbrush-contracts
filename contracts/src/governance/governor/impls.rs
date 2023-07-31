use crate::governance::governor::{Data, GovernorEvents};
use crate::traits::errors::GovernanceError;
use crate::traits::governance::{HashType, ProposalId, ProposalState, Transaction};
use crate::utils::crypto::SignatureType;
use openbrush::traits::{AccountId, Balance, Storage, Timestamp};

pub trait GovernorImpl: Storage<Data> + GovernorEvents {
    fn hash_proposal(&self, transactions: Vec<Transaction>, description_hash: HashType) -> HashType;

    fn state(&self, proposal_id: ProposalId) -> ProposalState;

    fn proposal_snapshot(&self, proposal_id: ProposalId) -> u128;

    fn proposal_deadline(&self, proposal_id: ProposalId) -> Timestamp;

    fn proposal_proposer(&self, proposal_id: ProposalId) -> AccountId;

    fn voting_delay(&self) -> u64;

    fn voting_period(&self) -> u64;

    fn quorum(&self, time_point: Timestamp) -> u128;

    fn get_votes(&self, account: AccountId, time_point: Timestamp) -> u128;

    fn get_votes_with_params(&self, account: AccountId, time_point: Timestamp, params: Vec<u8>) -> u128;

    fn has_voted(&self, proposal_id: ProposalId, account: AccountId) -> bool;

    fn propose(&mut self, transactions: Vec<Transaction>, description: String) -> Result<ProposalId, GovernanceError>;

    fn execute(&mut self, proposal_id: ProposalId) -> Result<(), GovernanceError>;

    fn cancel(&mut self, proposal_id: ProposalId) -> Result<(), GovernanceError>;

    fn cast_vote(&mut self, proposal_id: ProposalId, support: u8) -> Result<Balance, GovernanceError>;

    fn cast_vote_with_reason(
        &mut self,
        proposal_id: ProposalId,
        support: u8,
        reason: String,
    ) -> Result<Balance, GovernanceError>;

    fn cast_vote_with_reason_and_params(
        &mut self,
        proposal_id: ProposalId,
        support: u8,
        reason: String,
        params: Vec<u8>,
    ) -> Result<Balance, GovernanceError>;

    fn cast_vote_with_signature(
        &mut self,
        proposal_id: ProposalId,
        support: u8,
        reason: String,
        signature: SignatureType,
    ) -> Result<Balance, GovernanceError>;

    fn cast_vote_with_signature_and_params(
        &mut self,
        proposal_id: ProposalId,
        support: u8,
        reason: String,
        signature: SignatureType,
        params: Vec<u8>,
    ) -> Result<Balance, GovernanceError>;
}
