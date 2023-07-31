use openbrush::traits::{String, Timestamp};
use crate::traits::governance::ProposalId;
use openbrush::traits::{AccountId, Balance};
use crate::utils::crypto::SignatureType;



#[openbrush::trait_definition]
pub trait GovernorVotes {
    #[ink(message)]
    fn clock(&self) -> u64;

    #[ink(message)]
    fn get_votes(&self, account: AccountId) -> Balance;

    #[ink(message)]
    fn get_past_votes(&self, account: AccountId, timestamp: Timestamp) -> Balance;

    #[ink(message)]
    fn get_past_total_votes(&self, timestamp: Timestamp) -> Balance;

    #[ink(message)]
    fn delegates(&mut self, delegator: AccountId) -> AccountId;

    #[ink(message)]
    fn delegate(&mut self, delegatee: AccountId);

    #[ink(message)]
    fn delegate_by_signature(
        &mut self,
        delegatee: AccountId,
        nonce: u128,
        expiry: u128,
        signature: SignatureType,
    );
}
