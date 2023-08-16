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
    governance::ProposalId,
};
use openbrush::traits::{
    AccountId,
    Balance,
    String,
};

///Extension of {Governor} for simple, 3 options, vote counting.
#[openbrush::trait_definition]
pub trait GovernorCounting {
    ///Returns the current counting mode
    #[ink(message)]
    fn counting_mode(&self) -> String;

    ///Returns `true` if the account has voted for the proposal, `false` otherwise
    #[ink(message)]
    fn has_voted(&self, proposal_id: ProposalId, account: AccountId) -> bool;

    ///Returns the tuple (for, against, abstain) votes for a proposal, where `for` is the total
    ///number of votes for the proposal, `against` is the total number of votes against the
    ///proposal, and `abstain` is the total number of abstained votes.
    #[ink(message)]
    fn proposal_votes(&self, proposal_id: ProposalId) -> Result<(Balance, Balance, Balance), GovernanceError>;
}

#[openbrush::wrapper]
pub type GovernorCountingRef = dyn GovernorCounting;
