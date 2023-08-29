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

use crate::traits::{
    errors::GovernanceError,
    governance::utils::votes::*,
};
use openbrush::traits::AccountId;
pub use openbrush::utils::checkpoints::Checkpoint;

/// Extension of ERC20 to support Compound-like voting and delegation.
///
/// This extension keeps a history (checkpoints) of each account's vote power. Vote power can be delegated either
/// by calling the `delegate` function directly, or by providing a signature to be used with `delegate_by_sig`. Voting
/// power can be queried through the public accessors `get_votes` and `get_past_votes`.
///
/// By default, token balance does not account for voting power. This makes transfers cheaper. The downside is that it
/// requires users to delegate to themselves in order to activate checkpoints and have their voting power tracked.
#[openbrush::trait_definition]
pub trait PSP22Votes {
    /// Get number of checkpoints for `account`.
    #[ink(message)]
    fn num_checkpoints(&self, account: AccountId) -> u32;

    /// Get the `pos`-th checkpoint for `account`.
    #[ink(message)]
    fn checkpoints(&self, account: AccountId, pos: u32) -> Result<Checkpoint, GovernanceError>;
}

#[openbrush::wrapper]
pub type PSP22VotesWrapper = dyn PSP22Votes + Votes;
