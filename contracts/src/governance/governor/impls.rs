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
    extensions::{
        governor_settings::{
            GovernorSettingsImpl,
            GovernorSettingsInternal,
        },
        governor_votes::GovernorVotesInternal,
    },
    governance::governor::{
        Data,
        GovernorEvents,
        GovernorInternal,
    },
    governor::{
        GovernorStorageGetters,
        TimestampProvider,
    },
    nonces::NoncesImpl,
    traits::{
        errors::governance::GovernanceError,
        governance::{
            ExecutionStatus,
            HashType,
            ProposalCore,
            ProposalId,
            ProposalState,
            Transaction,
            VoteType,
        },
        types::SignatureType,
    },
    utils::crypto,
};
use ink::{
    env::{
        call::{
            build_call,
            ExecutionInput,
        },
        DefaultEnvironment,
    },
    prelude::vec::Vec,
};
use openbrush::{
    modifiers,
    traits::{
        AccountId,
        Balance,
        Storage,
        String,
        Timestamp,
    },
};
use scale::Encode;

#[openbrush::modifier_definition]
pub fn only_governance<T, F, R, E>(instance: &mut T, body: F) -> Result<R, E>
where
    T: GovernorInternal + Storage<Data>,
    F: FnOnce(&mut T) -> Result<R, E>,
    E: From<GovernanceError>,
{
    if T::env().caller() != instance._executor() {
        return Err(GovernanceError::OnlyExecutor(T::env().caller()).into())
    }

    if T::env().account_id() != instance._executor() {
        let transaction = ink::env::decode_input::<Transaction>().map_err(|_| GovernanceError::InvalidInput)?;

        while instance
            .data::<Data>()
            .governance_call
            .get_or_default()
            .pop_front()
            .ok_or(GovernanceError::ExecutionFailed(transaction.clone()))?
            != transaction
        {}
    }

    body(instance)
}

pub trait GovernorImpl:
    Storage<Data>
    + GovernorEvents
    + GovernorInternal
    + GovernorVotesInternal
    + GovernorSettingsInternal
    + NoncesImpl
    + GovernorSettingsImpl
    + GovernorStorageGetters
    + TimestampProvider
{
    fn hash_proposal(
        &self,
        transactions: Vec<Transaction>,
        description_hash: HashType,
    ) -> Result<HashType, GovernanceError> {
        self._hash_proposal(transactions, description_hash).into()
    }

    fn state(&self, proposal_id: ProposalId) -> Result<ProposalState, GovernanceError> {
        self._state(proposal_id)
    }

    fn proposal_snapshot(&self, proposal_id: ProposalId) -> Result<Timestamp, GovernanceError> {
        self._proposal_snapshot(proposal_id)
    }

    fn proposal_deadline(&self, proposal_id: ProposalId) -> Result<Timestamp, GovernanceError> {
        self.data::<Data>()
            .proposals
            .get(&proposal_id)
            .ok_or(GovernanceError::ProposalNotFound)?
            .deadline()
    }

    fn proposal_proposer(&self, proposal_id: ProposalId) -> Result<AccountId, GovernanceError> {
        self._proposal_proposer(proposal_id)
    }

    fn get_votes_with_params(
        &mut self,
        account: AccountId,
        time_point: Timestamp,
        params: Vec<u8>,
    ) -> Result<u128, GovernanceError> {
        self._get_votes(account, time_point, params)
    }

    fn propose(&mut self, transactions: Vec<Transaction>, description: String) -> Result<ProposalId, GovernanceError> {
        let proposer = Self::env().caller();

        if transactions.is_empty() {
            return Err(GovernanceError::ZeroProposalLength)
        }

        if !self._is_valid_description_for_proposer(proposer, description.clone())? {
            return Err(GovernanceError::ProposerRestricted(proposer))
        }

        let current_timestamp = TimestampProvider::block_timestamp(self);

        let proposer_votes = self.get_votes_with_params(proposer, current_timestamp.clone(), Vec::new())?;

        let votes_threshold = self.proposal_threshold();

        if proposer_votes < votes_threshold {
            return Err(GovernanceError::InsufficientProposerVotes(
                proposer,
                proposer_votes,
                votes_threshold,
            ))
        }

        let description_hash = self._hash_description(description.clone())?;

        let proposal_id = self.hash_proposal(transactions.clone(), description_hash)?;

        if self.data::<Data>().proposals.contains(&proposal_id) {
            return Err(GovernanceError::ProposalAlreadyExists)
        }

        let snapshot = current_timestamp + self.voting_delay();
        let duration = self.voting_period();

        self.data::<Data>().proposals.insert(
            &proposal_id,
            &ProposalCore {
                proposer: proposer.clone(),
                vote_start: snapshot.clone(),
                vote_duration: duration.clone(),
                ..Default::default()
            },
        );

        self.emit_proposal_created(
            proposal_id.clone(),
            proposer,
            transactions,
            snapshot,
            snapshot
                .checked_add(duration)
                .ok_or(GovernanceError::DeadlineOverflow)?,
            description.clone(),
        );

        Ok(proposal_id)
    }

    fn execute(
        &mut self,
        transactions: Vec<Transaction>,
        description_hash: HashType,
    ) -> Result<ProposalId, GovernanceError> {
        let proposal_id = self.hash_proposal(transactions.clone(), description_hash)?;

        let current_state = self.state(proposal_id.clone())?;

        if current_state != ProposalState::Succeeded && current_state != ProposalState::Queued {
            return Err(GovernanceError::UnexpectedProposalState(
                proposal_id.clone(),
                current_state,
                ProposalState::Succeeded.u128() | ProposalState::Queued.u128(),
            ))
        }

        let proposal = self
            .data::<Data>()
            .proposals
            .get(&proposal_id)
            .ok_or(GovernanceError::ProposalNotFound)?;

        self.data::<Data>().proposals.insert(
            &proposal_id,
            &ProposalCore {
                executed: ExecutionStatus::Executed,
                ..proposal
            },
        );

        self._before_execute(transactions.clone(), description_hash.clone())?;

        self._execute(transactions.clone(), description_hash.clone())?;

        self._after_execute(transactions.clone(), description_hash.clone())?;

        self.emit_proposal_executed(proposal_id.clone());

        Ok(proposal_id)
    }

    fn cancel(
        &mut self,
        transactions: Vec<Transaction>,
        description_hash: HashType,
    ) -> Result<ProposalId, GovernanceError> {
        let proposal_id = self.hash_proposal(transactions.clone(), description_hash.clone())?;

        let current_state = self.state(proposal_id.clone())?;

        let caller = Self::env().caller();

        if current_state != ProposalState::Pending {
            return Err(GovernanceError::UnexpectedProposalState(
                proposal_id.clone(),
                current_state,
                ProposalState::Pending.u128(),
            ))
        }

        if caller != self.proposal_proposer(proposal_id.clone())? {
            return Err(GovernanceError::OnlyProposer(caller))
        }

        self._cancel(transactions, description_hash)
    }

    fn cast_vote(&mut self, proposal_id: ProposalId, support: VoteType) -> Result<Balance, GovernanceError> {
        let voter = Self::env().caller();

        self._cast_vote(proposal_id, voter, support, String::new())
    }

    fn cast_vote_with_reason(
        &mut self,
        proposal_id: ProposalId,
        support: VoteType,
        reason: String,
    ) -> Result<Balance, GovernanceError> {
        let voter = Self::env().caller();

        self._cast_vote_with_params(proposal_id, voter, support, reason, Vec::new())
    }

    fn cast_vote_with_reason_and_params(
        &mut self,
        proposal_id: ProposalId,
        support: VoteType,
        reason: String,
        params: Vec<u8>,
    ) -> Result<Balance, GovernanceError> {
        let voter = Self::env().caller();

        self._cast_vote_with_params(proposal_id, voter, support, reason, params)
    }

    fn cast_vote_with_signature(
        &mut self,
        proposal_id: ProposalId,
        support: VoteType,
        reason: String,
        signature: SignatureType,
    ) -> Result<Balance, GovernanceError> {
        let message = crypto::hash_message(
            (proposal_id.clone(), support.clone(), reason.clone(), Vec::<u8>::new())
                .encode()
                .as_slice(),
        )?;

        let voter = Self::env().caller();
        let valid = crypto::verify_signature(&message, &voter.clone(), &signature)?;

        if !valid {
            return Err(GovernanceError::InvalidSignature(voter.clone()))
        }

        self._cast_vote(proposal_id, voter, support, reason)
    }

    fn cast_vote_with_signature_and_params(
        &mut self,
        proposal_id: ProposalId,
        support: VoteType,
        reason: String,
        signature: SignatureType,
        params: Vec<u8>,
    ) -> Result<Balance, GovernanceError> {
        let message = crypto::hash_message(
            (proposal_id.clone(), support.clone(), reason.clone(), params.clone())
                .encode()
                .as_slice(),
        )?;

        let voter = Self::env().caller();
        let valid = crypto::verify_signature(&message, &voter.clone(), &signature)?;

        if !valid {
            return Err(GovernanceError::InvalidSignature(voter.clone()))
        }

        self._cast_vote_with_params(proposal_id, voter, support, reason, params)
    }

    #[modifiers(only_governance)]
    fn relay(&mut self, target: AccountId, transaction: Transaction) -> Result<(), GovernanceError> {
        build_call::<DefaultEnvironment>()
            .call(target)
            .transferred_value(transaction.transferred_value)
            .exec_input(ExecutionInput::new(transaction.selector.into()).push_arg(transaction.clone().input))
            .returns::<()>()
            .try_invoke()
            .map_err(|_| GovernanceError::ExecutionFailed(transaction.clone()))?
            .map_err(|_| GovernanceError::ExecutionFailed(transaction.clone()))?;

        Ok(())
    }
}
