#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(AccessControl)]
#[openbrush::contract]
pub mod my_timelock_controller {
    use ink::prelude::vec::Vec;
    use openbrush::{
        contracts::{
            extensions::{
                governor_counting,
                governor_counting::*,
                governor_votes,
                governor_votes::*,
            },
            governor,
            governor::*,
            nonces,
            nonces::{
                NoncesError,
                NoncesImpl,
            },
            traits::{
                errors::GovernanceError,
                governance::{
                    extensions::{
                        governor_counting::*,
                        governor_votes::*,
                    },
                    governor::*,
                    HashType,
                    ProposalId,
                    ProposalState,
                    Transaction,
                },
                types::SignatureType,
                utils::nonces::*,
            },
        },
        traits::{
            Storage,
            String,
        },
    };

    const MAGIC_NUMBER: Balance = 42;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        access_control: access_control::Data,
        #[storage_field]
        governor: governor::Data,
        #[storage_field]
        governor_counting: governor_counting::Data,
        #[storage_field]
        governor_votes: governor_votes::Data,
        #[storage_field]
        nonces: nonces::Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(min_delay: Timestamp, proposers: Vec<AccountId>, executors: Vec<AccountId>) -> Self {
            let mut instance = Self::default();

            let caller = Self::env().caller();
            access_control::Internal::_init_with_admin(&mut instance, Some(caller));

            instance
        }
    }

    impl CountingInternal for Contract {}
    impl VotesInternal for Contract {
        fn _get_voting_units(&self, account: &AccountId) -> Balance {
            MAGIC_NUMBER
        }
    }

    impl GovernorCountingImpl for Contract {}
    impl GovernorVotesImpl for Contract {}

    impl VotesEvents for Contract {}
    impl GovernorEvents for Contract {}

    impl GovernorInternal for Contract {
        fn _voting_delay(&self) -> u64 {
            // VotesInternal::_voting_delay(self)
            todo!()
        }

        fn _voting_period(&self) -> u64 {
            // VotesInternal::_voting_period(self)
            todo!()
        }

        fn _quorum(&self, time_point: Timestamp) -> u128 {
            todo!()
        }

        fn _quorum_reached(&self, proposal_id: ProposalId) -> bool {
            todo!()
        }

        fn _vote_succeeded(&self, proposal_id: ProposalId) -> bool {
            todo!()
        }

        fn _get_votes(&self, account: AccountId, time_point: Timestamp, params: Vec<u8>) -> u128 {
            todo!()
        }

        fn _count_vote(
            &mut self,
            proposal_id: ProposalId,
            account: AccountId,
            support: u8,
            weight: u128,
            params: Vec<u8>,
        ) -> Result<(), GovernanceError> {
            todo!()
        }
    }

    impl GovernorImpl for Contract {}

    impl NoncesImpl for Contract {}

    impl Nonces for Contract {
        #[ink(message)]
        fn nonces(&self, account: AccountId) -> u128 {
            NoncesImpl::nonces(self, &account)
        }
    }

    impl GovernorCounting for Contract {
        #[ink(message)]
        fn counting_mode(&self) -> String {
            GovernorCountingImpl::counting_mode(self)
        }

        #[ink(message)]
        fn has_voted(&self, proposal_id: ProposalId, account: AccountId) -> bool {
            GovernorCountingImpl::has_voted(self, proposal_id, account)
        }

        #[ink(message)]
        fn proposal_votes(&self, proposal_id: ProposalId) -> Result<(Balance, Balance, Balance), GovernanceError> {
            GovernorCountingImpl::proposal_votes(self, proposal_id)
        }
    }

    impl Governor for Contract {
        #[ink(message)]
        fn hash_proposal(
            &self,
            transactions: Vec<Transaction>,
            description_hash: HashType,
        ) -> Result<HashType, GovernanceError> {
            GovernorImpl::hash_proposal(self, transactions, description_hash)
        }

        #[ink(message)]
        fn state(&self, proposal_id: ProposalId) -> Result<ProposalState, GovernanceError> {
            GovernorImpl::state(self, proposal_id)
        }

        #[ink(message)]
        fn proposal_snapshot(&self, proposal_id: ProposalId) -> Result<Timestamp, GovernanceError> {
            GovernorImpl::proposal_snapshot(self, proposal_id)
        }

        #[ink(message)]
        fn proposal_deadline(&self, proposal_id: ProposalId) -> Result<Timestamp, GovernanceError> {
            GovernorImpl::proposal_deadline(self, proposal_id)
        }

        #[ink(message)]
        fn proposal_proposer(&self, proposal_id: ProposalId) -> Result<AccountId, GovernanceError> {
            GovernorImpl::proposal_proposer(self, proposal_id)
        }

        #[ink(message)]
        fn voting_delay(&self) -> u64 {
            GovernorImpl::voting_delay(self)
        }

        #[ink(message)]
        fn voting_period(&self) -> u64 {
            GovernorImpl::voting_period(self)
        }

        #[ink(message)]
        fn quorum(&self, time_point: Timestamp) -> u128 {
            GovernorImpl::quorum(self, time_point)
        }

        #[ink(message)]
        fn get_votes(&self, account: AccountId, time_point: Timestamp) -> u128 {
            GovernorImpl::get_votes(self, account, time_point)
        }

        #[ink(message)]
        fn get_votes_with_params(&self, account: AccountId, time_point: Timestamp, params: Vec<u8>) -> u128 {
            GovernorImpl::get_votes_with_params(self, account, time_point, params)
        }

        #[ink(message)]
        fn has_voted(&self, proposal_id: ProposalId, account: AccountId) -> bool {
            todo!()
        }

        #[ink(message)]
        fn propose(
            &mut self,
            transactions: Vec<Transaction>,
            description: String,
        ) -> Result<ProposalId, GovernanceError> {
            GovernorImpl::propose(self, transactions, description)
        }

        #[ink(message)]
        fn execute(
            &mut self,
            transactions: Vec<Transaction>,
            description_hash: HashType,
        ) -> Result<ProposalId, GovernanceError> {
            GovernorImpl::execute(self, transactions, description_hash)
        }

        #[ink(message)]
        fn cancel(
            &mut self,
            transaction: Vec<Transaction>,
            description_hash: HashType,
        ) -> Result<ProposalId, GovernanceError> {
            GovernorImpl::cancel(self, transaction, description_hash)
        }

        #[ink(message)]
        fn cast_vote(&mut self, proposal_id: ProposalId, support: u8) -> Result<Balance, GovernanceError> {
            GovernorImpl::cast_vote(self, proposal_id, support)
        }

        #[ink(message)]
        fn cast_vote_with_reason(
            &mut self,
            proposal_id: ProposalId,
            support: u8,
            reason: String,
        ) -> Result<Balance, GovernanceError> {
            GovernorImpl::cast_vote_with_reason(self, proposal_id, support, reason)
        }

        #[ink(message)]
        fn cast_vote_with_reason_and_params(
            &mut self,
            proposal_id: ProposalId,
            support: u8,
            reason: String,
            params: Vec<u8>,
        ) -> Result<Balance, GovernanceError> {
            GovernorImpl::cast_vote_with_reason_and_params(self, proposal_id, support, reason, params)
        }

        #[ink(message)]
        fn cast_vote_with_signature(
            &mut self,
            proposal_id: ProposalId,
            support: u8,
            reason: String,
            signature: SignatureType,
        ) -> Result<Balance, GovernanceError> {
            GovernorImpl::cast_vote_with_signature(self, proposal_id, support, reason, signature)
        }

        #[ink(message)]
        fn cast_vote_with_signature_and_params(
            &mut self,
            proposal_id: ProposalId,
            support: u8,
            reason: String,
            signature: SignatureType,
            params: Vec<u8>,
        ) -> Result<Balance, GovernanceError> {
            GovernorImpl::cast_vote_with_signature_and_params(self, proposal_id, support, reason, signature, params)
        }
    }

    impl GovernorVotes for Contract {
        #[ink(message)]
        fn clock(&self) -> u64 {
            unimplemented!("clock")
        }

        #[ink(message)]
        fn get_votes(&self, account: AccountId) -> Result<Balance, GovernanceError> {
            GovernorVotesImpl::get_votes(self, account)
        }

        #[ink(message)]
        fn get_past_votes(&self, account: AccountId, timestamp: Timestamp) -> Result<Balance, GovernanceError> {
            GovernorVotesImpl::get_past_votes(self, account, timestamp)
        }

        #[ink(message)]
        fn get_past_total_supply(&self, timestamp: Timestamp) -> Result<Balance, GovernanceError> {
            GovernorVotesImpl::get_past_total_supply(self, timestamp)
        }

        #[ink(message)]
        fn delegates(&mut self, delegator: AccountId) -> AccountId {
            GovernorVotesImpl::delegates(self, delegator)
        }

        #[ink(message)]
        fn delegate(&mut self, delegatee: AccountId) -> Result<(), GovernanceError> {
            GovernorVotesImpl::delegate(self, delegatee)
        }

        #[ink(message)]
        fn delegate_by_signature(
            &mut self,
            signer: AccountId,
            delegatee: AccountId,
            nonce: u128,
            expiry: Timestamp,
            signature: SignatureType,
        ) -> Result<(), GovernanceError> {
            GovernorVotesImpl::delegate_by_signature(self, signer, delegatee, nonce, expiry, signature)
        }
    }
}
