// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::{
    governance::extensions::governor_quorum,
    traits::governance::extensions::governor_quorum::*,
};

use crate::{
    governance::{
        extensions::{
            governor_quorum::{
                Data,
                QuorumEvents,
            },
            governor_votes,
        },
        governor,
        governor::{
            only_governance,
            TimestampProvider,
        },
    },
    traits::governance::utils::votes::*,
};
use openbrush::traits::{
    Storage,
    Timestamp,
};

/// Extension of `Governor` for voting weight extraction from an `PSP22Votes` token and a quorum expressed as a
/// fraction of the total supply.
pub trait QuorumImpl:
    Storage<governor::Data> + Storage<Data> + Storage<governor_votes::Data> + QuorumEvents + TimestampProvider
{
    /// Initializes the quorum numerator
    fn _init_quorum_numerator(&mut self, numerator: u128) -> Result<(), GovernanceError> {
        self._update_quorum_numerator(numerator)
    }

    /// Returns the current quorum numerator
    fn quorum_numerator(&self) -> u128 {
        let history = self
            .data::<governor_quorum::Data>()
            .quorum_numerator_history
            .get_or_default();

        let (exist, _, last_value) = history.latest_checkpoint();

        if exist {
            last_value
        } else {
            0
        }
    }

    /// Returns the quorum numerator at a given timestamp
    fn quorum_numerator_at(&self, time_point: Timestamp) -> u128 {
        let history = self.data::<Data>().quorum_numerator_history.get_or_default();

        let (exist, timestamp, value) = history.latest_checkpoint();

        if !exist {
            return self.quorum_numerator()
        }

        if timestamp <= time_point {
            return value
        }

        history.upper_lookup_recent(time_point).unwrap_or(0)
    }

    /// Returns the current quorum denominator. May be overridden by a derived contract.
    fn quorum_denominator(&self) -> u128 {
        100
    }

    /// Returns the quorum at a given timestamp
    fn quorum(&self, timestamp: Timestamp) -> Result<u128, GovernanceError> {
        let mut token = self
            .data::<governor_votes::Data>()
            .token
            .get()
            .ok_or(GovernanceError::TokenNotSet)?;

        let past_total_supply = VotesRef::get_past_total_supply(&mut token, timestamp)?;

        past_total_supply
            .checked_mul(self.quorum_numerator_at(timestamp))
            .ok_or(GovernanceError::Overflow)?
            .checked_div(self.quorum_denominator())
            .ok_or(GovernanceError::Overflow)
    }

    /// Updates the quorum numerator
    #[openbrush::modifiers(only_governance)]
    fn update_quorum_numerator(&mut self, numerator: u128) -> Result<(), GovernanceError> {
        self._update_quorum_numerator(numerator)
    }

    /// Updates the quorum numerator and adds a checkpoint to the history. Emits a `quorum_numerator_updated` event.
    fn _update_quorum_numerator(&mut self, new_quorum_numerator: u128) -> Result<(), GovernanceError> {
        let denominator = self.quorum_denominator();

        if new_quorum_numerator > denominator {
            return Err(GovernanceError::InvalidQuorumFraction)
        }

        let old_quorum_numerator = self.quorum_numerator();
        let mut history = self.data::<Data>().quorum_numerator_history.get_or_default();

        let timestamp = TimestampProvider::block_timestamp(self);
        history.push(timestamp, new_quorum_numerator.clone())?;

        self.data::<Data>().quorum_numerator_history.set(&history);

        self.emit_quorum_numerator_updated(old_quorum_numerator, new_quorum_numerator);

        Ok(())
    }
}
