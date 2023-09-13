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
    governance::{
        governor::TimestampProvider,
        utils::votes::{
            Data,
            VotesEvents,
        },
    },
    traits::errors::{
        GovernanceError,
        MathError,
    },
};
use openbrush::{
    traits::{
        AccountId,
        Balance,
        Storage,
    },
    utils::checkpoints::{
        Checkpoint,
        Checkpoints,
        CheckpointsError,
    },
};

pub trait VotesInternal: Storage<Data> + VotesEvents + TimestampProvider {
    /// Returns the total number of votes.
    fn _get_total_supply(&self) -> Balance {
        self.data::<Data>().total_checkpoints.get_or_default().latest()
    }

    /// Returns the address delegated to by `delegator`.
    fn _delegates(&self, delegator: &Option<AccountId>) -> Option<AccountId> {
        self.data::<Data>().delegation.get(&delegator)
    }

    /// Delegate votes from `delegator` to `delegatee`.
    fn _delegate(
        &mut self,
        delegator: &Option<AccountId>,
        delegatee: &Option<AccountId>,
    ) -> Result<(), GovernanceError> {
        let old_delegate = self._delegates(&delegator);

        self.data::<Data>()
            .delegation
            .insert(&delegator, &delegatee.ok_or(GovernanceError::InvalidInput)?);

        self.emit_delegate_changed_event(&delegator, &old_delegate, delegatee);

        self._move_delegate_votes(
            &old_delegate,
            delegatee,
            self._get_voting_units(&delegator.ok_or(GovernanceError::InvalidInput)?),
        )
    }

    /// Transfers `amount` voting units from `from` to `to`.
    fn _transfer_voting_units(
        &mut self,
        from: &Option<AccountId>,
        to: &Option<AccountId>,
        amount: Balance,
    ) -> Result<(), GovernanceError> {
        let mut store = self.data::<Data>().total_checkpoints.get_or_default();
        if from.is_none() {
            self._push(&mut store, Self::_add, amount)?;
        }
        if to.is_none() {
            self._push(&mut store, Self::_sub, amount)?;
        }
        self._move_delegate_votes(&self._delegates(from), &self._delegates(to), amount)
    }

    /// Moves voting units from `from` to `to`.
    fn _move_delegate_votes(
        &mut self,
        from: &Option<AccountId>,
        to: &Option<AccountId>,
        amount: Balance,
    ) -> Result<(), GovernanceError> {
        if from != to && amount > 0 {
            if let Some(from_addr) = from {
                let mut store = self
                    .data::<Data>()
                    .delegate_checkpoints
                    .get(&from_addr)
                    .unwrap_or_default();

                let (old_value, new_value) = self._push(&mut store, Self::_sub, amount)?;
                self.data::<Data>().delegate_checkpoints.insert(&from_addr, &store);
                self.emit_delegate_votes_changed_event(&from_addr, old_value, new_value);
            }
            if let Some(to_addr) = to {
                let mut store = self
                    .data::<Data>()
                    .delegate_checkpoints
                    .get(&to_addr)
                    .unwrap_or_default();

                let (old_value, new_value) = self._push(&mut store, Self::_add, amount)?;
                self.data::<Data>().delegate_checkpoints.insert(&to_addr, &store);
                self.emit_delegate_votes_changed_event(&to_addr, old_value, new_value);
            }
        }
        Ok(())
    }

    /// Returns number of checkpoints for `account`.
    fn _num_checkpoints(&self, account: &AccountId) -> u32 {
        self.data::<Data>()
            .delegate_checkpoints
            .get(&account)
            .unwrap_or_default()
            .len() as u32
    }

    /// Returns the checkpoint for `account` at the given `pos`.
    fn _checkpoints(&self, account: &AccountId, pos: u32) -> Result<Checkpoint, GovernanceError> {
        self.data::<Data>()
            .delegate_checkpoints
            .get(&account)
            .unwrap_or_default()
            .at(pos as usize)
            .ok_or(GovernanceError::IndexOutOfRange)
            .cloned()
    }

    /// Creates a new checkpoint for `account` and returns its `old_value` and `new_value`.
    fn _push(
        &mut self,
        store: &mut Checkpoints,
        op: fn(u128, u128) -> Result<u128, GovernanceError>,
        delta: Balance,
    ) -> Result<(u128, u128), GovernanceError> {
        let (old_value, new_value) = store
            .push(TimestampProvider::block_timestamp(self), op(store.latest(), delta)?)
            .map_err(|err| <CheckpointsError as Into<GovernanceError>>::into(err))?;
        Ok((old_value, new_value))
    }

    fn _add(a: u128, b: u128) -> Result<u128, GovernanceError> {
        Ok(a.checked_add(b).ok_or(MathError::Overflow)?)
    }

    fn _sub(a: u128, b: u128) -> Result<u128, GovernanceError> {
        Ok(a.checked_sub(b).ok_or(MathError::Overflow)?)
    }

    /// Returns the number of voting units owned by `account`.
    fn _get_voting_units(&self, account: &AccountId) -> Balance;
}
