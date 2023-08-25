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
    governance::{
        extensions::{governor_counting::CountingInternal, governor_votes::GovernorVotesInternal},
        governor::{CallInput, Data, GovernorEvents},
    },
    traits::{
        errors::governance::GovernanceError,
        governance::{
            CancelationStatus, ExecutionStatus, HashType, ProposalCore, ProposalId, ProposalState, Transaction,
            VoteType, ALL_PROPOSAL_STATES,
        },
    },
    utils::crypto,
};
use ink::{
    env::{
        call::{build_call, Call, ExecutionInput, Selector},
        CallFlags, DefaultEnvironment,
    },
    prelude::{borrow::ToOwned, collections::VecDeque, vec::Vec},
};
use openbrush::traits::{AccountId, Balance, DefaultEnv, Storage, String};
use scale::Encode;

pub trait GovernorInternal:
    Storage<Data> + GovernorEvents + CountingInternal + GovernorVotesInternal + TimestampProvider
{
    /// Hashing function used to (re)build the proposal id from the proposal details.
    fn _hash_proposal(
        &self,
        transactions: Vec<Transaction>,
        description_hash: HashType,
    ) -> Result<HashType, GovernanceError> {
        let message = (transactions, description_hash).encode();

        crypto::hash_message(message.as_slice()).map_err(|err| err.into())
    }

    /// Current state of a proposal, following Compound's convention
    fn _state(&self, proposal_id: ProposalId) -> Result<ProposalState, GovernanceError> {
        let current_time = self.block_timestamp();

        let proposal = self
            .data::<Data>()
            .proposals
            .get(&proposal_id)
            .ok_or(GovernanceError::NonexistentProposal)?;

        if proposal.executed == ExecutionStatus::Executed {
            return Ok(ProposalState::Executed);
        }

        if proposal.canceled == CancelationStatus::Canceled {
            return Ok(ProposalState::Canceled);
        }

        let snapshot = proposal.vote_start;

        if snapshot > current_time {
            return Ok(ProposalState::Pending);
        }

        let deadline = proposal.deadline()?;

        if deadline >= current_time {
            return Ok(ProposalState::Active);
        }

        if self._vote_succeeded(proposal_id.clone()) && self._quorum_reached(proposal_id.clone())? {
            Ok(ProposalState::Succeeded)
        } else {
            Ok(ProposalState::Defeated)
        }
    }

    /// Returns default parameters for the proposal
    fn _default_params(&self) -> Vec<u8> {
        Vec::new()
    }

    /// Executes a proposal if it is in the `Succeeded` state.
    fn _execute(&mut self, transactions: Vec<Transaction>, _description_hash: HashType) -> Result<(), GovernanceError> {
        for tx in transactions.iter() {
            build_call::<DefaultEnvironment>()
                .call_type(
                    Call::new(tx.callee.clone())
                        .gas_limit(1000000000)
                        .transferred_value(tx.transferred_value.clone()),
                )
                .exec_input(ExecutionInput::new(Selector::new(tx.selector.clone())).push_arg(CallInput(&tx.input)))
                .call_flags(CallFlags::default().set_allow_reentry(true))
                .returns::<()>()
                .try_invoke()
                .map_err(|_| GovernanceError::ExecutionFailed)?
                .map_err(|_| GovernanceError::ExecutionFailed)?;
        }

        Ok(())
    }

    /// Adds a proposal to the queue of proposals to be executed by the governor.
    fn _before_execute(
        &mut self,
        transactions: Vec<Transaction>,
        _description_hash: HashType,
    ) -> Result<(), GovernanceError> {
        let self_address = Self::env().account_id();
        let executor = self._executor();
        if executor != self_address {
            for tx in transactions.iter() {
                if tx.callee == self_address {
                    let mut governance_call = self.data::<Data>().governance_call.get_or_default();
                    governance_call.push_back(tx.clone());
                    self.data::<Data>().governance_call.set(&governance_call);
                }
            }
        }
        Ok(())
    }

    /// Removes a proposal from the queue of proposals to be executed by the governor.
    fn _after_execute(
        &mut self,
        _transactions: Vec<Transaction>,
        _description_hash: HashType,
    ) -> Result<(), GovernanceError> {
        if self._executor() != Self::env().account_id()
            && !self.data::<Data>().governance_call.get_or_default().is_empty()
        {
            self.data::<Data>().governance_call.set(&VecDeque::new());
        }

        Ok(())
    }

    /// Cancels a proposal.
    fn _cancel(
        &mut self,
        transactions: Vec<Transaction>,
        description_hash: HashType,
    ) -> Result<ProposalId, GovernanceError> {
        let proposal_id = self._hash_proposal(transactions, description_hash)?;
        let current_state = self._state(proposal_id.clone())?;

        let forbidden_states =
            ProposalState::Canceled.u128() | ProposalState::Executed.u128() | ProposalState::Expired.u128();

        if forbidden_states.clone() & current_state.clone().u128() != 0 {
            return Err(GovernanceError::UnexpectedProposalState);
        }

        let proposal = self
            .data::<Data>()
            .proposals
            .get(&proposal_id)
            .ok_or(GovernanceError::ProposalNotFound)?;

        self.data::<Data>().proposals.insert(
            &proposal_id,
            &ProposalCore {
                canceled: CancelationStatus::Canceled,
                ..proposal
            },
        );

        self.emit_proposal_canceled(proposal_id.clone());

        Ok(proposal_id)
    }

    /// Casts a vote on a proposal with `proposal_id`, `support`(for/against/abstain) and `reason`.
    fn _cast_vote(
        &mut self,
        proposal_id: ProposalId,
        account: AccountId,
        support: VoteType,
        reason: String,
    ) -> Result<Balance, GovernanceError> {
        self._cast_vote_with_params(proposal_id, account, support, reason, self._default_params())
    }

    /// Returns the AccountId of the proposer of a proposal
    fn _proposal_proposer(&self, proposal_id: ProposalId) -> Result<AccountId, GovernanceError> {
        Ok(self
            .data::<Data>()
            .proposals
            .get(&proposal_id)
            .ok_or(GovernanceError::ProposalNotFound)?
            .proposer)
    }

    /// Casts a vote on a proposal with `proposal_id`, `support`(for/against/abstain), `reason` and `params`.
    fn _cast_vote_with_params(
        &mut self,
        proposal_id: ProposalId,
        account: AccountId,
        support: VoteType,
        reason: String,
        params: Vec<u8>,
    ) -> Result<Balance, GovernanceError> {
        let current_state = self._state(proposal_id.clone())?;

        if current_state != ProposalState::Active {
            return Err(GovernanceError::UnexpectedProposalState);
        }

        let snapshot = self._proposal_snapshot(proposal_id.clone())?;
        let weight = self._get_votes(account.clone(), snapshot, params.clone())?;

        self._count_vote(proposal_id.clone(), account.clone(), support.clone(), weight.clone())?;

        if params.len() == 0 {
            self.emit_vote_cast(proposal_id.clone(), account.clone(), support, weight.clone(), reason);
        } else {
            self.emit_vote_cast_with_params(
                proposal_id.clone(),
                account.clone(),
                support,
                weight.clone(),
                reason,
                params,
            );
        }

        Ok(weight)
    }

    /// Returns the AccountId of the executor.
    fn _executor(&self) -> AccountId {
        Self::env().account_id()
    }

    /// Checks if the `description` is valid for the `proposer`.
    fn _is_valid_description_for_proposer(
        &self,
        proposer: AccountId,
        description: String,
    ) -> Result<bool, GovernanceError> {
        if !description.contains("#proposer=0x") {
            return Ok(true);
        }

        let pos = description.find("proposer=0x").unwrap() + 11usize;
        let address = &description[pos..];

        if hex::decode(address).is_err() {
            return Ok(true);
        }

        let proposer_str = hex::encode(proposer);
        let result = String::from("#proposer=0x".to_owned() + &proposer_str);

        Ok(description.ends_with(&result))
    }

    /// Returns amount of votes that voter needs to have to be able to vote.
    fn _proposal_threshold(&self) -> u128 {
        0
    }

    /// Return the hash of the description.
    fn _hash_description(&self, description: String) -> Result<HashType, GovernanceError> {
        Ok(crypto::hash_message(description.as_bytes())?)
    }
}

/// Provides custom timestamp functionality.
pub trait TimestampProvider: DefaultEnv {
    /// Returns the current block timestamp.
    fn block_timestamp(&self) -> u64 {
        Self::env().block_timestamp()
    }
}
