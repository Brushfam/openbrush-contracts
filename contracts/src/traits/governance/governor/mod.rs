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
    governance::{
        HashType,
        ProposalId,
        ProposalState,
        Transaction,
        VoteType,
    },
    types::SignatureType,
};
use ink::prelude::vec::Vec;
use openbrush::traits::{
    AccountId,
    Balance,
    String,
    Timestamp,
};

/// Core of the governance system, designed to be extended though various modules.
///
/// This contract is abstract and requires several functions to be implemented in various modules:
///
/// - A counting module must implement `quorum`, `_quorum_reached`, `_vote_succeeded` and `_count_vote`}
/// - A voting module must implement `_get_votes`
/// - Additionally, `voting_period` must also be implemented
#[openbrush::trait_definition]
pub trait Governor {
    /// Hashing function used to (re)build the proposal id from the proposal details.
    #[ink(message)]
    fn hash_proposal(
        &self,
        transactions: Vec<Transaction>,
        description_hash: HashType,
    ) -> Result<HashType, GovernanceError>;

    /// Current state of a proposal, following Compound's convention
    #[ink(message)]
    fn state(&self, proposal_id: ProposalId) -> Result<ProposalState, GovernanceError>;

    /// Returns timestamp at which votes for a proposal starts
    #[ink(message)]
    fn proposal_snapshot(&self, proposal_id: ProposalId) -> Result<Timestamp, GovernanceError>;

    /// Returns timestamp at which votes for a proposal ends
    #[ink(message)]
    fn proposal_deadline(&self, proposal_id: ProposalId) -> Result<Timestamp, GovernanceError>;

    /// Returns the AccountId of the proposer of a proposal
    #[ink(message)]
    fn proposal_proposer(&self, proposal_id: ProposalId) -> Result<AccountId, GovernanceError>;

    /// Returns the number of votes already casted for a proposal by a given account
    #[ink(message)]
    fn get_votes_with_params(
        &mut self,
        account: AccountId,
        timestamp: Timestamp,
        params: Vec<u8>,
    ) -> Result<u128, GovernanceError>;

    /// Makes a proposal for a list of transactions to be executed.
    /// Returns the id of the proposal
    #[ink(message)]
    fn propose(&mut self, transactions: Vec<Transaction>, description: String) -> Result<ProposalId, GovernanceError>;

    /// Executes a proposal if it is in the `Succeeded` state.
    /// Returns the id of the executed proposal
    #[ink(message)]
    fn execute(
        &mut self,
        transactions: Vec<Transaction>,
        description_hash: HashType,
    ) -> Result<ProposalId, GovernanceError>;

    /// Cancels a proposal if it is in the `Pending` state.
    /// Returns the id of the cancelled proposal
    #[ink(message)]
    fn cancel(
        &mut self,
        transactions: Vec<Transaction>,
        description_hash: HashType,
    ) -> Result<ProposalId, GovernanceError>;

    /// Casts a vote for a proposal from a message sender.
    /// Returns the number of votes already casted for the proposal by the sender
    #[ink(message)]
    fn cast_vote(&mut self, proposal_id: ProposalId, support: VoteType) -> Result<Balance, GovernanceError>;

    /// Casts a vote with reason for a proposal from a message sender.
    /// Returns the number of votes already casted for the proposal by the sender
    #[ink(message)]
    fn cast_vote_with_reason(
        &mut self,
        proposal_id: ProposalId,
        support: VoteType,
        reason: String,
    ) -> Result<Balance, GovernanceError>;

    /// Casts a vote with reason and parameters for a proposal from a message sender.
    /// Returns the number of votes already casted for the proposal by the sender
    #[ink(message)]
    fn cast_vote_with_reason_and_params(
        &mut self,
        proposal_id: ProposalId,
        support: VoteType,
        reason: String,
        params: Vec<u8>,
    ) -> Result<Balance, GovernanceError>;

    /// Casts a vote with signature for a proposal from a message sender. Returns the number of votes already casted for the proposal by the sender
    #[ink(message)]
    fn cast_vote_with_signature(
        &mut self,
        proposal_id: ProposalId,
        support: VoteType,
        reason: String,
        signature: SignatureType,
    ) -> Result<Balance, GovernanceError>;

    /// Casts a vote with signature and parameters for a proposal from a message sender. Returns the number of votes already casted for the proposal by the sender
    #[ink(message)]
    fn cast_vote_with_signature_and_params(
        &mut self,
        proposal_id: ProposalId,
        support: VoteType,
        reason: String,
        signature: SignatureType,
        params: Vec<u8>,
    ) -> Result<Balance, GovernanceError>;

    /// Relays a transaction or function call to an arbitrary target. In cases where the governance executor
    /// is some contract other than the governor itself, like when using a timelock, this function can be invoked
    /// in a governance proposal to recover tokens or Ether that was sent to the governor contract by mistake.
    #[ink(message)]
    fn relay(&mut self, target: AccountId, transaction: Transaction) -> Result<(), GovernanceError>;
}

#[openbrush::wrapper]
pub type GovernorRef = dyn Governor;
