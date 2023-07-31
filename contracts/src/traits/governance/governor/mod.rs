use crate::traits::errors::GovernanceError;
use crate::traits::governance::{HashType, ProposalId, ProposalState, Transaction};
use crate::utils::crypto::SignatureType;
use openbrush::traits::String;
use openbrush::traits::{AccountId, Balance, Timestamp};

#[openbrush::trait_definition]
pub trait Governor {
    #[ink(message)]
    fn hash_proposal(&self, transactions: Vec<Transaction>, description_hash: HashType) -> HashType;

    #[ink(message)]
    fn state(&self, proposal_id: ProposalId) -> ProposalState;

    #[ink(message)]
    fn proposal_snapshot(&self, proposal_id: ProposalId) -> u128;

    #[ink(message)]
    fn proposal_deadline(&self, proposal_id: ProposalId) -> Timestamp;

    #[ink(message)]
    fn proposal_proposer(&self, proposal_id: ProposalId) -> AccountId;

    #[ink(message)]
    fn voting_delay(&self) -> u64;

    #[ink(message)]
    fn voting_period(&self) -> u64;

    #[ink(message)]
    fn quorum(&self, time_point: Timestamp) -> u128;

    #[ink(message)]
    fn get_votes(&self, account: AccountId, time_point: Timestamp) -> u128;

    #[ink(message)]
    fn get_votes_with_params(&self, account: AccountId, time_point: Timestamp, params: Vec<u8>) -> u128;

    #[ink(message)]
    fn has_voted(&self, proposal_id: ProposalId, account: AccountId) -> bool;

    #[ink(message)]
    fn propose(&mut self, transactions: Vec<Transaction>, description: String) -> Result<ProposalId, GovernanceError>;

    #[ink(message)]
    fn execute(&mut self, proposal_id: ProposalId) -> Result<(), GovernanceError>;

    #[ink(message)]
    fn cancel(&mut self, proposal_id: ProposalId) -> Result<(), GovernanceError>;

    #[ink(message)]
    fn cast_vote(&mut self, proposal_id: ProposalId, support: u8) -> Result<Balance, GovernanceError>;

    #[ink(message)]
    fn cast_vote_with_reason(
        &mut self,
        proposal_id: ProposalId,
        support: u8,
        reason: String,
    ) -> Result<Balance, GovernanceError>;

    #[ink(message)]
    fn cast_vote_with_reason_and_params(
        &mut self,
        proposal_id: ProposalId,
        support: u8,
        reason: String,
        params: Vec<u8>,
    ) -> Result<Balance, GovernanceError>;

    #[ink(message)]
    fn cast_vote_with_signature(
        &mut self,
        proposal_id: ProposalId,
        support: u8,
        reason: String,
        signature: SignatureType,
    ) -> Result<Balance, GovernanceError>;

    #[ink(message)]
    fn cast_vote_with_signature_and_params(
        &mut self,
        proposal_id: ProposalId,
        support: u8,
        reason: String,
        signature: SignatureType,
        params: Vec<u8>,
    ) -> Result<Balance, GovernanceError>;
}
