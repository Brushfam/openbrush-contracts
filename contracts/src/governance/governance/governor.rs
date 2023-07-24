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

use crate::timelock_controller::Transaction;
pub use crate::{
    governance,
    traits::governance::*,
    utils::nonces::*,
};
use ink::{
    blake2x256,
    env::call::{
        build_call,
        Selector,
    },
    prelude::{
        string::String,
        vec::Vec,
    },
    storage::traits::Storable,
};
use openbrush::{
    storage::{
        Mapping,
        TypeGuard,
    },
    traits::{
        AccountId,
        Balance,
        Storage,
        StorageAsMut,
        StorageAsRef,
        Timestamp,
    },
};
use secp256k1::{
    PublicKey,
    *,
};

// pub const _ALL_PROPOSAL_STATES_BITMAP : Vec<u8> = GovernorImpl::_encode_state_bitmap(ProposalState::Pending)
// | GovernorImpl::_encode_state_bitmap(ProposalState::Active)
// | GovernorImpl::_encode_state_bitmap(ProposalState::Canceled)
// | GovernorImpl::_encode_state_bitmap(ProposalState::Defeated)
// | GovernorImpl::_encode_state_bitmap(ProposalState::Succeeded)
// | GovernorImpl::_encode_state_bitmap(ProposalState::Queued)
// | GovernorImpl::_encode_state_bitmap(ProposalState::Expired)
// | GovernorImpl::_encode_state_bitmap(ProposalState::Executed);

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    #[lazy]
    pub name: String,
    pub proposals: Mapping<Id, ProposalCore>,
    #[lazy]
    governance_call: Vec<Selector>,
}

#[openbrush::storage_item]
enum ExecutionStatus {
    NotExecuted,
    Executed,
}

#[openbrush::storage_item]
enum CancellationStatus {
    NotCanceled,
    Canceled,
}

#[openbrush::storage_item]
pub struct ProposalCore {
    proposer: AccountId,
    vote_start: Timestamp,
    vote_duration: Timestamp,
    executed: ExecutionStatus,
    canceled: CancellationStatus,
}

pub trait GovernorImpl: Storage<Data> + Internal + Governor + Nonces {
    fn name(&self) -> String {
        self.data().name.clone()
    }

    fn set_name(&mut self, name: String) {
        self.data().name.set(&name);
    }

    fn state(&self, proposal_id: Id) -> Result<ProposalState, GovernorError> {
        let proposal = self
            .data()
            .proposals
            .get(&proposal_id)
            .ok_or(GovernorError::NonexistentProposal(proposal_id.clone()))?;

        if proposal.executed == ExecutionStatus::Executed {
            return Ok(ProposalState::Executed)
        }

        if proposal.canceled == CancellationStatus::Canceled {
            return Ok(ProposalState::Canceled)
        }

        let snapshot = self.proposal_snapshot(&proposal_id);

        if snapshot == 0 {
            return Err(GovernorError::ZeroSnapshot)
        }

        let current_timestamp = self.env().block_timestamp();

        if current_timestamp <= snapshot {
            return Ok(ProposalState::Pending)
        }

        let deadline = self.proposal_deadline(proposal_id).ok()?;

        if current_timestamp <= deadline {
            return Ok(ProposalState::Active)
        }

        return if self._quorum_reached(&proposal_id) && self._vote_succeeded(&proposal_id) {
            Ok(ProposalState::Succeeded)
        } else {
            Ok(ProposalState::Defeated)
        }
    }

    fn proposal_threshold(&self) -> u128 {
        0
    }

    fn proposal_snapshot(&self, proposal_id: &Id) -> Timestamp {
        self.data().proposals.get(proposal_id).unwrap_or_default().vote_start
    }

    fn proposal_deadline(&self, proposal_id: Id) -> Result<Timestamp, GovernorError> {
        let proposal = self.data().proposals.get(&proposal_id).unwrap_or_default();
        proposal
            .vote_start
            .checked_add(proposal.vote_duration) as Timestamp
            .ok_or(Err(GovernorError::DeadlineOverflow))?
    }

    fn proposal_proposer(&self, proposal_id: Id) -> AccountId {
        self.data().proposals.get(&proposal_id).unwrap_or_default().proposer
    }

    fn propose(&mut self, transactions: Vec<Transaction>) -> Result<Id, GovernorError> {
        let proposer = self.env().caller();

        if !self._is_valid_description_for_proposer(&description, &proposer) {
            return Err(GovernorError::InvalidDescription)
        }

        let current_timestamp = Self::env().block_timestamp();

        let proposer_votes = self.get_votes(proposer, current_timestamp - 1);

        let votes_threshold = self.proposal_threshold();

        if proposer_votes < votes_threshold {
            return Err(GovernorError::InsufficientProposerVotes(
                proposer.clone(),
                proposer_votes,
                votes_threshold,
            ))
        }

        let description_hash = blake2x256!(Storable::encode(description));

        let proposal_id = self.hash_proposal(&targets, &values, &calldatas, &description_hash);

        if targets.len() != values.len() || targets.len() != calldatas.len() || targets.len() == 0 {
            return Err(GovernorError::InvalidProposalLength(
                targets.len(),
                values.len(),
                calldatas.len(),
            ))
        }

        if let Some(proposal) = self.data().proposals.get(&proposal_id) {
            if proposal.vote_start == 0 {
                return Err(GovernorError::UnexpectedProposalState(
                    proposal_id.clone(),
                    self.state(&proposal_id),
                    Vec::<u8>::new(),
                ))
            }
        }
        // todo: + voting delay
        let snapshot = current_timestamp;
        let duration = self.voting_period();

        self.data().proposals.insert(
            &proposal_id,
            &ProposalCore {
                proposer: proposer.clone(),
                vote_start: snapshot,
                vote_duration: duration,
                executed: false,
                canceled: false,
            },
        );

        self._emit_proposal_created_event(
            &proposal_id,
            &proposer,
            &targets,
            &values,
            &calldatas,
            &description_hash,
        );

        Ok(proposal_id)
    }

    fn execute(
        &mut self,
        targets: Vec<AccountId>,
        values: Vec<Balance>,
        calldatas: Vec<Vec<u8>>,
        description_hash: Vec<u8>,
    ) -> Result<Id, GovernorError> {
        let proposal_id = self.hash_proposal(&targets, &values, &calldatas, &description_hash);

        let current_state = self.state(proposal_id.clone())?;

        if current_state != ProposalState::Succeeded && current_state != ProposalState::Queued {
            return Err(GovernorError::UnexpectedProposalState(
                proposal_id.clone(),
                current_state.clone(),
                Vec::new(),
            ))
        }

        let mut proposal = self
            .data()
            .proposals
            .get(&proposal_id)
            .ok_or(GovernorError::NonexistentProposal(proposal_id.clone()))?;

        proposal.executed = ExecutionStatus::Executed;

        self.data().proposals.insert(&proposal_id, &proposal);

        self._emit_proposal_executed_event(&proposal_id);

        self._before_execute(
            proposal_id.clone(),
            targets.clone(),
            values.clone(),
            calldatas.clone(),
            description_hash.clone(),
        );
        self._execute(
            proposal_id.clone(),
            targets.clone(),
            values.clone(),
            calldatas.clone(),
            description_hash.clone(),
        );
        self._after_execute(
            proposal_id.clone(),
            targets.clone(),
            values.clone(),
            calldatas.clone(),
            description_hash.clone(),
        );

        Ok(proposal_id)
    }

    fn cancel(
        &mut self,
        targets: Vec<AccountId>,
        values: Vec<Balance>,
        calldatas: Vec<Vec<u8>>,
        description_hash: Vec<u8>,
    ) -> Result<Id, GovernorError> {
        let description_hash = Blake2s::new().chain(description.as_bytes()).finalize().into();
        let proposal_id = self.hash_proposal(&targets, &values, &calldatas, &description_hash);

        let current_state = self.state(proposal_id.clone())?;

        if current_state != ProposalState::Pending {
            return Err(GovernorError::UnexpectedProposalState(
                proposal_id.clone(),
                current_state.clone(),
                _encode_state_bitmap(ProposalState::Pending),
            ))
        }

        let caller = Self::env().caller();

        if caller != self.proposal_proposer(proposal_id.clone()) {
            return Err(GovernorError::OnlyProposer(self.env().caller()))
        }

        self._cancel(
            targets.clone(),
            values.clone(),
            calldatas.clone(),
            description_hash.clone(),
        );
    }

    fn get_votes(&self, account: AccountId, timestamp: Timestamp) -> u128 {
        self._get_votes(account, timestamp, default_params())
    }

    fn get_votes_with_params(&self, account: AccountId, timestamp: Timestamp, params: Vec<u8>) -> u128 {
        self._get_votes(account, timestamp, params)
    }

    fn cast_vote(&mut self, proposal_id: Id, support: bool) -> Result<u128, GovernorError> {
        let voter = Self::env().caller();
        self._cast_vote(proposal_id, voter, support, "".to_string(), self.default_params())
    }

    fn cast_vote_with_reason(
        &mut self,
        proposal_id: Id,
        support: bool,
        reason: String,
    ) -> Result<Balance, GovernorError> {
        let voter = Self::env().caller();
        self._cast_vote(proposal_id, voter, support, reason, self.default_params())
    }

    fn cast_vote_with_reason_and_params(
        &mut self,
        proposal_id: Id,
        support: bool,
        reason: String,
        params: Vec<u8>,
    ) -> Result<Balance, GovernorError> {
        let voter = Self::env().caller();
        self._cast_vote(proposal_id, voter, support, reason, params)
    }

    fn cast_vote_by_sig(
        &mut self,
        proposal_id: Id,
        support: bool,
        voter: AccountId,
        signature: Vec<u8>,
    ) -> Result<Balance, GovernorError> {
        // todo
        let message_hash = Blake2s::new()
            .chain(proposal_id.as_bytes())
            .chain(support.to_le_bytes())
            .chain(voter.as_bytes())
            .chain(self._use_nonce(&voter).to_le_bytes())
            .finalize()
            .into();
        let public_key = PublicKey::from_slice(&voter.into());
        let sig = ecdsa::Signature::from_compact(&signature.into());
        let secp = Secp256k1::verification_only();
        let verify = secp.verify_ecdsa(&message_hash, &sig, &public_key);

        if !verify.is_ok() {
            return Err(GovernorError::InvalidSignature(voter))
        }

        return self._cast_vote(proposal_id, voter, support, "".to_string(), self.default_params())
    }

    fn cast_vote_with_reason_and_params_by_sig(
        &self,
        proposal_id: Id,
        support: bool,
        voter: AccountId,
        reason: String,
        params: Vec<u8>,
        signature: Vec<u8>,
    ) -> Result<Balance, GovernorError> {
        let message_hash = Blake2s::new()
            .chain(proposal_id.as_bytes())
            .chain(support.to_le_bytes())
            .chain(voter.as_bytes())
            .chain(self._use_nonce(&voter).to_le_bytes())
            .chain(Blake2s::new().chain(reason.as_bytes()).finalize().into())
            .chain(Blake2s::new().chain(params.into()).finalize().into())
            .finalize()
            .into();
        let public_key = PublicKey::from_slice(&voter.into());
        let sig = ecdsa::Signature::from_compact(&signature.into());
        let secp = Secp256k1::verification_only();
        let verify = secp.verify_ecdsa(&message_hash, &sig, &public_key);

        if !verify.is_ok() {
            return Err(GovernorError::InvalidSignature(voter))
        }

        return self._cast_vote(proposal_id, voter, support, reason, params)
    }

    //#[openbrush::modifier(only_governance)]
    fn relay(&mut self, target: AccountId, value: Balance, data: Vec<u8>) -> Result<(), GovernorError> {
        todo!("relay")
    }

    fn hash_proposal(
        &self,
        targets: &Vec<AccountId>,
        values: &Vec<Balance>,
        calldatas: &Vec<Vec<u8>>,
        description_hash: &Vec<u8>,
    ) -> Id {
        return Blake2s::new()
            .chain(targets)
            .chain(values)
            .chain(calldatas)
            .chain(description_hash)
            .finalize()
            .into()
    }
}

pub trait Internal {
    fn default_params(&self) -> Vec<u8>;

    fn _execute(&mut self, proposal_id: Id, transactions: Vec<Transaction>) -> Result<Id, GovernorError>;

    fn _before_execute(&mut self, proposal_id: Id, transactions: Vec<Transaction>) -> Result<(), GovernorError>;

    fn _after_execute(&mut self, proposal_id: Id, transactions: Vec<Transaction>) -> Result<(), GovernorError>;

    fn _cancel(&mut self, transactions: Vec<Transaction>) -> Result<Id, GovernorError>;

    fn _cast_vote(
        &mut self,
        proposal_id: Id,
        voter: AccountId,
        support: bool,
        reason: String,
        params: Vec<u8>,
    ) -> Result<u128, GovernorError>;

    fn _is_valid_description_for_proposer(&self, proposer: AccountId, description: String) -> bool;

    fn _encode_state_bitmap(&self, proposal_state: ProposalState) -> Vec<u8>;

    fn _executor(&self) -> AccountId;

    fn _try_hex_to_uint(&self, char: char) -> (bool, u8);
}

pub trait InternalImpl: Storage<Data> + Internal + GovernorImpl {
    fn default_params(&self) -> Vec<u8> {
        Vec::new()
    }

    fn _execute(&mut self, transaction: Transaction) -> Result<(), GovernorError> {
        for i in 0..targets.len() {
            ink::env::call::build_call::<ink::env::DefaultEnvironment>()
                .call(targets[i])
                .exec_input(Selector::new)
        }
    }

    fn _before_execute(&mut self, proposal_id: Id, transactions: Vec<Transaction>) -> Result<(), GovernorError> {
        if self._executor() != Self::env().account_id() {
            for i in 0..targets.len() {
                if targets[i] == Self::env().account_id() {
                    self.data().governance_call.push(<Blake2x256 as CryptoHash>::hash(
                        <Transaction as Storable>::encode(transaction),
                    ));
                }
            }
        }
        Ok(())
    }

    fn _after_execute(&mut self, proposal_id: Id, transactions: Vec<Transaction>) -> Result<(), GovernorError> {
        if self._executor() != Self::env().account_id() {
            if !governance_call.is_empty() {
                self.data().governance_call.clear();
            }
        }
        Ok(())
    }

    fn _cancel(&mut self, transactions: Vec<Transaction>) -> Result<Id, GovernorError> {
        let proposal_id = self.hash_proposal(&targets, &values, &calldatas, &description_hash);

        let current_state = self.state(&proposal_id)?;

        let forbiden_states = self._encode_state_bitmap(ProposalState::Canceled)
            | self._encode_state_bitmap(ProposalState::Expired)
            | self._encode_state_bitmap(ProposalState::Executed);

        // todo: make it const
        let _ALL_PROPOSAL_STATES_BITMAP = self._encode_state_bitmap(ProposalState::Pending)
            | self._encode_state_bitmap(ProposalState::Active)
            | self._encode_state_bitmap(ProposalState::Canceled)
            | self._encode_state_bitmap(ProposalState::Defeated)
            | self._encode_state_bitmap(ProposalState::Succeeded)
            | self._encode_state_bitmap(ProposalState::Queued)
            | self._encode_state_bitmap(ProposalState::Expired)
            | self._encode_state_bitmap(ProposalState::Executed);

        if self._encode_state_bitmap(current_state) & forbiden_states != 0 {
            return Err(GovernorError::UnexpectedProposalState(
                proposal_id,
                current_state,
                _ALL_PROPOSAL_STATES_BITMAP ^ forbiden_states,
            ))
        }

        let mut proposal = self.data().proposals.get(&proposal_id)?;
        proposal.canceled = CancellationStatus::Canceled;

        self._emit_proposal_canceled_event(proposal_id);

        proposal_id
    }

    fn _cast_vote(
        &mut self,
        proposal_id: Id,
        voter: AccountId,
        support: bool,
        reason: String,
        params: Vec<u8>,
    ) -> Result<u128, GovernorError> {
        let current_state = self.state(proposal_id);

        if !current_state.is_ok() {
            return Err(current_state.unwrap())
        } else if current_state != ProposalState::Active {
            return Err(GovernorError::UnexpectedProposalState(
                proposal_id.clone(),
                current_state.unwrap().clone(),
                self._encode_state_bitmap(ProposalState::Active),
            ))
        }
        let weight = self._get_votes(&voter, self.proposal_snapshot(&proposal_id), params.clone());
        self._count_vote(&proposal_id, &voter, &support, &weight, reason.clone(), params.clone());

        if params.len() == 0 {
            self._emit_vote_cast_event(&voter, &proposal_id, &support, &weight, &reason);
        } else {
            self._emit_vote_cast_with_params_event(&voter, &proposal_id, &support, &weight, &reason, &params);
        }

        return weight
    }

    fn _is_valid_description_for_proposer(&self, proposer: AccountId, description: String) -> bool {
        todo!("is_valid_description_for_proposer")
    }

    fn _encode_state_bitmap(&self, proposal_state: ProposalState) -> Vec<u8> {
        todo!("encode_state_bitmap")
    }

    fn _executor(&self) -> AccountId {
        Self::env().caller()
    }

    fn _try_hex_to_uint(&self, char: char) -> (bool, u8) {
        todo!("try_hex_to_uint")
    }
}
