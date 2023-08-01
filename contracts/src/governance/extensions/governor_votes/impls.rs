use scale::Encode;
use crate::utils::crypto::SignatureType;
use openbrush::traits::{Storage, Timestamp};
use openbrush::traits::{AccountId, Balance};
use crate::governance::extensions::governor_votes::{Data, VotesEvents, VotesInternal};
use crate::traits::errors::GovernanceError;
use crate::utils::crypto;
use crate::utils::nonces::NoncesImpl;

pub trait GovernorVotesImpl: Storage<Data> + VotesInternal + NoncesImpl + VotesEvents{
    fn clock(&self) -> u64 {
        Self::env().block_timestamp()
    }

    fn get_votes(&self, account: AccountId) -> Balance {
        self.data::<Data>().delegate_checkpoints.get(&account).unwrap_or_default().latest()
    }

    fn get_past_votes(&self, account: AccountId, timestamp: Timestamp) -> Result<Balance, GovernanceError> {
        let current_block_timestamp = self.clock();
        if timestamp > current_block_timestamp {
            return Err(GovernanceError::FutureLookup(timestamp, current_block_timestamp))
        }
        match self.data::<Data>().delegate_checkpoints.get(&account).unwrap_or_default().upper_lookup_recent(timestamp as u32) {
            Some(value) => Ok(value),
            None => Ok(0)
        }
    }

    fn get_past_total_supply(&self, timestamp: Timestamp) -> Result<Balance, GovernanceError> {
        let current_block_timestamp = self.clock();
        if timestamp > current_block_timestamp {
            return Err(GovernanceError::FutureLookup(timestamp, current_block_timestamp))
        }

        let checkpoints = &self.data::<Data>().total_checkpoints.get_or_default();
        match checkpoints.upper_lookup_recent(timestamp as u32) {
            Some(value) => Ok(value),
            None => Ok(0)
        }
    }

    fn delegates(&mut self, delegator: AccountId) -> AccountId {
        self.data::<Data>().delegation.get(&delegator).unwrap_or(AccountId::from([0x0; 32]))
    }

    fn delegate(&mut self, delegatee: AccountId) {
        let account = Self::env().caller();
        self._delegate(&account, &delegatee)
    }

    fn delegate_by_signature(&mut self, signer: AccountId, delegatee: AccountId, nonce: u128, expiry: Timestamp, signature: SignatureType) -> Result<(), GovernanceError> {
        if self.clock() > expiry {
            return Err(GovernanceError::ExpiredSignature(expiry));
        }
        let message_hash = crypto::hash_message(
            Encode::encode(&(&delegatee, &nonce, &expiry)).as_slice()
        )?;
        let verify_result = crypto::verify_signature(
            &message_hash,
            &signer,
            &signature
        )?;
        if !verify_result {
            return Err(GovernanceError::InvalidSignature(signer));
        } else {
            self._use_checked_nonce(&signer, nonce)?;
            self._delegate(&signer, &delegatee);
            Ok(())
        }
    }
}
