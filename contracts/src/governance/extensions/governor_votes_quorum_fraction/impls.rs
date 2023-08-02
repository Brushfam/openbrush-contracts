use ink::storage::traits::StorageKey;
use crate::{
    governance::extensions::governor_votes_quorum_fraction::{
        Data,
        VotesQuorumFractionEvents,
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
use openbrush::traits::{AccountId, Balance, Storage, StorageAsRef, Timestamp};
use scale::Encode;
use crate::extensions::governor_votes::GovernorVotesImpl;

pub trait GovernorVotesQuorumFractionImpl: Storage<Data> + NoncesImpl + VotesQuorumFractionEvents + GovernorVotesImpl{
    fn quorum_numerator(&self) -> u128 {
        self.data().quorum_numerator_history.get().latest()
    }

    fn quorum_numerator_by_timestamp(&self, timestamp: Timestamp) -> u128 {
        let len = self.data().quorum_numerator_history.get().len();
        if len == 0 {
            return 0;
        }
        let latest = self.data().quorum_numerator_history.get().checkpoints[len - 1];
        if latest.key <= timestamp {
            return latest.value;
        }
        self.data().quorum_numerator_history.get().upper_lookup_recent(timestamp)
    }

    fn quorum_denominator(&self) -> u128 {
        100
    }

    fn quorum(&self, timestamp: Timestamp) -> u128 {
        self.get_past_total_supply(timestamp) * self.quorum_numerator_by_timestamp(timestamp) / self.quorum_denominator()
    }

    fn update_quorum_numerator(&mut self, new_quorum_numerator: u128) -> Result<(), GovernanceError> {
        let denominator = self.quorum_denominator();
        if new_quorum_numerator > denominator {
            return Err(GovernanceError::InvalidQuorumFraction(new_quorum_numerator, denominator))
        }

        let old_quorum_numerator = self.quorum_numerator();
        self.data().quorum_numerator_history.get().push(Self::env().block_timestamp(), new_quorum_numerator);
        self.emit_quorum_numerator_updated_event(old_quorum_numerator, new_quorum_numerator);
        Ok(())
    }
}
