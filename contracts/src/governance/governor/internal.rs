use crate::{
    governance::governor::{
        Data,
        GovernorEvents,
    },
    traits::{
        errors::governance::GovernanceError,
        governance::{
            CancellationStatus,
            ExecutionStatus,
            HashType,
            ProposalCore,
            ProposalId,
            ProposalState,
            Transaction,
            ALL_PROPOSAL_STATES,
        },
    },
    utils::crypto,
};
use ink::{
    env::{
        call::{
            build_call,
            ExecutionInput,
            Selector,
        },
        DefaultEnvironment,
    },
    prelude::{
        borrow::ToOwned,
        vec::Vec,
    },
};
use openbrush::traits::{
    AccountId,
    Balance,
    Storage,
    String,
    Timestamp,
};
use scale::Encode;

pub trait GovernorInternal: Storage<Data> + GovernorEvents {
    fn _hash_proposal(
        &self,
        transactions: Vec<Transaction>,
        description_hash: HashType,
    ) -> Result<HashType, GovernanceError> {
        let message = (transactions, description_hash).encode();

        crypto::hash_message(message.as_slice()).map_err(|err| err.into())
    }

    fn _state(&self, _proposal_id: ProposalId) -> Result<ProposalState, GovernanceError> {
        let current_time = Self::env().block_timestamp();

        let proposal = self.data().proposals.get(&_proposal_id).unwrap_or_default();

        if proposal.executed == ExecutionStatus::Executed {
            return Ok(ProposalState::Executed)
        }

        if proposal.cancelled == CancellationStatus::Cancelled {
            return Ok(ProposalState::Cancelled)
        }

        let snapshot = self._proposal_snapshot(_proposal_id.clone())?;

        if snapshot == 0 {
            return Err(GovernanceError::NonexistentProposal(_proposal_id.clone()))
        }

        let deadline = proposal.deadline()?;

        if deadline >= current_time {
            return Ok(ProposalState::Active)
        }

        if self._vote_succeeded(_proposal_id.clone()) && self._quorum_reached(_proposal_id.clone()) {
            Ok(ProposalState::Succeeded)
        } else {
            Ok(ProposalState::Defeated)
        }
    }

    fn _proposal_snapshot(&self, proposal_id: ProposalId) -> Result<Timestamp, GovernanceError> {
        Ok(self
            .data()
            .proposals
            .get(&proposal_id)
            .ok_or(GovernanceError::ProposalNotFound)?
            .vote_start)
    }

    /// Implement this manually
    fn _voting_delay(&self) -> u64;

    /// Implement this manually
    fn _voting_period(&self) -> u64;

    /// Implement this manually
    fn _quorum(&self, time_point: Timestamp) -> u128;

    /// Implement this manually
    fn _quorum_reached(&self, proposal_id: ProposalId) -> bool;

    /// Implement this manually
    fn _vote_succeeded(&self, proposal_id: ProposalId) -> bool;

    /// Implement this manually
    fn _get_votes(&self, account: AccountId, time_point: Timestamp, params: Vec<u8>) -> u128;

    /// Implement this manually
    fn _count_vote(
        &mut self,
        proposal_id: ProposalId,
        account: AccountId,
        support: u8,
        weight: u128,
        params: Vec<u8>,
    ) -> Result<(), GovernanceError>;

    fn _default_params(&self) -> Vec<u8> {
        Vec::new()
    }

    fn _execute(&mut self, transactions: Vec<Transaction>, _description_hash: HashType) -> Result<(), GovernanceError> {
        for tx in transactions.iter() {
            let is_ok = build_call::<DefaultEnvironment>()
                .call(tx.destination.clone())
                .gas_limit(tx.gas_limit.clone())
                .transferred_value(tx.transferred_value.clone())
                .exec_input(ExecutionInput::new(Selector::new(tx.selector.clone())).push_arg(tx.input.clone()))
                .returns::<bool>()
                .invoke();

            if !is_ok {
                return Err(GovernanceError::ExecutionFailed(tx.clone()))
            }
        }

        Ok(())
    }

    fn _before_execute(
        &mut self,
        transactions: Vec<Transaction>,
        _description_hash: HashType,
    ) -> Result<(), GovernanceError> {
        let self_address = Self::env().account_id();
        let executor = self._executor();
        if executor != self_address {
            for tx in transactions.iter() {
                if tx.destination == self_address {
                    let mut governance_call = self.data().governance_call.get_or_default();
                    governance_call.push(tx.clone());
                    self.data().governance_call.set(&governance_call);
                }
            }
        }
        Ok(())
    }

    fn _after_execute(
        &mut self,
        _transactions: Vec<Transaction>,
        _description_hash: HashType,
    ) -> Result<(), GovernanceError> {
        if self._executor() != Self::env().account_id() && !self.data().governance_call.get_or_default().is_empty() {
            self.data().governance_call.set(&Vec::new());
        }

        Ok(())
    }

    fn _cancel(
        &mut self,
        transactions: Vec<Transaction>,
        description_hash: HashType,
    ) -> Result<ProposalId, GovernanceError> {
        let proposal_id = self._hash_proposal(transactions, description_hash)?;
        let current_state = self._state(proposal_id.clone())?;

        let forbidden_states =
            ProposalState::Cancelled.u128() | ProposalState::Executed.u128() | ProposalState::Expired.u128();

        if forbidden_states.clone() & current_state.clone().u128() != 0 {
            return Err(GovernanceError::UnexpectedProposalState(
                proposal_id.clone(),
                current_state,
                ALL_PROPOSAL_STATES ^ forbidden_states,
            ))
        }

        let proposal = self.data().proposals.get(&proposal_id).unwrap_or_default();

        self.data().proposals.insert(
            &proposal_id,
            &ProposalCore {
                cancelled: CancellationStatus::Cancelled,
                ..proposal
            },
        );

        self.emit_proposal_cancelled(proposal_id.clone());

        Ok(proposal_id)
    }

    fn _cast_vote(
        &mut self,
        proposal_id: ProposalId,
        account: AccountId,
        support: u8,
        reason: String,
    ) -> Result<Balance, GovernanceError> {
        self._cast_vote_with_params(proposal_id, account, support, reason, self._default_params())
    }

    fn _proposal_proposer(&self, proposal_id: ProposalId) -> Result<AccountId, GovernanceError> {
        Ok(self
            .data()
            .proposals
            .get(&proposal_id)
            .ok_or(GovernanceError::ProposalNotFound)?
            .proposer)
    }

    fn _cast_vote_with_params(
        &mut self,
        proposal_id: ProposalId,
        account: AccountId,
        support: u8,
        reason: String,
        params: Vec<u8>,
    ) -> Result<Balance, GovernanceError> {
        let current_state = self._state(proposal_id.clone())?;

        if current_state != ProposalState::Active {
            return Err(GovernanceError::UnexpectedProposalState(
                proposal_id.clone(),
                current_state,
                ProposalState::Active.u128(),
            ))
        }

        let snapshot = self._proposal_snapshot(proposal_id.clone())?;
        let weight = self._get_votes(account.clone(), snapshot, params.clone());

        self._count_vote(
            proposal_id.clone(),
            account.clone(),
            support.clone(),
            weight.clone(),
            params.clone(),
        )?;

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

    fn _executor(&self) -> AccountId {
        Self::env().account_id()
    }

    fn _is_valid_description_for_proposer(
        &self,
        proposer: AccountId,
        description: String,
    ) -> Result<bool, GovernanceError> {
        let proposer_bytes: &[u8; 32] = proposer.as_ref();
        let proposer_str =
            core::str::from_utf8(proposer_bytes.as_slice()).map_err(|_| GovernanceError::ErrorParsingDescription)?;

        let result = String::from("#proposer={}".to_owned() + proposer_str);

        Ok(result == description)
    }

    fn _proposal_threshold(&self) -> u128 {
        0
    }

    fn _hash_description(&self, description: String) -> Result<HashType, GovernanceError> {
        Ok(crypto::hash_message(description.as_bytes())?)
    }
}
