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

use scale::Encode;
use crate::utils::structs::checkpoints::{Checkpoint, Checkpoints};
pub use crate::{
    governance::*,
    traits::governance::{
        extensions::{counting::*, votes::*},
        *,
    },
};
pub use crate::governance::{GovernorImpl, Internal as _, InternalImpl as _};
use openbrush::storage::Mapping;
use openbrush::traits::{BlockNumber, Storage};
use openbrush::traits::{AccountId, Balance, StorageAsRef, Timestamp};
use crate::traits::errors::CheckpointsError;
use crate::utils::crypto;
use openbrush::traits::String;

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    delegation: Mapping<AccountId, AccountId>,
    delegate_checkpoints: Mapping<AccountId, Checkpoints>,
    total_checkpoints: Checkpoints,
}

pub trait GovernorVotesImpl: governor::Internal + Internal + Storage<Data> + GovernorImpl {
    ///Returns the current amount of votes that `account` has.
    fn get_votes(&self, account: AccountId) -> u128{
        self.data().delegate_checkpoints.get(&account).unwrap_or_default().latest()
    }

    ///Returns the amount of votes that `account` had at a specific moment in the past. If the `clock()` is
    /// configured to use block numbers, this will return the value at the end of the corresponding block.
    ///
    /// Requirements:
    ///
    /// - `timepoint` must be in the past. If operating using block numbers, the block must be already mined.
    fn get_past_votes(&self, account: AccountId, timepoint: BlockNumber) -> Result<Option<u128>, VotesError> {
        let current_timepoint = self.clock();

        if timepoint >= current_timepoint {
            return Err(VotesError::FutureLookup(timepoint.clone(), current_timepoint));
        }

        Ok(self.data().delegate_checkpoints.get(&account).unwrap_or_default().upper_lookup_recent(timepoint))
    }

    ///Returns the total supply of votes available at a specific moment in the past. If the `clock()` is
    /// configured to use block numbers, this will return the value at the end of the corresponding block.
    ///
    /// NOTE: This value is the sum of all available votes, which is not necessarily the sum of all delegated votes.
    /// Votes that have not been delegated are still part of total supply, even though they would not participate in a
    /// vote.
    ///
    /// Requirements:
    ///
    /// - `timepoint` must be in the past. If operating using block numbers, the block must be already mined.
    fn get_past_total_supply(&self, timestamp: BlockNumber) -> Result<u128, VotesError> {
        let current_timepoint = self.clock();

        if timestamp >= current_timepoint {
            return Err(VotesError::FutureLookup(timestamp.clone(), current_timepoint));
        }

        Ok(self.data().total_checkpoints.upper_lookup_recent(timestamp).unwrap_or_default())
    }

    ///Returns the delegate that `account` has chosen.
    fn delegates(&self, account: &AccountId) -> AccountId {
        self.data().delegation.get(&account).unwrap_or_default()
    }

    ///Delegates votes from the sender to `delegatee`.
    fn delegate(&mut self, delegatee: AccountId) {
        let account = Self::env().caller();
        self._delegate(account, delegatee);
    }

    ///Delegates votes from signer to `delegatee`.
    fn delegate_by_sig(
        &mut self,
        signer: AccountId,
        delegatee: AccountId,
        nonce: u128,
        expiry: u128,
        signature: &[u8; 65]
    ) -> Result<(), VotesError>{
        let message_hash = crypto::hash_message(
            Encode::encode(&(&delegatee, &nonce, &expiry)).as_slice(),
        )?;

        let verify = crypto::verify_signature(
            &message_hash,
            &signer,
            signature,
        )?;
        self._use_checked_nonce(&signer, nonce)?;
        self._delegate(signer, delegatee);
        Ok(())
    }

    fn clock(&self) -> BlockNumber {
        Self::env().block_number()
    }

    fn clock_mode(&self) -> Result<String, VotesError> {
        if self.clock() != Self::env().block_number() as u64 {
            return Err(VotesError::InconsistentClock);
        }
        Ok("mode=blocknumber&from=default".to_string())
    }
}

pub trait Internal {
    fn _get_total_supply(&self) -> u128;

    fn _delegate(&mut self, account: AccountId, delegatee: AccountId);

    fn _trasfer_voting_units(&mut self, from: AccountId, to: AccountId, amount: Balance);

    fn _move_delegate_votes(&mut self, from: AccountId, to: AccountId, amount: Balance);

    fn _num_checkpoints(&self, account: AccountId) -> usize;

    fn _checkpoints(&self, account: AccountId, index: u128) -> Checkpoints;

    fn _push(&mut self, store: &mut Checkpoints, op: fn(u128, u128) -> u128, delta: u128) -> (u128, u128);

    fn _add(a: u128, b: u128) -> u128;

    fn _sub(a: u128, b: u128) -> u128;

    fn _get_voting_units(&self, account: AccountId) -> u128;
}

pub trait InternalImpl: Internal + Storage<Data> + GovernorVotesImpl + GovernorImpl {
    fn _get_total_supply(&self) -> u128 {
        self.data().total_checkpoints.latest()
    }

    fn _delegate(&mut self, account: AccountId, delegatee: AccountId) {
        let mut old_delegatee = self.delegates(&account);
        self.data().delegation.insert(&account, &delegatee);

        self._emit_delegate_changed_event(account, old_delegatee, delegatee);

        self._move_delegate_votes(old_delegatee, delegatee, self._get_voting_units(account));
    }

    fn _trasfer_voting_units(&mut self, from: AccountId, to: AccountId, amount: Balance) {
        if from == AccountId::from([0x00; 32]) {
            self._push(
                &mut self.data().total_checkpoints,
                self._add,
                amount,
            );
        }
        if to == AccountId::from([0x00; 32]) {
            self._push(
                &mut self.data().total_checkpoints,
                self._sub,
                amount,
            );
        }
        self._move_delegate_votes(self.delegates(&from), self.delegates(& to), amount);
    }

    fn _move_delegate_votes(&mut self, from: AccountId, to: AccountId, amount: Balance) {
        if from != to && amount > 0 {
            if from != AccountId::from([0x00; 32]) {
                let (old_value, new_value) = self._push(
                    &mut self.data().delegate_checkpoints.get(&from).unwrap_or_default(),
                    self._sub,
                    amount,
                );
                self._emit_delegate_votes_changed_event(from, old_value, new_value);
            }
            if to != AccountId::from([0x00; 32]) {
                let (old_value, new_value) = self._push(
                    &mut self.data().delegate_checkpoints.get(&to).unwrap_or_default(),
                    self._add,
                    amount,
                );
                self._emit_delegate_votes_changed_event(to, old_value, new_value);
            }
        }
    }

    fn _num_checkpoints(&self, account: AccountId) -> usize {
        self.data().delegate_checkpoints.get(&account).unwrap_or_default().len()
    }

    fn _checkpoints(&self, account: AccountId, index: usize) -> Option<&Checkpoint> {
        self.data().delegate_checkpoints.get(&account).unwrap_or_default().at(index)
    }

    fn _push(&mut self, store: &mut Checkpoints, op: fn(u128, u128) -> u128, delta: u128) -> Result<(u128, u128), CheckpointsError> {
        store.push(self.clock(), op(store.latest(), delta))
    }

    fn _add(a: u128, b: u128) -> u128 {
        a + b
    }

    fn _sub(a: u128, b: u128) -> u128 {
        a - b
    }

    fn _get_voting_units(&self, account: AccountId) -> u128;
}
