// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::traits::{
    errors::GovernanceError,
    governance::{
        HashType,
        ProposalId,
        ProposalState,
        Transaction,
        VoteType,
    },
    types::Signature,
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
    fn cast_vote(
        &mut self,
        proposal_id: ProposalId,
        support: VoteType,
        reason: Option<String>,
        params: Option<Vec<u8>>,
    ) -> Result<Balance, GovernanceError>;

    /// Casts a vote with signature for a proposal from a message sender. Returns the number of votes already casted for the proposal by the sender
    #[ink(message)]
    fn cast_vote_with_signature(
        &mut self,
        proposal_id: ProposalId,
        support: VoteType,
        reason: String,
        signature: Signature,
    ) -> Result<Balance, GovernanceError>;

    /// Casts a vote with signature and parameters for a proposal from a message sender. Returns the number of votes already casted for the proposal by the sender
    #[ink(message)]
    fn cast_vote_with_signature_and_params(
        &mut self,
        proposal_id: ProposalId,
        support: VoteType,
        reason: String,
        signature: Signature,
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
