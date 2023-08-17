#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::contract]
pub mod my_governor {
    use ink::prelude::vec::Vec;
    use openbrush::{
        contracts::{
            extensions::{
                governor_counting,
                governor_counting::*,
                governor_quorum,
                governor_quorum::*,
                governor_settings,
                governor_settings::*,
                governor_votes,
                governor_votes::*,
            },
            governance::utils::votes,
            governor,
            governor::*,
            nonces,
            nonces::NoncesImpl,
            traits::{
                errors::GovernanceError,
                governance::{
                    extensions::{
                        governor_counting::*,
                        governor_quorum::*,
                        governor_settings::*,
                    },
                    governor::*,
                    HashType,
                    ProposalId,
                    ProposalState,
                    ProposalVote,
                    Transaction,
                    VoteType,
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

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        governor: governor::Data,
        #[storage_field]
        governor_counting: governor_counting::Data,
        #[storage_field]
        governor_votes: governor_votes::Data,
        #[storage_field]
        nonces: nonces::Data,
        #[storage_field]
        settings: governor_settings::Data,
        #[storage_field]
        quorum: governor_quorum::Data,
        mock_timestamp: Timestamp,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(
            token: AccountId,
            voting_delay: u64,
            voting_period: u64,
            proposal_threshold: u128,
            numerator: u128,
        ) -> Self {
            let mut instance = Self::default();

            let caller = Self::env().caller();

            instance._init_governor_votes(token).unwrap();
            instance
                ._init_governor_settings(voting_delay, voting_period, proposal_threshold)
                .unwrap();
            instance._init_quorum_numerator(numerator).unwrap();
            instance.mock_timestamp = Self::env().block_timestamp();

            instance
        }

        #[ink(message)]
        pub fn block_timestamp(&self) -> Timestamp {
            self.mock_timestamp
        }

        #[ink(message)]
        pub fn set_block_timestamp(&mut self, timestamp: Timestamp) {
            self.mock_timestamp = timestamp;
        }

        #[ink(message)]
        pub fn increase_block_timestamp(&mut self, timestamp: Timestamp) {
            self.mock_timestamp += timestamp;
        }

        #[ink(message)]
        pub fn _count_vote(
            &mut self,
            proposal_id: ProposalId,
            account: AccountId,
            support: VoteType,
            weight: Balance,
        ) -> Result<(), GovernanceError> {
            CountingInternal::_count_vote(self, proposal_id, account, support, weight)
        }

        #[ink(message)]
        pub fn _get_votes(
            &mut self,
            account: AccountId,
            timepoint: Timestamp,
            _params: Vec<u8>,
        ) -> Result<Balance, GovernanceError> {
            GovernorVotesInternal::_get_votes(self, account, timepoint, _params)
        }
    }

    impl TimestampProvider for Contract {
        fn block_timestamp(&self) -> Timestamp {
            self.mock_timestamp
        }
    }
    impl NoncesImpl for Contract {}

    impl GovernorSettingsEvents for Contract {}
    impl GovernorSettingsInternal for Contract {}
    impl GovernorSettingsImpl for Contract {}

    impl GovernorVotesInternal for Contract {}

    impl QuorumEvents for Contract {}
    impl QuorumImpl for Contract {}

    impl GovernorStorageGetters for Contract {}

    impl GovernorCountingImpl for Contract {}
    impl CountingInternal for Contract {}

    impl GovernorInternal for Contract {}
    impl GovernorEvents for Contract {}
    impl GovernorImpl for Contract {}

    impl GovernorCounting for Contract {
        #[ink(message)]
        fn has_voted(&self, proposal_id: ProposalId, account: AccountId) -> bool {
            GovernorCountingImpl::has_voted(self, proposal_id, account)
        }

        #[ink(message)]
        fn proposal_votes(&self, proposal_id: ProposalId) -> Result<ProposalVote, GovernanceError> {
            GovernorCountingImpl::proposal_votes(self, proposal_id)
        }
    }

    impl Quorum for Contract {
        #[ink(message)]
        fn quorum_numerator(&self) -> u128 {
            QuorumImpl::quorum_numerator(self)
        }

        #[ink(message)]
        fn quorum_numerator_at(&self, time_point: Timestamp) -> u128 {
            QuorumImpl::quorum_numerator_at(self, time_point)
        }

        #[ink(message)]
        fn quorum_denominator(&self) -> u128 {
            QuorumImpl::quorum_denominator(self)
        }

        #[ink(message)]
        fn quorum(&self, time_point: Timestamp) -> Result<u128, GovernanceError> {
            QuorumImpl::quorum(self, time_point)
        }

        #[ink(message)]
        fn update_quorum_numerator(&mut self, numerator: u128) -> Result<(), GovernanceError> {
            QuorumImpl::update_quorum_numerator(self, numerator)
        }
    }

    impl GovernorSettings for Contract {
        #[ink(message)]
        fn set_voting_delay(&mut self, new_voting_delay: u64) -> Result<(), GovernanceError> {
            GovernorSettingsImpl::set_voting_delay(self, new_voting_delay)
        }

        #[ink(message)]
        fn set_voting_period(&mut self, new_voting_period: u64) -> Result<(), GovernanceError> {
            GovernorSettingsImpl::set_voting_period(self, new_voting_period)
        }

        #[ink(message)]
        fn set_proposal_threshold(&mut self, new_proposal_threshold: u128) -> Result<(), GovernanceError> {
            GovernorSettingsImpl::set_proposal_threshold(self, new_proposal_threshold)
        }

        #[ink(message)]
        fn voting_delay(&self) -> u64 {
            GovernorSettingsImpl::voting_delay(self)
        }

        #[ink(message)]
        fn voting_period(&self) -> u64 {
            GovernorSettingsImpl::voting_period(self)
        }

        #[ink(message)]
        fn proposal_threshold(&self) -> u128 {
            GovernorSettingsImpl::proposal_threshold(self)
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
        fn get_votes_with_params(
            &mut self,
            account: AccountId,
            time_point: Timestamp,
            params: Vec<u8>,
        ) -> Result<u128, GovernanceError> {
            GovernorImpl::get_votes_with_params(self, account, time_point, params)
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
            transactions: Vec<Transaction>,
            description_hash: HashType,
        ) -> Result<ProposalId, GovernanceError> {
            GovernorImpl::cancel(self, transactions, description_hash)
        }

        #[ink(message)]
        fn cast_vote(&mut self, proposal_id: ProposalId, support: VoteType) -> Result<Balance, GovernanceError> {
            GovernorImpl::cast_vote(self, proposal_id, support)
        }

        #[ink(message)]
        fn cast_vote_with_reason(
            &mut self,
            proposal_id: ProposalId,
            support: VoteType,
            reason: String,
        ) -> Result<Balance, GovernanceError> {
            GovernorImpl::cast_vote_with_reason(self, proposal_id, support, reason)
        }

        #[ink(message)]
        fn cast_vote_with_reason_and_params(
            &mut self,
            proposal_id: ProposalId,
            support: VoteType,
            reason: String,
            params: Vec<u8>,
        ) -> Result<Balance, GovernanceError> {
            GovernorImpl::cast_vote_with_reason_and_params(self, proposal_id, support, reason, params)
        }

        #[ink(message)]
        fn cast_vote_with_signature(
            &mut self,
            proposal_id: ProposalId,
            support: VoteType,
            reason: String,
            signature: SignatureType,
        ) -> Result<Balance, GovernanceError> {
            GovernorImpl::cast_vote_with_signature(self, proposal_id, support, reason, signature)
        }

        #[ink(message)]
        fn cast_vote_with_signature_and_params(
            &mut self,
            proposal_id: ProposalId,
            support: VoteType,
            reason: String,
            signature: SignatureType,
            params: Vec<u8>,
        ) -> Result<Balance, GovernanceError> {
            GovernorImpl::cast_vote_with_signature_and_params(self, proposal_id, support, reason, signature, params)
        }

        #[ink(message)]
        fn relay(&mut self, target: AccountId, transaction: Transaction) -> Result<(), GovernanceError> {
            GovernorImpl::relay(self, target, transaction)
        }
    }

    impl Nonces for Contract {
        #[ink(message)]
        fn nonces(&self, account: AccountId) -> u128 {
            NoncesImpl::nonces(self, &account)
        }
    }
}
