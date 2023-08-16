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

#[openbrush::trait_definition]
pub trait Governor {
    #[ink(message)]
    fn hash_proposal(
        &self,
        transactions: Vec<Transaction>,
        description_hash: HashType,
    ) -> Result<HashType, GovernanceError>;

    #[ink(message)]
    fn state(&self, proposal_id: ProposalId) -> Result<ProposalState, GovernanceError>;

    #[ink(message)]
    fn proposal_snapshot(&self, proposal_id: ProposalId) -> Result<Timestamp, GovernanceError>;

    #[ink(message)]
    fn proposal_deadline(&self, proposal_id: ProposalId) -> Result<Timestamp, GovernanceError>;

    #[ink(message)]
    fn proposal_proposer(&self, proposal_id: ProposalId) -> Result<AccountId, GovernanceError>;

    #[ink(message)]
    fn get_votes_with_params(
        &mut self,
        account: AccountId,
        time_point: Timestamp,
        params: Vec<u8>,
    ) -> Result<u128, GovernanceError>;

    #[ink(message)]
    fn propose(&mut self, transactions: Vec<Transaction>, description: String) -> Result<ProposalId, GovernanceError>;

    #[ink(message)]
    fn execute(
        &mut self,
        transactions: Vec<Transaction>,
        description_hash: HashType,
    ) -> Result<ProposalId, GovernanceError>;

    #[ink(message)]
    fn cancel(
        &mut self,
        transactions: Vec<Transaction>,
        description_hash: HashType,
    ) -> Result<ProposalId, GovernanceError>;

    #[ink(message)]
    fn cast_vote(&mut self, proposal_id: ProposalId, support: VoteType) -> Result<Balance, GovernanceError>;

    #[ink(message)]
    fn cast_vote_with_reason(
        &mut self,
        proposal_id: ProposalId,
        support: VoteType,
        reason: String,
    ) -> Result<Balance, GovernanceError>;

    #[ink(message)]
    fn cast_vote_with_reason_and_params(
        &mut self,
        proposal_id: ProposalId,
        support: VoteType,
        reason: String,
        params: Vec<u8>,
    ) -> Result<Balance, GovernanceError>;

    #[ink(message)]
    fn cast_vote_with_signature(
        &mut self,
        proposal_id: ProposalId,
        support: VoteType,
        reason: String,
        signature: SignatureType,
    ) -> Result<Balance, GovernanceError>;

    #[ink(message)]
    fn cast_vote_with_signature_and_params(
        &mut self,
        proposal_id: ProposalId,
        support: VoteType,
        reason: String,
        signature: SignatureType,
        params: Vec<u8>,
    ) -> Result<Balance, GovernanceError>;

    #[ink(message)]
    fn relay(&mut self, target: AccountId, transaction: Transaction) -> Result<(), GovernanceError>;
}

#[openbrush::wrapper]
pub type GovernorRef = dyn Governor;
