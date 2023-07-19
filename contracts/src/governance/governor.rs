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
    governance,
    traits::governance::*,
};
use ink::prelude::{
    vec::Vec,
    string::String,
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
    },
};
use openbrush::traits::{StorageAsRef, Timestamp};
use blake2::Blake2s;

pub type Selector = Vec<u8>;

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    #[lazy]
    pub name: String,
    pub proposals: Mapping<Id, ProposalCore>,
    pub governance_call: Vec<Selector>,
}

#[openbrush::storage_item]
pub struct ProposalCore {
    proposer: AccountId,
    vote_start: Timestamp,
    vote_duration: Timestamp,
    executed: bool,
    canceled: bool,
}

pub trait GovernorImpl: Storage<Data> + Internal + Gobernor{
    fn name(&self) -> String {
        self.data().name.clone()
    }

    fn state(&self, proposal_id: Id) -> Result<ProposalState, GovernorError> {
        let proposal = self.data().proposals.get(&proposal_id).ok_or(GovernorError::NonexistentProposal(proposal_id.clone()))?;

        if proposal.executed {
            return Ok(ProposalState::Executed);
        }

        if proposal.canceled {
            return Ok(ProposalState::Canceled);
        }

        let snapshot = self.proposal_snapshot(&proposal_id);

        if snapshot == 0 {
            return Err(GovernorError::NonexistentProposal(proposal_id.clone()));
        }

        let current_timestamp = self.env().block_timestamp();

        if current_timestamp <= snapshot {
            return Ok(ProposalState::Pending);
        }

        let deadline = self.proposal_deadline(proposal_id);

        if current_timestamp <= deadline {
            return Ok(ProposalState::Active);
        }

        if _quorum_reached(&proposal_id) && _vote_succeeded(&proposal_id) {
            return Ok(ProposalState::Succeeded);
        } else {
            return Ok(ProposalState::Defeated);
        }
    }

    fn proposal_threshold(&self) -> u128 {
        0
    }

    fn proposal_snapshot(&self, proposal_id: &Id) -> Timestamp {
        self.data().proposals.get(proposal_id).map(|proposal| proposal.vote_start).unwrap_or(0)
    }

    fn proposal_deadline(&self, proposal_id: Id) -> Timestamp {
        self.data().proposals.get(&proposal_id).map(|proposal| proposal.vote_start + proposal.vote_duration).unwrap_or(0)
    }

    fn proposal_proposer(&self, proposal_id: Id) -> AccountId {
        self.data().proposals.get(&proposal_id).map(|proposal| proposal.proposer).unwrap_or(AccountId::default())
    }

    fn propose(&mut self,
               targets: Vec<AccountId>,
               values: Vec<Balance>,
               calldatas: Vec<Vec<u8>>,
               description: String
    ) -> Result<Id, GovernorError> {
        let proposer = self.env().caller();

        if _is_valid_description_for_proposer(&description, &proposer) {
            return Err(GovernorError::InvalidDescription);
        }

        let current_timestamp = self.env().block_timestamp();

        {
            let proposer_votes = self.get_votes(proposer, current_timestamp - 1);

            let votes_threshold = self.proposal_threshold();

            if proposer_votes < votes_threshold {
                return Err(GovernorError::InsufficientProposerVotes(proposer.clone(), proposer_votes, votes_threshold));
            }
        }

        let description_hash = Blake2s::new()
            .chain(description.as_bytes())
            .finalize()
            .into();

        let proposal_id = self.hash_proposal(targets.clone(), values.clone(), calldatas.clone(), description_hash.clone());

        if targets.len() != values.len() || targets.len() != calldatas.len() || targets.len() == 0 {
            return Err(GovernorError::InvalidProposalLength(targets.len(), values.len(), calldatas.len()));
        }
        if self.data().proposals.contains_key(&proposal_id) {
            return Err(GovernorError::UnexpectedProposalState(proposal_id.clone(), self.state(proposal_id.clone()).unwrap(), Vec::new()));
        }

        let snapshot = current_timestamp + self.voting_delay();
        let duration = self.voting_period();

        self.data().proposals.insert(&proposal_id, &ProposalCore {
            proposer: proposer.clone(),
            vote_start: snapshot,
            vote_duration: duration,
            executed: false,
            canceled: false,
        });

        self._emit_proposal_created_event(&proposal_id, &proposer, &targets, &values, &calldatas, &description_hash);

        Ok(proposal_id)
    }

    fn execute(&mut self,
               targets: Vec<AccountId>,
               values: Vec<Balance>,
               calldatas: Vec<Vec<u8>>,
               description_hash: Vec<u8>
    ) -> Result<Id, GovernorError> {
        let description_hash = Blake2s::new()
            .chain(description.as_bytes())
            .finalize()
            .into();
        let proposal_id = self.hash_proposal(targets.clone(), values.clone(), calldatas.clone(), description_hash.clone());

        let current_state = self.state(proposal_id.clone()).unwrap();

        if current_state != ProposalState::Succeeded && current_state != ProposalState::Queued {
            return Err(GovernorError::UnexpectedProposalState(proposal_id.clone(), current_state.clone(), Vec::new()));
        }

        let mut proposal = self.data().proposals.get(&proposal_id).unwrap();

        proposal.executed = true;

        self.data().proposals.insert(&proposal_id, &proposal);

        self._emit_proposal_executed_event(&proposal_id);

        self._before_execute(proposal_id.clone(), targets.clone(), values.clone(), calldatas.clone(), description_hash.clone());
        self._execute(proposal_id.clone(), targets.clone(), values.clone(), calldatas.clone(), description_hash.clone());
        self._after_execute(proposal_id.clone(), targets.clone(), values.clone(), calldatas.clone(), description_hash.clone());

        Ok(proposal_id)
    }

    fn cancel(&mut self,
              targets: Vec<AccountId>,
              values: Vec<Balance>,
              calldatas: Vec<Vec<u8>>,
              description_hash: Vec<u8>
    ) -> Result<Id, GovernorError> {
        let description_hash = Blake2s::new()
            .chain(description.as_bytes())
            .finalize()
            .into();
        let proposal_id = self.hash_proposal(targets.clone(), values.clone(), calldatas.clone(), description_hash.clone());

        let current_state = self.state(proposal_id.clone()).unwrap();

        if current_state != ProposalState::Pending {
            return Err(GovernorError::UnexpectedProposalState(
                proposal_id.clone(),
                current_state.clone(),
                _encode_state_bitmap(ProposalState::Pending)
            ));
        }

        if self.env().caller() != self.proposal_proposer(proposal_id.clone()) {
            return Err(GovernorError::OnlyProposer(self.env().caller()));
        }

        return self._cancel(targets.clone(), values.clone(), calldatas.clone(), description_hash.clone());
    }

    fn get_votes(&self, account: AccountId, timestamp: Timestamp) -> u128 {
        self._get_votes(account, timestamp, default_params())
    }

    fn get_votes_with_params(
        &self,
        account: AccountId,
        timestamp: Timestamp,
        params: Vec<u8>
    ) -> u128 {
        self._get_votes(account, timestamp, params)
    }

    fn cast_vote(
        &mut self,
        proposal_id: Id,
        support: bool
    ) -> Result<u128, GovernorError> {
        let voter = self.env().caller();
        self._cast_vote(proposal_id, voter, support, "".to_string(), self.default_params())
    }

    fn cast_vote_with_reason(
        &mut self,
        proposal_id: Id,
        support: bool,
        reason: String
    ) -> Result<Balance, GovernorError> {
        let voter = self.env().caller();
        self._cast_vote(proposal_id, voter, support, reason, self.default_params())
    }

    fn cast_vote_with_reason_and_params(
        &mut self,
        proposal_id: Id,
        support: bool,
        reason: String,
        params: Vec<u8>
    ) -> Result<Balance, GovernorError> {
        let voter = self.env().caller();
        self._cast_vote(proposal_id, voter, support, reason, params)
    }

    fn cast_vote_by_sig(
        &self,
        proposal_id: Id,
        support: bool,
        voter: AccountId,
        signature: Vec<u8>
    ) -> Result<Balance, GovernorError> {
        todo!("cast_vote_by_sig")
    }

    fn cast_vote_with_reason_and_params_by_sig(
        &self,
        proposal_id: Id,
        support: bool,
        voter: AccountId,
        reason: String,
        params: Vec<u8>,
        signature: Vec<u8>
    ) -> Result<Balance, GovernorError> {
        todo!("cast_vote_with_reason_and_params_by_sig")
    }

    //#[openbrush::modifier(only_governance)]
    fn relay(
        &mut self,
        target: AccountId,
        value: Balance,
        data: Vec<u8>
    ) -> Result<(), GovernorError> {
        todo!("relay")
    }

    fn hash_proposal(
        &self,
        targets: Vec<AccountId>,
        values: Vec<Balance>,
        calldatas: Vec<Vec<u8>>,
        description_hash: Vec<u8>
    ) -> Id {
        return Blake2s::new()
            .chain(targets)
            .chain(values)
            .chain(calldatas)
            .chain(description_hash)
            .finalize()
            .into();
    }
}

pub trait Internal {
    fn default_params(&self) -> Vec<u8>;

    fn _execute(&mut self,
                proposal_id: Id,
                targets: Vec<AccountId>,
                values: Vec<Balance>,
                calldatas: Vec<Vec<u8>>,
                description_hash: Vec<u8>
    ) -> Result<Id, GovernorError>;

    fn _before_execute(
        &mut self,
        proposal_id: Id,
        targets: Vec<AccountId>,
        values: Vec<Balance>,
        calldatas: Vec<Vec<u8>>,
        description_hash: Vec<u8>
    ) -> Result<(), GovernorError>;

    fn _after_execute(
        &mut self,
        proposal_id: Id,
        targets: Vec<AccountId>,
        values: Vec<Balance>,
        calldatas: Vec<Vec<u8>>,
        description_hash: Vec<u8>
    ) -> Result<(), GovernorError>;

    fn _cancel(
        targets: Vec<AccountId>,
        values: Vec<Balance>,
        calldatas: Vec<Vec<u8>>,
        description_hash: Vec<u8>
    ) -> Result<Id, GovernorError>;

    fn _cast_vote(
        &mut self,
        proposal_id: Id,
        voter: AccountId,
        support: bool,
        reason: String,
        params: Vec<u8>
    ) -> Result<u128, GovernorError>;

    fn _is_valid_description_for_proposer(
        proposer: AccountId,
        description: String
    ) -> bool;

    fn _encode_state_bitmap(
        &self,
        proposal_state: ProposalState,
    ) -> Vec<u8>;

    fn _executor(&self) -> AccountId;

    fn _try_hex_to_uint(char: char) -> (bool, u8);
}

pub trait InternalImpl: Storage<Data> + Internal {
    fn default_params(&self) -> Vec<u8> {
        Vec::new()
    }

    fn _execute(&mut self,
                proposal_id: Id,
                targets: Vec<AccountId>,
                values: Vec<Balance>,
                calldatas: Vec<Vec<u8>>,
                description_hash: Vec<u8>
    ) -> Result<(), GovernorError> {
        for i in 0..targets.len() {
            let target = targets[i];
            let value = values[i];
            let calldata = calldatas[i].clone();
            let success = metod_call_dry_run!(target, , calldata);
        }
        Ok(())
    }

    fn _before_execute(
        &mut self,
        proposal_id: Id,
        targets: Vec<AccountId>,
        values: Vec<Balance>,
        calldatas: Vec<Vec<u8>>,
        description_hash: Vec<u8>
    ) -> Result<(), GovernorError> {
        if self._executor() != self.env().account_id() {
            for i in 0..targets.len() {
                if targets[i] == self.env().account_id() {
                    self.data().governance_call.push(
                        Blake2s::new()
                            .chain(calldatas[i].as_bytes())
                            .finalize()
                            .into()
                    );
                }
            }
        }
        Ok(())
    }


    fn _after_execute(
        &mut self,
        proposal_id: Id,
        targets: Vec<AccountId>,
        values: Vec<Balance>,
        calldatas: Vec<Vec<u8>>,
        description_hash: Vec<u8>
    ) -> Result<(), GovernorError> {
        if self._executor() != self.env().account_id() {
            if !governance_call.is_empty() {
                self.data().governance_call.clear();
            }
        }
        Ok(())
    }

    fn _cancel(
        &mut self,
        targets: Vec<AccountId>,
        values: Vec<Balance>,
        calldatas: Vec<Vec<u8>>,
        description_hash: Vec<u8>
    ) -> Result<Id, GovernorError> {
        let proposal_id = self.hash_proposal(targets, values, calldatas, description_hash);

        let current_state = self.state(&proposal_id);

        let forbiden_states = self._encode_state_bitmap(ProposalState::Canceled)
            | self._encode_state_bitmap(ProposalState::Expired)
            | self._encode_state_bitmap(ProposalState::Executed);

        let _all_proposal_states_bitmap = self._encode_state_bitmap(ProposalState::Pending)
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
                _all_proposal_states_bitmap ^ forbiden_states
            ));
        }

        let mut proposal = self.data().proposals.get(&proposal_id).unwrap();
        proposal.canceled = true;

        self._emit_proposal_canceled_event(proposal_id);

        proposal_id
    }

    fn _cast_vote(
        &mut self,
        proposal_id: Id,
        voter: AccountId,
        support: bool,
        reason: String,
        params: Vec<u8>
    ) -> Result<u128, GovernorError> {
        todo!("cast_vote")
    }

    fn _is_valid_description_for_proposer(
        proposer: AccountId,
        description: String
    ) -> bool {
        todo!("is_valid_description_for_proposer")
    }

    fn _encode_state_bitmap(
        &self,
        proposal_state: ProposalState,
    ) -> Vec<u8> {
        todo!("encode_state_bitmap")
    }

    fn _executor(&self) -> AccountId {
        self.env().caller()
    }

    fn _try_hex_to_uint(char: char) -> (bool, u8) {
        todo!("try_hex_to_uint")
    }
}