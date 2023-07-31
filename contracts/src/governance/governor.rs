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

use crate::governance::extensions::timelock_controller::CallInput;
use crate::traits::errors::GovernorError::DeadlineOverflow;
use crate::traits::types::Transaction;
use crate::utils::crypto;
use crate::utils::crypto::hash_message;
pub use crate::{traits::governance::*, utils::nonces::*};
use ink::env::call::ExecutionInput;
use ink::env::{CallFlags, DefaultEnvironment};
use ink::prelude::vec;
use ink::primitives::AccountId;
use ink::{
    env::call::{build_call, Selector},
    prelude::{string::String, vec::Vec},
};
use openbrush::{
    storage::Mapping,
    traits::{Balance, Storage, Timestamp},
};
use scale::Encode;

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    pub proposals: Mapping<ProposalId, ProposalCore>,
    #[lazy]
    governance_call: Vec<Selector>,
}

#[derive(scale::Decode, scale::Encode, Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
enum ExecutionStatus {
    #[default]
    NotExecuted,
    Executed,
}

#[derive(scale::Decode, scale::Encode, Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
enum CancellationStatus {
    #[default]
    NotCanceled,
    Canceled,
}

#[derive(scale::Decode, scale::Encode, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
pub struct ProposalCore {
    proposer: AccountId,
    vote_start: Timestamp,
    vote_duration: Timestamp,
    executed: ExecutionStatus,
    canceled: CancellationStatus,
}

impl Default for ProposalCore {
    fn default() -> Self {
        Self {
            proposer: AccountId::from([0x00; 32]),
            vote_start: Default::default(),
            vote_duration: Default::default(),
            executed: Default::default(),
            canceled: Default::default(),
        }
    }
}

impl ProposalCore {
    pub fn new(proposer: AccountId, vote_start: Timestamp, vote_duration: Timestamp) -> Self {
        Self {
            proposer,
            vote_start,
            vote_duration,
            executed: ExecutionStatus::NotExecuted,
            canceled: CancellationStatus::NotCanceled,
        }
    }

    pub fn is_executed(&self) -> bool {
        self.executed == ExecutionStatus::Executed
    }

    pub fn is_canceled(&self) -> bool {
        self.canceled == CancellationStatus::Canceled
    }

    pub fn deadline(&self) -> Result<u64, GovernorError> {
        let start = self.vote_start.clone();
        let duration = self.vote_duration.clone();

        start.checked_add(duration).ok_or(DeadlineOverflow)
    }

    pub fn hash(&self) -> [u8; 32] {
        use ink::env::hash;

        let mut bytes: Vec<u8> = scale::Encode::encode(&self);

        let mut output = <hash::Blake2x256 as hash::HashOutput>::Type::default();
        ink::env::hash_bytes::<hash::Blake2x256>(&bytes[..], &mut output);

        output
    }
}

pub trait GovernorImpl: Storage<Data> + Internal + Nonces {
    /// [OK]
    fn state(&self, proposal_id: &ProposalId) -> Result<ProposalState, GovernorError> {
        let proposal = self
            .data()
            .proposals
            .get(proposal_id)
            .ok_or(GovernorError::NonexistentProposal(proposal_id.clone()))?;

        if proposal.executed == ExecutionStatus::Executed {
            return Ok(ProposalState::Executed);
        }

        if proposal.canceled == CancellationStatus::Canceled {
            return Ok(ProposalState::Canceled);
        }

        let snapshot = self.proposal_snapshot(&proposal_id);
        if snapshot == 0 {
            return Err(GovernorError::ZeroSnapshot);
        }

        let current_timestamp = Self::env().block_timestamp();
        if current_timestamp <= snapshot {
            return Ok(ProposalState::Pending);
        }

        let deadline = self.proposal_deadline(proposal_id.clone())?;
        if current_timestamp <= deadline {
            return Ok(ProposalState::Active);
        }

        return if self._quorum_reached(&proposal_id) && self._vote_succeeded(&proposal_id) {
            Ok(ProposalState::Succeeded)
        } else {
            Ok(ProposalState::Defeated)
        };
    }

    /// [OK]
    fn proposal_threshold(&self) -> u128 {
        0
    }

    /// [OK]
    fn proposal_snapshot(&self, proposal_id: &ProposalId) -> Timestamp {
        self.data().proposals.get(proposal_id).unwrap_or_default().vote_start
    }

    /// [OK]
    fn proposal_deadline(&self, proposal_id: ProposalId) -> Result<Timestamp, GovernorError> {
        let proposal = self.data().proposals.get(&proposal_id).unwrap_or_default();
        proposal.deadline()
    }

    /// [OK]
    fn proposal_proposer(&self, proposal_id: ProposalId) -> AccountId {
        self.data().proposals.get(&proposal_id).unwrap_or_default().proposer
    }

    /// [NOT OK]
    fn propose(&mut self, transactions: Vec<Transaction>, description: String) -> Result<ProposalId, GovernorError> {
        let proposer = Self::env().caller();

        if !self._is_valid_description_for_proposer(proposer.clone(), description.clone()) {
            return Err(GovernorError::ProposerRestricted(proposer.clone()));
        }

        let current_timestamp = Self::env().block_timestamp();
        // current_timestamp - 1 can't underflow because it's always > 0
        let proposer_votes = self.get_votes(proposer, current_timestamp.clone() - 1);
        let votes_threshold = self.proposal_threshold();

        if proposer_votes < votes_threshold {
            return Err(GovernorError::InsufficientProposerVotes(
                proposer.clone(),
                proposer_votes.clone(),
                votes_threshold.clone(),
            ));
        }

        let description_hash = hash_message(scale::Encode::encode(&description).as_slice())?;
        let proposal_id = self.hash_proposal(transactions, &description_hash)?;

        if transactions.len() == 0 {
            return Err(GovernorError::ZeroProposalLength);
        }

        if let Some(proposal) = self.data().proposals.get(&proposal_id) {
            if proposal.vote_start != 0 {
                return Err(GovernorError::UnexpectedProposalState(
                    proposal_id.clone(),
                    self.state(&proposal_id)?,
                    0,
                ));
            }
        }

        // TODO: + voting delay
        let snapshot = current_timestamp + self.voting_delay();
        let duration = self.voting_period();

        self.data().proposals.insert(
            &proposal_id,
            &ProposalCore {
                proposer: proposer.clone(),
                vote_start: snapshot,
                vote_duration: duration,
                executed: ExecutionStatus::NotExecuted,
                canceled: CancellationStatus::NotCanceled,
            },
        );

        self._emit_proposal_created_event(&proposal_id, &proposer, &description);

        Ok(proposal_id)
    }

    /// [NOT OK]
    fn execute(
        &mut self,
        transactions: Vec<Transaction>,
        description_hash: [u8; 32],
    ) -> Result<ProposalId, GovernorError> {
        let proposal_id = self.hash_proposal(transactions, &description_hash)?;

        let current_state = self.state(&proposal_id)?;

        if current_state != ProposalState::Succeeded && current_state != ProposalState::Queued {
            return Err(GovernorError::UnexpectedProposalState(
                proposal_id.clone(),
                current_state.clone(),
                ProposalState::Succeeded.u128() | ProposalState::Queued.u128(),
            ));
        }

        let mut proposal = self
            .data()
            .proposals
            .get(&proposal_id)
            .ok_or(GovernorError::NonexistentProposal(proposal_id.clone()))?;

        proposal.executed = ExecutionStatus::Executed;

        self.data().proposals.insert(&proposal_id, &proposal);

        self._emit_proposal_executed_event(&proposal_id);

        self._before_execute(proposal_id.clone(), transactions.clone())?;
        self._execute(proposal_id.clone(), transactions.clone())?;
        self._after_execute(proposal_id.clone(), transactions.clone())?;

        Ok(proposal_id)
    }

    /// [NOT OK]
    fn cancel(&mut self, transactions: Vec<Transaction>, description_hash: [u8; 32]) -> Result<u128, GovernorError> {
        let proposal_id = self.hash_proposal(transactions, &description_hash)?;

        let current_state = self.state(&proposal_id)?;

        if current_state != ProposalState::Pending {
            return Err(GovernorError::UnexpectedProposalState(
                proposal_id.clone(),
                current_state.clone(),
                ProposalState::Pending.into(),
            ));
        }

        let caller = Self::env().caller();

        if caller != self.proposal_proposer(proposal_id.clone()) {
            return Err(GovernorError::OnlyProposer(Self::env().caller()));
        }

        self._cancel(transactions.clone(), description_hash.clone())
    }

    fn get_votes(&self, account: AccountId, timestamp: Timestamp) -> u128 {
        self._get_votes(account, timestamp, vec![])
    }

    fn get_votes_with_params(&self, account: AccountId, timestamp: Timestamp, params: Vec<u8>) -> u128 {
        self._get_votes(account, timestamp, params)
    }

    fn cast_vote(&mut self, proposal_id: ProposalId, support: bool) -> Result<u128, GovernorError> {
        let voter = Self::env().caller();
        self._cast_vote(proposal_id, voter, support, String::new(), vec![])
    }

    fn cast_vote_with_reason(
        &mut self,
        proposal_id: ProposalId,
        support: bool,
        reason: String,
    ) -> Result<Balance, GovernorError> {
        let voter = Self::env().caller();
        self._cast_vote(proposal_id, voter, support, reason, vec![])
    }

    fn cast_vote_with_reason_and_params(
        &mut self,
        proposal_id: ProposalId,
        support: bool,
        reason: String,
        params: Vec<u8>,
    ) -> Result<Balance, GovernorError> {
        let voter = Self::env().caller();
        self._cast_vote(proposal_id, voter, support, reason, params)
    }

    fn cast_vote_by_sig(
        &mut self,
        proposal_id: ProposalId,
        support: bool,
        voter: AccountId,
        signature: &[u8; 65],
    ) -> Result<Balance, GovernorError> {
        let message_hash =
            hash_message(Encode::encode(&(&proposal_id, &support, voter.clone(), self._use_nonce(&voter))).as_slice())?;

        let verify = crypto::verify_signature(&message_hash, &voter, &signature)
            .map_err(|_| GovernorError::InvalidSignature(voter))?;

        if !verify {
            Err(GovernorError::InvalidSignature(voter))
        } else {
            self._cast_vote(proposal_id, voter, support, String::new(), self.default_params())
        }
    }

    fn cast_vote_with_reason_and_params_by_sig(
        &mut self,
        proposal_id: ProposalId,
        support: bool,
        voter: AccountId,
        reason: String,
        params: Vec<u8>,
        signature: &[u8; 65],
    ) -> Result<Balance, GovernorError> {
        let message_hash = crypto::hash_message(
            Encode::encode(&(
                &proposal_id,
                &support,
                voter.clone(),
                &reason,
                &params,
                self._use_nonce(&voter),
            ))
            .as_slice(),
        )?;

        let verify = crypto::verify_signature(&message_hash, &voter, &signature)
            .map_err(GovernorError::InvalidSignature(voter))?;

        if !verify {
            Err(GovernorError::InvalidSignature(voter))
        } else {
            self._cast_vote(proposal_id, voter, support, reason, params)
        }
    }

    //#[openbrush::modifier(only_governance)]
    fn relay(&mut self, target: AccountId, value: Balance, data: Vec<u8>) -> Result<(), GovernorError> {
        todo!("relay")
    }

    fn hash_proposal(
        &self,
        transactions: Vec<Transaction>,
        description_hash: &[u8; 32],
    ) -> Result<[u8; 32], GovernorError> {
        let encoded_msg = Encode::encode(&(transactions, description_hash));

        let mut proposal_id: [u8; 32] = [0; 32];
        ink::env::hash_bytes::<ink::env::hash::Blake2x256>(&encoded_msg, &mut proposal_id);

        Ok(proposal_id)
    }
}

pub trait Internal {
    fn _execute(&mut self, proposal_id: ProposalId, transactions: Vec<Transaction>) -> Result<u128, GovernorError>;

    fn _before_execute(&mut self, proposal_id: ProposalId, transactions: Vec<Transaction>)
        -> Result<(), GovernorError>;

    fn _after_execute(&mut self, proposal_id: ProposalId, transactions: Vec<Transaction>) -> Result<(), GovernorError>;

    fn _cancel(&mut self, transactions: Vec<Transaction>, description_hash: &[u8; 32]) -> Result<u128, GovernorError>;

    fn _cast_vote(
        &mut self,
        proposal_id: ProposalId,
        voter: AccountId,
        support: bool,
        reason: String,
        params: Vec<u8>,
    ) -> Result<u128, GovernorError>;

    fn _is_valid_description_for_proposer(&self, proposer: AccountId, description: String) -> bool;

    fn _encode_state_bitmap(&self, proposal_state: ProposalState) -> Vec<u8>;

    fn _try_hex_to_uint(&self, char: char) -> (bool, u8);

    /// TODO
    fn _get_votes(&self, account: AccountId, timestamp: Timestamp, params: Vec<u8>) -> u128;

    /// TODO
    fn _count_vote(
        &self,
        proposal_id: &ProposalId,
        voter: &AccountId,
        support: &bool,
        weight: &u128,
        reason: String,
        params: Vec<u8>,
    ) -> Result<u128, GovernorError>;

    /// TODO
    fn _quorum_reached(&self, proposal_id: &ProposalId) -> bool;

    /// TODO
    fn _vote_reached(&self, proposal_id: &ProposalId) -> bool;

    /// TODO
    fn voting_delay(&self) -> u64;

    /// TODO
    fn voting_period(&self) -> u64;

    fn _emit_proposal_executed_event(&self, proposal_id: &ProposalId, initiator: &AccountId);

    fn _emit_vote_cast_with_params_event(
        &self,
        voter: &AccountId,
        proposal_id: &ProposalId,
        support: &bool,
        votes: &u128,
        reason: &String,
        params: &Vec<u8>,
    );

    fn _emit_vote_cast_event(
        &self,
        voter: &AccountId,
        proposal_id: &ProposalId,
        support: &bool,
        votes: &u128,
        reason: &String,
    );

    fn _emit_proposal_created_event(&self, proposal_id: &ProposalId, proposer: &AccountId, description: &String);

    fn _emit_proposal_canceled_event(&self, proposal_id: &ProposalId);
}

pub trait InternalImpl: Storage<Data> + Internal + GovernorImpl {
    /// TODO : recheck this method
    fn _execute(
        &mut self,
        _proposal_id: ProposalId,
        transactions: Vec<Transaction>,
        _description_hash: &[u8; 32],
    ) -> Result<(), GovernorError> {
        for tx in transactions {
            build_call::<DefaultEnvironment>()
                .call_type(
                    ink::env::call::Call::new(tx.callee.ok_or(GovernorError::InvalidInput)?)
                        .gas_limit(tx.gas_limit)
                        .transferred_value(tx.transferred_value),
                )
                .exec_input(
                    ExecutionInput::new(tx.selector.into())
                        .push_arg(CallInput::try_from(&tx.input).map_err(|_| GovernorError::InvalidInput)?),
                )
                .returns::<()>()
                .call_flags(CallFlags::default().set_allow_reentry(true))
                .try_invoke()
                .map_err(|_| GovernorError::UnderlyingTransactionReverted)?
                .map_err(|_| GovernorError::UnderlyingTransactionReverted)?;
        }

        Ok(())
    }

    fn _before_execute(
        &mut self,
        _proposal_id: ProposalId,
        transactions: Vec<Transaction>,
        _description_hash: &[u8; 32],
    ) -> Result<(), GovernorError> {
        let account_id = Self::env().account_id();
        if self._executor() != account_id {
            for tx in transactions {
                if tx.callee == Option::from(account_id) {
                    let mut governance_call: Vec<Selector> = self.data().governance_call.get().unwrap_or_default();
                    governance_call.push(Selector::new(tx.selector));
                    self.data().governance_call.set(&governance_call);
                }
            }
        }
        Ok(())
    }

    fn _after_execute(
        &mut self,
        _proposal_id: ProposalId,
        _transactions: Vec<Transaction>,
        _description_hash: &[u8; 32],
    ) -> Result<(), GovernorError> {
        if self._executor() != Self::env().account_id() {
            if !self.data().governance_call.get().unwrap_or_default().is_empty() {
                self.data().governance_call.set(&Vec::new());
            }
        }
        Ok(())
    }

    fn _cancel(
        &mut self,
        transactions: Vec<Transaction>,
        description_hash: &[u8; 32],
    ) -> Result<ProposalId, GovernorError> {
        let proposal_id = self.hash_proposal(transactions, description_hash)?;

        let current_state = self.state(&proposal_id)?;

        let forbidden_states =
            ProposalState::Canceled.u128() | ProposalState::Expired.u128() | ProposalState::Executed.u128();

        if current_state.clone().u128() & forbidden_states.clone() != 0 {
            return Err(GovernorError::UnexpectedProposalState(
                proposal_id.clone(),
                current_state,
                ALL_PROPOSAL_STATES ^ forbidden_states.clone(),
            ));
        }

        let mut proposal = self
            .data()
            .proposals
            .get(&proposal_id)
            .ok_or(GovernorError::ProposalNotFound)?;
        proposal.canceled = CancellationStatus::Canceled;
        self.data().proposals.insert(&proposal_id, &proposal);

        self._emit_proposal_canceled_event(&proposal_id);

        Ok(proposal_id)
    }

    fn _cast_vote(
        &mut self,
        proposal_id: ProposalId,
        voter: AccountId,
        support: bool,
        reason: String,
        params: Vec<u8>,
    ) -> Result<u128, GovernorError> {
        let current_state = self.state(&proposal_id)?;

        if current_state != ProposalState::Active {
            return Err(GovernorError::UnexpectedProposalState(
                proposal_id.clone(),
                current_state.clone(),
                ProposalState::Active as u128,
            ));
        }

        let weight = self._get_votes(voter.clone(), self.proposal_snapshot(&proposal_id), params.clone());
        self._count_vote(&proposal_id, &voter, &support, &weight, reason.clone(), params.clone());

        if params.len() == 0 {
            self._emit_vote_cast_event(&voter, &proposal_id, &support, &weight, &reason);
        } else {
            self._emit_vote_cast_with_params_event(&voter, &proposal_id, &support, &weight, &reason, &params);
        }

        Ok(weight)
    }

    /// TODO : recheck this method
    fn _is_valid_description_for_proposer(&self, proposer: AccountId, description: String) -> bool {
        true
    }

    fn _executor(&self) -> AccountId {
        Self::env().caller()
    }
}
