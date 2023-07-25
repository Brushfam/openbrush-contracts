// Copyright (c) 2023 727.ventures
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

pub use crate::traits::{
    errors::{
        GovernorError,
    },
    types::{
        Id,
        Timestamp,
    }
};
use ink::prelude::{
    vec::Vec,
    string::String,
};
use openbrush::traits::{
    AccountId,
    Balance,
};

#[openbrush::wrapper]
pub type GovernorRef = dyn Governor;

pub enum ProposalState {
    Pending,
    Active,
    Canceled,
    Defeated,
    Succeeded,
    Queued,
    Expired,
    Executed
}

#[openbrush::trait_definition]
pub trait Governor {
    /// Returns the name.
    #[ink(message)]
    fn name(&self) -> String;

    /// Returns the current state of proposal with `proposal_id` id.
    #[ink(message)]
    fn state(&self, proposal_id: Id) -> Result<ProposalState, GovernorError>;

    /// Returns the number of votes required in order for a voter to become a proposer.
    #[ink(message)]
    fn proposal_threshold(&self) -> u128;

    /// Returns the time when the proposal with `proposal_id` id will start to receive votes.
    #[ink(message)]
    fn proposal_snapshot(&self, proposal_id: Id) -> Timestamp;

    /// Returns the time when the proposal with `proposal_id` id will end to receive votes.
    #[ink(message)]
    fn proposal_deadline(&self, proposal_id: Id) -> Timestamp;

    /// Returns the AccountId of the proposer of the proposal with `proposal_id` id.
    #[ink(message)]
    fn proposal_proposer(&self, proposal_id: Id) -> AccountId;

    /// Create a new proposal. Vote start after a delay specified by voting_delay() and lasts for a
    /// duration specified by voting_period().
    #[ink(message)]
    fn propose(&mut self,
               targets: Vec<AccountId>,
               values: Vec<Balance>,
               calldatas: Vec<Vec<u8>>,
               description: String
    ) -> Result<Id, GovernorError>;

    /// Execute a successful proposal. This requires the quorum to be reached, the vote to be successful, and the
    /// deadline to be reached.
    #[ink(message)]
    fn execute(&mut self,
               targets: Vec<AccountId>,
               values: Vec<Balance>,
               calldatas: Vec<Vec<u8>>,
               description_hash: Vec<u8>
    ) -> Result<Id, GovernorError>;

    /// Cancel a proposal. A proposal is cancellable by the proposer, but only while it is Pending state, i.e.
    /// before the vote starts.
    #[ink(message)]
    fn cancel(&mut self,
              targets: Vec<AccountId>,
              values: Vec<Balance>,
              calldatas: Vec<Vec<u8>>,
              description_hash: Vec<u8>
    ) -> Result<Id, GovernorError>;

    /// Returns the voting power of an `account` at a specific `timepoint`.
    #[ink(message)]
    fn get_votes(&self, account: AccountId, timepoint: Timestamp) -> u128;

    /// Returns the voting power of an `account` at a specific `timepoint` given additional encoded parameters.
    #[ink(message)]
    fn get_votes_with_params(
        &self,
        account: AccountId,
        timepoint: Timestamp,
        params: Vec<u8>
    ) -> u128;

    /// Cast a vote for a proposal.
    #[ink(message)]
    fn cast_vote(
        &self,
        proposal_id: Id,
        support: bool
    ) -> Result<Balance, GovernorError>;

    /// Cast a vote with a reason.
    #[ink(message)]
    fn cast_vote_with_reason(
        &self,
        proposal_id: Id,
        support: bool,
        reason: String
    ) -> Result<Balance, GovernorError>;

    /// Cast a vote with a reason and additional encoded parameters.
    #[ink(message)]
    fn cast_vote_with_reason_and_params(
        &self,
        proposal_id: Id,
        support: bool,
        reason: String,
        params: Vec<u8>
    ) -> Result<Balance, GovernorError>;

    /// Cast a vote using the voter's signature, including ecdsa signature support.
    #[ink(message)]
    fn cast_vote_by_sig(
        &self,
        proposal_id: Id,
        support: bool,
        voter: AccountId,
        signature: Vec<u8>
    ) -> Result<Balance, GovernorError>;

    /// Cast a vote with a reason and additional encoded parameters using the voter's signature,
    /// including acdsa signature support.
    #[ink(message)]
    fn cast_vote_with_reason_and_params_by_sig(
        &self,
        proposal_id: Id,
        support: bool,
        voter: AccountId,
        reason: String,
        params: Vec<u8>,
        signature: Vec<u8>
    ) -> Result<Balance, GovernorError>;


    /// Relays a transaction or function call to an arbitrary target. In cases where the governance executor
    /// is some contract other than the governor itself, like when using a timelock, this function can be invoked
    /// in a governance proposal to recover tokens or Ether that was sent to the governor contract by mistake.
    /// Note that if the executor is simply the governor itself, use of `relay` is redundant.
    #[ink(message)]
    fn relay(
        &mut self,
        target: AccountId,
        value: Balance,
        data: Vec<u8>
    ) -> Result<(), GovernorError>;

    /// Hash a proposal's elements (targets, values, signatures, calldatas, and description) into a single proposal hash.
    #[ink(message)]
    fn hash_proposal(
        &self,
        targets: Vec<AccountId>,
        values: Vec<Balance>,
        calldatas: Vec<Vec<u8>>,
        description_hash: Vec<u8>
    ) -> Vec<u8>;
}
