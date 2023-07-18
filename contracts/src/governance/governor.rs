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
        Selector
    },
};

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    #[lazy]
    pub name: String,
    pub proposals: Mapping<Id, ProposalCore>,
    pub governance_call: Vec<Selector>,
}

#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
#[derive(PartialEq, Eq, scale::Encode, scale::Decode)]
pub struct ProposalCore {
    proposer: AccountId,
    vote_start: Timestamp,
    vote_duration: Timestamp,
    executed: bool,
    canceled: bool,
}

pub trait GovernanceImpl: Storage<Data> + Internal {
    fn name(&self) -> String {
        self._name()
    }

    fn state(&self, id: Id) -> ProposalState {
        self._state(id)
    }

    fn proposal_threshold(&self, id: Id) -> Balance {
        self._proposal_threshold(id)
    }

    fn proposal_snapshot(&self, id: Id) -> Id {
        self._proposal_snapshot(id)
    }

    fn proposal_deadline(&self, id: Id) -> Timestamp {
        self._proposal_deadline(id)
    }

    fn proposal_proposer(&self, id: Id) -> AccountId {
        self._proposal_proposer(id)
    }

    fn propose(
        &mut self,
        targets: Vec<AccountId>, 
        values: Vec<Balance>,  
        calldatas: Vec<Vec<u8>>, 
        description: String) -> Id {
        self._propose(targets, values, calldatas, description)
    }

    fn execute(&mut self, id: Id) -> Result<(), GovernanceError> {
        self._execute(id)
    }

    fn cancel(&mut self, id: Id) -> Result<(), GovernanceError> {
        self._cancel(id)
    }

    fn get_votes(&mut self, account: AccountId, timepoint: Timestamp) -> u128 {
        self._get_votes(id)
    }

    fn get_votes_with_params(
        &mut self,
        account: AccountId,
        timepoint: Timestamp,
        params: Vec<u8>
    ) -> u128 {
        self._get_votes_with_params(id, account)
    }

    fn cast_vote(
        &mut self,
        proposal_id: Id,
        support: bool
    ) -> Balance {
        self._cast_vote(id, support)
    }

    fn cast_vote_with_reason(
        &mut self,
        proposal_id: Id,
        support: bool,
        reason: String
    ) -> Balance {
        self._cast_vote_with_reason(id, support, reason)
    }

    fn cast_vote_with_reason_and_params(
        &mut self,
        proposal_id: Id,
        support: bool, 
        reason: String,
        params: Vec<u8>
    ) -> Balance {
        self._cast_vote_with_reason_and_params(id, support, reason, account)
    }

    fn cast_vote_by_sig(
        &mut self,
        proposal_id: Id,
        support: bool,
        voter: AccountId,
        signature: Vec<u8>
    ) -> Balance {
        self._cast_vote_by_sig(id, support, v, r, s)
    }

    fn cast_vote_with_reason_and_params_by_sig(
        &mut self,
        proposal_id: Id,
        support: bool,
        voter: AccountId,
        reason: String,
        params: Vec<u8>,
        signature: Vec<u8>
    ) -> Balance {
        self._cast_vote_with_reason_and_params_by_sig(id, support, voter, reason, params, signature)
    }

    fn relay(
        &mut self,
        target: AccountId,
        value: Balance,
        data: Vec<u8>
    ) -> Balance {
        self._relay(target, value, data)
    }

    fn hash_proposal(
        &mut self,
        targets: Vec<AccountId>,
        values: Vec<Balance>,
        calldatas: Vec<Vec<u8>>,
        description_hash: Vec<u8>
    ) -> Vec<u8> {
        self._hash_proposal(targets, values, calldatas, description_hash)
    }
}

pub trait Internal {
    fn _name(&self) -> String;

    fn _state(&self, id: Id) -> ProposalState;

    fn _proposal_threshold(&self, id: Id) -> Balance;

    fn _proposal_snapshot(&self, id: Id) -> Id;

    fn _proposal_deadline(&self, id: Id) -> Timestamp;

    fn _proposal_proposer(&self, id: Id) -> AccountId;

    fn _propose(
        &mut self,
        targets: Vec<AccountId>, 
        values: Vec<Balance>,  
        calldatas: Vec<Vec<u8>>, 
        description: String) -> Id;

    fn _execute(&mut self, id: Id) -> Result<(), GovernanceError>;

    fn _cancel(&mut self, id: Id) -> Result<(), GovernanceError>;

    fn _get_votes(&mut self, account: AccountId, timepoint: Timestamp) -> u128;

    fn _get_votes_with_params(
        &mut self,
        account: AccountId,
        timepoint: Timestamp,
        params: Vec<u8>
    ) -> u128;

    fn _cast_vote(
        &mut self,
        proposal_id: Id,
        support: bool
    ) -> Balance;

    fn _cast_vote_with_reason(
        &mut self,
        proposal_id: Id,
        support: bool,
        reason: String
    ) -> Balance;

    fn _cast_vote_with_reason_and_params(
        &mut self,
        proposal_id: Id,
        support: bool, 
        reason: String,
        params: Vec<u8>
    ) -> Balance;

    fn _cast_vote_by_sig(
        &mut self,
        proposal_id: Id,
        support: bool,
        voter: AccountId,
        signature: Vec<u8>
    ) -> Balance;

    fn _cast_vote_with_reason_and_params_by_sig(
        &mut self,
        proposal_id: Id,
        support: bool,
        voter: AccountId,
        reason: String,
        params: Vec<u8>,
        signature: Vec<u8>
    ) -> Balance;

    fn _relay(
        &mut self,
        target: AccountId,
        value: Balance,
        data: Vec<u8>
    ) -> Balance;

    fn _hash_proposal(
        &mut self,
        targets: Vec<AccountId>,
        values: Vec<Balance>,
        calldatas: Vec<Vec<u8>>,
        description_hash: Vec<u8>
    ) -> Vec<u8>;
}

pub trait InternalImpl: Storage<Data> + Internal {
    fn _name(&self) -> String {
        self.data().name.clone()
    }

    fn _state(&self, id: Id) -> ProposalState {
        let proposal = self.data().proposals.get(&id).unwrap();
        if proposal.canceled {
            return ProposalState::Canceled;
        }
        if proposal.executed {
            return ProposalState::Executed;
        }
        if self.env().block_timestamp() < proposal.vote_start {
            return ProposalState::Pending;
        }
        if self.env().block_timestamp() < proposal.vote_start + proposal.vote_duration {
            return ProposalState::Active;
        }
        if self.env().block_timestamp() < proposal.vote_start + proposal.vote_duration + self.env().voting_delay() {
            return ProposalState::Succeeded;
        }
        if self.env().block_timestamp() < proposal.vote_start + proposal.vote_duration + self.env().voting_delay() + self.env().voting_period() {
            return ProposalState::Queued;
        }
        if self.env().block_timestamp() < proposal.vote_start + proposal.vote_duration + self.env().voting_delay() + self.env().voting_period() + self.env().execution_delay() {
            return ProposalState::Expired;
        }
        ProposalState::Executed
    }

    fn _proposal_threshold(&self, id: Id) -> Balance {
        let proposal = self.data().proposals.get(&id).unwrap();
        let total_supply = self.total_supply();
        let threshold = total_supply * self.env().quorum_vote_threshold() / 100;
        threshold
    }

    fn _proposal_snapshot(&self, id: Id) -> Id {
        let proposal = self.data().proposals.get(&id).unwrap();
        proposal.vote_start
    }

    fn _proposal_deadline(&self, id: Id) -> Timestamp {
        let proposal = self.data().proposals.get(&id).unwrap();
        proposal.vote_start + proposal.vote_duration
    }

    fn _proposal_proposer(&self, id: Id) -> AccountId {
        let proposal = self.data().proposals.get(&id).unwrap();
        proposal.proposer
    }

    fn _propose(
        &mut self,
        targets: Vec<AccountId>, 
        values: Vec<Balance>,  
        calldatas: Vec<Vec<u8>>, 
        description: String) -> Id {


    }

}