// Copyright (c) 2023 Brushfam
// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use crate::{
    extensions::{
        governor_quorum::{
            Data,
            QuorumEvents,
        },
        governor_votes,
    },
    governor::TimestampProvider,
    traits::{
        errors::GovernanceError,
        governance::utils::VotesRef,
    },
};
use openbrush::traits::{
    Storage,
    Timestamp,
};

pub trait QuorumImpl: Storage<Data> + Storage<governor_votes::Data> + QuorumEvents + TimestampProvider {
    /// Constructor
    fn _init_quorum_numerator(&mut self, numerator: u128) -> Result<(), GovernanceError> {
        self._update_quorum_numerator(numerator)
    }

    fn quorum_numerator(&self) -> u128 {
        let history = self.data::<Data>().quorum_numerator_history.get_or_default();

        let (exist, _, last_value) = history.latest_checkpoint();

        if !exist {
            0
        } else {
            last_value
        }
    }

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

    /// may be overriden by the contract
    fn quorum_denominator(&self) -> u128 {
        100
    }

    fn quorum(&self, time_point: Timestamp) -> Result<u128, GovernanceError> {
        let mut token = self
            .data::<governor_votes::Data>()
            .token
            .get()
            .ok_or(GovernanceError::TokenNotSet)?;

        let past_total_supply = VotesRef::get_past_total_supply(&mut token, time_point)?;

        past_total_supply
            .checked_mul(self.quorum_numerator_at(time_point))
            .ok_or(GovernanceError::Overflow)?
            .checked_div(self.quorum_denominator())
            .ok_or(GovernanceError::Overflow)
    }

    // TODO: [only-governance]
    fn update_quorum_numerator(&mut self, numerator: u128) -> Result<(), GovernanceError> {
        self._update_quorum_numerator(numerator)
    }

    fn _update_quorum_numerator(&mut self, new_quorum_numerator: u128) -> Result<(), GovernanceError> {
        let denominator = self.quorum_denominator();

        if new_quorum_numerator > denominator {
            return Err(GovernanceError::InvalidQuorumFraction(
                new_quorum_numerator,
                denominator,
            ))
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
