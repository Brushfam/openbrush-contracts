use crate::utils::crypto::SignatureType;
use openbrush::traits::{Storage, Timestamp};
use openbrush::traits::{AccountId, Balance};
use crate::governance::extensions::governor_votes::{Data, VotesEvents, VotesInternal};
use crate::utils::nonces::Nonces;

pub trait GovernorVotesImpl: Storage<Data> + VotesInternal + Nonces + VotesEvents{
    fn clock(&self) -> u64;

    fn get_votes(&self, account: AccountId) -> Balance;

    fn get_past_votes(&self, account: AccountId, timestamp: Timestamp) -> Balance;

    fn get_past_total_votes(&self, timestamp: Timestamp) -> Balance;

    fn delegates(&mut self, delegator: AccountId) -> AccountId;

    fn delegate(&mut self, delegatee: AccountId);

    fn delegate_by_signature(&mut self, delegatee: AccountId, nonce: u128, expiry: u128, signature: SignatureType);
}
