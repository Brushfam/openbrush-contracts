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
    governance::governor,
    traits::{
        errors::governance::GovernanceError,
        governance::{
            governor::*,
            *,
        },
        types::SignatureType,
    },
};
use crate::{
    governance::{
        extensions::{
            governor_settings::{
                GovernorSettingsImpl,
                GovernorSettingsInternal,
            },
            governor_votes::GovernorVotesInternal,
        },
        governor::{
            Data,
            GovernorEvents,
            GovernorInternal,
            GovernorStorageGetters,
            TimestampProvider,
        },
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

/// @dev Restricts a function so it can only be executed through governance proposals. For example, governance
/// parameter setters in {GovernorSettings} are protected using this modifier.
///
/// The governance executing address may be different from the Governor's own address, for example it could be a
/// timelock. This can be customized by modules by overriding `_executor`. The executor is only able to invoke these
/// functions during the execution of the governor's `execute` function, and not under any other circumstances. Thus,
/// for example, additional timelock proposers are not able to change governance parameters without going through the
/// governance protocol.
#[openbrush::modifier_definition]
pub fn only_governance<T, F, R, E>(instance: &mut T, body: F) -> Result<R, E>
where
    T: Storage<Data>,
    F: FnOnce(&mut T) -> Result<R, E>,
    E: From<GovernanceError>,
{
    if T::env().caller() != T::env().account_id() {
        return Err(GovernanceError::OnlyExecutor.into())
    }

    // todo: add check if executor is not this contract

    body(instance)
}

/// Core of the governance system, designed to be extended though various modules.
///
/// This contract is abstract and requires several functions to be implemented in various modules:
///
/// - A counting module must implement `quorum`, `_quorum_reached`, `_vote_succeeded` and `_count_vote`
/// - A voting module must implement `_get_votes`
/// - Additionally, `voting_period` must also be implemented
pub trait GovernorImpl:
    Storage<Data>
    + GovernorEvents
    + GovernorInternal
    + GovernorVotesInternal
    + GovernorSettingsInternal
    + GovernorSettingsImpl
    + GovernorStorageGetters
    + TimestampProvider
{
    /// Hashing function used to (re)build the proposal id from the proposal details.
    fn hash_proposal(
        &self,
        transactions: Vec<Transaction>,
        description_hash: HashType,
    ) -> Result<HashType, GovernanceError> {
        self._hash_proposal(transactions, description_hash).into()
    }

    /// Current state of a proposal, following Compound's convention
    fn state(&self, proposal_id: ProposalId) -> Result<ProposalState, GovernanceError> {
        self._state(proposal_id)
    }

    /// Returns timestamp at which votes for a proposal starts
    fn proposal_snapshot(&self, proposal_id: ProposalId) -> Result<Timestamp, GovernanceError> {
        self._proposal_snapshot(proposal_id)
    }

    /// Returns timestamp at which votes for a proposal ends
    fn proposal_deadline(&self, proposal_id: ProposalId) -> Result<Timestamp, GovernanceError> {
        self.data::<Data>()
            .proposals
            .get(&proposal_id)
            .ok_or(GovernanceError::ProposalNotFound)?
            .deadline()
    }

    /// Returns the AccountId of the proposer of a proposal
    fn proposal_proposer(&self, proposal_id: ProposalId) -> Result<AccountId, GovernanceError> {
        self._proposal_proposer(proposal_id)
    }
    /// Returns the number of votes already casted for a proposal by a given account
    fn get_votes_with_params(
        &mut self,
        account: AccountId,
        timestamp: Timestamp,
        params: Vec<u8>,
    ) -> Result<u128, GovernanceError> {
        self._get_votes(account, timestamp, params)
    }

    /// Makes a proposal for a list of transactions to be executed.
    /// Returns the id of the proposal
    fn propose(&mut self, transactions: Vec<Transaction>, description: String) -> Result<ProposalId, GovernanceError> {
        if transactions.is_empty() {
            return Err(GovernanceError::ZeroProposalLength)
        }

        if !self._is_valid_description_for_proposer(Self::env().caller(), description.clone())? {
            return Err(GovernanceError::ProposerRestricted)
        }

        let current_timestamp = TimestampProvider::block_timestamp(self);

        let proposer_votes = self.get_votes_with_params(Self::env().caller(), current_timestamp.clone(), Vec::new())?;

        let votes_threshold = self.proposal_threshold();

        if proposer_votes < votes_threshold {
            return Err(GovernanceError::InsufficientProposerVotes)
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
                proposer: Self::env().caller(),
                vote_start: snapshot,
                vote_duration: duration,
                executed: ExecutionStatus::NotExecuted,
                canceled: CancelationStatus::NotCanceled,
            },
        );

        self.emit_proposal_created(
            proposal_id,
            Self::env().caller(),
            transactions,
            snapshot,
            snapshot
                .checked_add(duration)
                .ok_or(GovernanceError::DeadlineOverflow)?,
            description,
        );

        Ok(proposal_id)
    }

    /// Executes a proposal if it is in the `Succeeded` state.
    /// Returns the id of the executed proposal
    fn execute(
        &mut self,
        transactions: Vec<Transaction>,
        description_hash: HashType,
    ) -> Result<ProposalId, GovernanceError> {
        let proposal_id = self.hash_proposal(transactions.clone(), description_hash)?;

        let current_state = self.state(proposal_id.clone())?;

        if current_state != ProposalState::Succeeded && current_state != ProposalState::Queued {
            return Err(GovernanceError::UnexpectedProposalState)
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

    /// Cancels a proposal if it is in the `Pending` state.
    /// Returns the id of the canceled proposal
    fn cancel(
        &mut self,
        transactions: Vec<Transaction>,
        description_hash: HashType,
    ) -> Result<ProposalId, GovernanceError> {
        let proposal_id = self.hash_proposal(transactions.clone(), description_hash.clone())?;

        let current_state = self.state(proposal_id.clone())?;

        if current_state != ProposalState::Pending {
            return Err(GovernanceError::UnexpectedProposalState)
        }

        if Self::env().caller() != self.proposal_proposer(proposal_id.clone())? {
            return Err(GovernanceError::OnlyProposer)
        }

        self._cancel(transactions, description_hash)
    }

    /// Casts a vote for a proposal from a message sender.
    /// Returns the number of votes already casted for the proposal by the sender
    fn cast_vote(
        &mut self,
        proposal_id: ProposalId,
        support: VoteType,
        reason: Option<String>,
        params: Option<Vec<u8>>,
    ) -> Result<Balance, GovernanceError> {
        self._cast_vote_with_params(
            proposal_id,
            Self::env().caller(),
            support,
            reason.unwrap_or_default(),
            params.unwrap_or_default(),
        )
    }
    /// Casts a vote with signature for a proposal from a message sender. Returns the number of votes already casted for the proposal by the sender
    fn cast_vote_with_signature(
        &mut self,
        proposal_id: ProposalId,
        support: VoteType,
        reason: String,
        signature: SignatureType,
        params: Option<Vec<u8>>,
    ) -> Result<Balance, GovernanceError> {
        let message = crypto::hash_message(
            (
                proposal_id.clone(),
                support.clone(),
                reason.clone(),
                params.clone().unwrap_or_default(),
            )
                .encode()
                .as_slice(),
        )?;

        let valid = crypto::verify_signature(&message, &Self::env().caller(), &signature)?;

        if !valid {
            return Err(GovernanceError::InvalidSignature)
        }

        self._cast_vote_with_params(
            proposal_id,
            Self::env().caller(),
            support,
            reason,
            params.unwrap_or_default(),
        )
    }

    /// Relays a transaction or function call to an arbitrary target. In cases where the governance executor
    /// is some contract other than the governor itself, like when using a timelock, this function can be invoked
    /// in a governance proposal to recover tokens or Ether that was sent to the governor contract by mistake.
    #[modifiers(only_governance)]
    fn relay(&mut self, target: AccountId, transaction: Transaction) -> Result<(), GovernanceError> {
        build_call::<DefaultEnvironment>()
            .call(target)
            .transferred_value(transaction.transferred_value)
            .exec_input(ExecutionInput::new(transaction.selector.into()).push_arg(transaction.clone().input))
            .returns::<()>()
            .try_invoke()
            .map_err(|_| GovernanceError::ExecutionFailed)?
            .map_err(|_| GovernanceError::ExecutionFailed)?;

        Ok(())
    }
}
