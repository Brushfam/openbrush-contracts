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

pub use crate::traits::errors::{GovernorError, VotesError};
use openbrush::traits::{AccountId, Timestamp};

#[openbrush::wrapper]
pub type GovernorVotesRef = dyn GovernorVotes;

#[openbrush::trait_definition]
pub trait GovernorVotes {
    #[ink(message)]
    fn get_votes(&self, account: AccountId) -> u128;

    #[ink(message)]
    fn get_past_votes(&self, account: AccountId, timestamp: Timestamp) -> u128;

    #[ink(message)]
    fn get_past_total_supply(&self, timestamp: Timestamp) -> u128;

    #[ink(message)]
    fn delegates(&self, account: AccountId) -> AccountId;

    #[ink(message)]
    fn delegate(&mut self, delegatee: AccountId);

    #[ink(message)]
    fn delegate_by_sig(&mut self, delegatee: AccountId, nonce: u128, expiry: u128, signature: Vec<u8>);

    #[ink(message)]
    fn clock(&self) -> u64;

    #[ink(message)]
    fn clock_mode(&self) -> String;
}
