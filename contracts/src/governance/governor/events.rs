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

pub use crate::traits::governance::{ProposalId, Transaction, VoteType};
pub use ink::prelude::vec::Vec;
pub use openbrush::traits::{AccountId, Balance, String, Timestamp};

pub trait GovernorEvents {
    /// Emitted when a proposal is created
    fn emit_proposal_created(
        &self,
        _proposal_id: ProposalId,
        _proposer: AccountId,
        _transactions: Vec<Transaction>,
        _vote_start: Timestamp,
        _vote_end: Timestamp,
        _description: String,
    ) {
    }

    /// Emitted when a proposal is canceled
    fn emit_proposal_canceled(&self, _proposal_id: ProposalId) {}

    /// Emitted when a proposal is executed
    fn emit_proposal_executed(&self, _proposal_id: ProposalId) {}

    /// Emitted when the vote is casted
    fn emit_vote_cast(
        &self,
        _proposal_id: ProposalId,
        _voter: AccountId,
        _support: VoteType,
        _weight: Balance,
        _reason: String,
    ) {
    }

    /// Emitted when the vote is casted with params
    fn emit_vote_cast_with_params(
        &self,
        _proposal_id: ProposalId,
        _voter: AccountId,
        _support: VoteType,
        _weight: Balance,
        _reason: String,
        _params: Vec<u8>,
    ) {
    }
}
