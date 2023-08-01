use crate::utils::crypto::SignatureType;
use openbrush::traits::Timestamp;
use openbrush::traits::{AccountId, Balance};
use crate::traits::governance::GovernanceError;

#[openbrush::trait_definition]
pub trait GovernorVotes {
    #[ink(message)]
    fn clock(&self) -> u64;

    #[ink(message)]
    fn get_votes(&self, account: AccountId) -> Balance;

    #[ink(message)]
    fn get_past_votes(&self, account: AccountId, timestamp: Timestamp) -> Result<Balance, GovernanceError>;

    #[ink(message)]
    fn get_past_total_supply(&self, timestamp: Timestamp) -> Result<Balance, GovernanceError>;

    #[ink(message)]
    fn delegates(&mut self, delegator: AccountId) -> AccountId;

    #[ink(message)]
    fn delegate(&mut self, delegatee: AccountId);

    #[ink(message)]
    fn delegate_by_signature(&mut self, delegatee: AccountId, nonce: u128, expiry: Timestamp, signature: SignatureType) -> Result<(), GovernanceError>;
}
