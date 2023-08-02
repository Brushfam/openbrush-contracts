use crate::{
    governance::extensions::governor_votes::{
        Data,
        VotesEvents,
        VotesInternal,
    },
    traits::{
        errors::GovernanceError,
        types::SignatureType,
    },
    utils::{
        crypto,
        nonces::NoncesImpl,
    },
};
use openbrush::traits::{
    AccountId,
    Balance,
    Storage,
    Timestamp,
};
use scale::Encode;

pub trait GovernorVotesImpl: Storage<Data> + VotesInternal + NoncesImpl + VotesEvents {
    fn get_votes(&self, account: AccountId) -> Result<Balance, GovernanceError> {
        Ok(self
            .data::<Data>()
            .delegate_checkpoints
            .get(&account)
            .ok_or(GovernanceError::AccountNotFound)?
            .latest())
    }

    fn get_past_votes(&self, account: AccountId, timestamp: Timestamp) -> Result<Balance, GovernanceError> {
        let current_block_timestamp = Self::env().block_timestamp();
        if timestamp > current_block_timestamp {
            return Err(GovernanceError::FutureLookup(timestamp, current_block_timestamp))
        }
        match self
            .data::<Data>()
            .delegate_checkpoints
            .get(&account)
            .ok_or(GovernanceError::AccountNotFound)?
            .upper_lookup_recent(timestamp)
        {
            Some(value) => Ok(value),
            None => Ok(0),
        }
    }

    fn get_past_total_supply(&self, timestamp: Timestamp) -> Result<Balance, GovernanceError> {
        let current_block_timestamp = Self::env().block_timestamp();
        if timestamp > current_block_timestamp {
            return Err(GovernanceError::FutureLookup(timestamp, current_block_timestamp))
        }

        let checkpoints = &self.data::<Data>().total_checkpoints.get_or_default();
        match checkpoints.upper_lookup_recent(timestamp) {
            Some(value) => Ok(value),
            None => Ok(0),
        }
    }

    fn delegates(&mut self, delegator: AccountId) -> AccountId {
        self._delegates(&delegator)
    }

    fn delegate(&mut self, delegatee: AccountId) -> Result<(), GovernanceError> {
        let account = Self::env().caller();
        self._delegate(&account, &delegatee)
    }

    fn delegate_by_signature(
        &mut self,
        signer: AccountId,
        delegatee: AccountId,
        nonce: u128,
        expiry: Timestamp,
        signature: SignatureType,
    ) -> Result<(), GovernanceError> {
        if Self::env().block_timestamp() > expiry {
            return Err(GovernanceError::ExpiredSignature(expiry))
        }
        let message_hash = crypto::hash_message(Encode::encode(&(&delegatee, &nonce, &expiry)).as_slice())?;
        let verify_result = crypto::verify_signature(&message_hash, &signer, &signature)?;
        if !verify_result {
            return Err(GovernanceError::InvalidSignature(signer))
        } else {
            self._use_checked_nonce(&signer, nonce)?;
            self._delegate(&signer, &delegatee)
        }
    }
}
