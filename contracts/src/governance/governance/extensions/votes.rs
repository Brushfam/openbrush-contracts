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


pub use crate::{
    governance::governance::*,
    traits::governance::{
        extensions::{
            counting::*,
            votes::*,
        },
        utils::votes::*,
        *,
    },
};
use openbrush::traits::{AccountId, Balance, StorageAsRef, Timestamp};
pub use governance::governance::{
    Internal as _,
    InternalImpl as _,
    GovernorImpl,
};
use openbrush::storage::Mapping;
use openbrush::traits::Storage;
use crate::utils::structs::checkpoints::Checkpoints;

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    delegation: Mapping<AccountId, AccountId>,
    delegate_checkpoints: Mapping<AccountId, Checkpoints>,
    total_checkpoints: Checkpoints,
}

pub trait GovernorVotesImpl: governor::Internal + Internal + Storage<Data> + GovernorImpl{
    fn get_votes(&self, account: AccountId) -> u128;

    fn get_past_votes(&self, account: AccountId, timestamp: Timestamp) -> Result<u128, VotesError>;

    fn get_past_total_supply(&self, timestamp: Timestamp) -> Result<u128, VotesError>;

    fn delegates(&self, account: AccountId) -> AccountId;

    fn delegate(&mut self, delegatee: AccountId);

    fn delegate_by_sig(
        &mut self,
        delegatee: AccountId,
        nonce: u128,
        expiry: u128,
        signature: Vec<u8>
    );

    fn clock(&self) -> u64;

    fn clock_mode(&self) -> Result<String, VotesError>;
}

pub trait Internal {
    fn _get_total_supply(&self) -> u128;

    fn _delegate(&mut self, delegator: AccountId, delegatee: AccountId);

    fn _trasfer_voting_units(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: Balance,
    );

    fn _move_delegate_votes(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: Balance,
    );

    fn _num_checkpoints(&self, account: AccountId) -> u128;

    fn _checkpoints(&self, account: AccountId, index: u128) -> Checkpoints;

    fn _push_checkpoints(
        &mut self,
        store: Checkpoints,
        op: fn(u128, u128) -> u128,
        delta: u128,
    ) -> (u128, u128);

    fn _add(a: u128, b: u128) -> u128;

    fn _sub(a: u128, b: u128) -> u128;

    fn _get_voting_units(&self, account: AccountId) -> u128;
}

pub trait InternalImpl: Internal + Storage<Data> + GovernorVotesImpl + GovernorImpl {
    fn _get_total_supply(&self) -> u128;

    fn _delegate(&mut self, delegator: AccountId, delegatee: AccountId);

    fn _trasfer_voting_units(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: Balance,
    );

    fn _move_delegate_votes(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: Balance,
    );

    fn _num_checkpoints(&self, account: AccountId) -> u128;

    fn _checkpoints(&self, account: AccountId, index: u128) -> Checkpoints;

    fn _push_checkpoints(
        &mut self,
        store: Checkpoints,
        op: fn(u128, u128) -> u128,
        delta: u128,
    ) -> (u128, u128);

    fn _add(a: u128, b: u128) -> u128 {
        a + b
    }

    fn _sub(a: u128, b: u128) -> u128 {
        a - b
    }

    fn _get_voting_units(&self, account: AccountId) -> u128;
}