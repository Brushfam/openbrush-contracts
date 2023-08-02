use crate::{
    governance::governor::{
        Data,
        GovernorEvents,
        GovernorInternal,
    },
    traits::{
        errors::governance::GovernanceError,
        governance::{
            ExecutionStatus,
            HashType,
            ProposalCore,
            ProposalId,
            ProposalState,
            Transaction,
        },
        types::SignatureType,
        utils::nonces::Nonces,
    },
    utils::crypto,
};
use ink::prelude::vec::Vec;
use openbrush::traits::{
    AccountId,
    Balance,
    Storage,
    String,
    Timestamp,
};
use scale::Encode;

pub trait GovernorImpl: Storage<Data> + GovernorEvents + GovernorInternal + Nonces {
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

    fn proposal_threshold(&self) -> u128 {
        self._proposal_threshold()
    }

    fn proposal_snapshot(&self, proposal_id: ProposalId) -> Result<Timestamp, GovernanceError> {
        self._proposal_snapshot(proposal_id)
    }

    fn proposal_deadline(&self, proposal_id: ProposalId) -> Result<Timestamp, GovernanceError> {
        self.data()
            .proposals
            .get(&proposal_id)
            .ok_or(GovernanceError::ProposalNotFound)?
            .deadline()
    }

    fn proposal_proposer(&self, proposal_id: ProposalId) -> Result<AccountId, GovernanceError> {
        self._proposal_proposer(proposal_id)
    }

    fn voting_delay(&self) -> u64 {
        self._voting_delay()
    }

    fn voting_period(&self) -> u64 {
        self._voting_period()
    }

    fn quorum(&self, time_point: Timestamp) -> u128 {
        self._quorum(time_point)
    }

    fn get_votes(&self, account: AccountId, time_point: Timestamp) -> u128 {
        self._get_votes(account, time_point, Vec::new())
    }

    fn get_votes_with_params(&self, account: AccountId, time_point: Timestamp, params: Vec<u8>) -> u128 {
        self._get_votes(account, time_point, params)
    }

    fn propose(&mut self, transactions: Vec<Transaction>, description: String) -> Result<ProposalId, GovernanceError> {
        let proposer = Self::env().caller();

        if !self._is_valid_description_for_proposer(proposer, description.clone())? {
            return Err(GovernanceError::ProposerRestricted(proposer))
        }

        let current_timestamp = Self::env().block_timestamp();

        let proposer_votes = self.get_votes(proposer, current_timestamp.clone());

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

        if transactions.len() == 0 {
            return Err(GovernanceError::ZeroProposalLength)
        }

        if self.data().proposals.contains(&proposal_id) {
            return Err(GovernanceError::ProposalAlreadyExists)
        }

        let snapshot = current_timestamp + self.voting_delay();
        let duration = self.voting_period();

        self.data().proposals.insert(
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
            .data()
            .proposals
            .get(&proposal_id)
            .ok_or(GovernanceError::ProposalNotFound)?;

        self.data().proposals.insert(
            &proposal_id,
            &ProposalCore {
                executed: ExecutionStatus::Executed,
                ..proposal
            },
        );

        self.emit_proposal_executed(proposal_id.clone());

        self._before_execute(transactions.clone(), description_hash.clone())?;

        self._execute(transactions.clone(), description_hash.clone())?;

        self._after_execute(transactions.clone(), description_hash.clone())?;

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

        if current_state != ProposalState::Active {
            return Err(GovernanceError::UnexpectedProposalState(
                proposal_id.clone(),
                current_state,
                ProposalState::Active.u128(),
            ))
        }

        if caller != self.proposal_proposer(proposal_id.clone())? {
            return Err(GovernanceError::OnlyProposer(caller))
        }

        self._cancel(transactions, description_hash)
    }

    fn cast_vote(&mut self, proposal_id: ProposalId, support: u8) -> Result<Balance, GovernanceError> {
        let voter = Self::env().caller();

        self._cast_vote(proposal_id, voter, support, String::new())
    }

    fn cast_vote_with_reason(
        &mut self,
        proposal_id: ProposalId,
        support: u8,
        reason: String,
    ) -> Result<Balance, GovernanceError> {
        let voter = Self::env().caller();

        self._cast_vote_with_params(proposal_id, voter, support, reason, Vec::new())
    }

    fn cast_vote_with_reason_and_params(
        &mut self,
        proposal_id: ProposalId,
        support: u8,
        reason: String,
        params: Vec<u8>,
    ) -> Result<Balance, GovernanceError> {
        let voter = Self::env().caller();

        self._cast_vote_with_params(proposal_id, voter, support, reason, params)
    }

    fn cast_vote_with_signature(
        &mut self,
        proposal_id: ProposalId,
        support: u8,
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
        support: u8,
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
}
