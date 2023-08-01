#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(AccessControl)]
#[openbrush::contract]
pub mod my_timelock_controller {
    use openbrush::{
        contracts::{
            crypto::SignatureType,
            extensions::{
                governor_counting::{
                    CountingInternal,
                    GovernorCountingImpl,
                },
                governor_votes::{
                    GovernorVotesImpl,
                    VotesEvents,
                    VotesInternal,
                },
            },
            governor::{
                GovernorEvents,
                GovernorImpl,
                GovernorInternal,
            },
            traits::{
                errors::GovernanceError,
                governance::{
                    extensions::{
                        governor_counting::GovernorCounting,
                        governor_votes::GovernorVotes,
                    },
                    governor::Governor,
                    HashType,
                    ProposalId,
                    ProposalState,
                    Transaction,
                },
            },
        },
        traits::{
            AccountId,
            Balance,
            Storage,
            Timestamp,
        },
    };

    const MAGIC_NUMBER: Balance = 42;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        access_control: access_control::Data,
        #[storage_field]
        timelock: timelock_controller::Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(min_delay: Timestamp, proposers: Vec<AccountId>, executors: Vec<AccountId>) -> Self {
            let mut instance = Self::default();

            let caller = Self::env().caller();
            // `TimelockController` and `AccessControl` have `_init_with_admin` methods.
            // You need to call it for each trait separately, to initialize everything for these traits.
            access_control::Internal::_init_with_admin(&mut instance, Some(caller));
            timelock_controller::Internal::_init_with_admin(
                &mut instance,
                Some(caller),
                min_delay,
                proposers,
                executors,
            );

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

    impl GovernorCounting for Contract {
        fn counting_mode(&self) -> openbrush::traits::String {
            GovernorCountingImpl::counting_mode(self)
        }

        fn has_voted(&self, proposal_id: ProposalId, account: AccountId) -> bool {
            GovernorCountingImpl::has_voted(self, proposal_id, account)
        }

        fn proposal_votes(&self, proposal_id: ProposalId) -> Result<(Balance, Balance, Balance), GovernanceError> {
            GovernorCountingImpl::proposal_votes(self, proposal_id)
        }
    }

    impl Governor for Contract {
        fn hash_proposal(&self, transactions: Vec<Transaction>, description_hash: HashType) -> HashType {
            GovernorImpl::hash_proposal(self, transactions, description_hash)
        }

        fn state(&self, proposal_id: ProposalId) -> ProposalState {
            GovernorImpl::state(self, proposal_id)
        }

        fn proposal_snapshot(&self, proposal_id: ProposalId) -> Result<Timestamp, GovernanceError> {
            GovernorImpl::proposal_snapshot(self, proposal_id)
        }

        fn proposal_deadline(&self, proposal_id: ProposalId) -> Result<Timestamp, GovernanceError> {
            GovernorImpl::proposal_deadline(self, proposal_id)
        }

        fn proposal_proposer(&self, proposal_id: ProposalId) -> Result<AccountId, GovernanceError> {
            GovernorImpl::proposal_proposer(self, proposal_id)
        }

        fn voting_delay(&self) -> u64 {
            GovernorImpl::voting_delay(self)
        }

        fn voting_period(&self) -> u64 {
            GovernorImpl::voting_period(self)
        }

        fn quorum(&self, time_point: Timestamp) -> u128 {
            GovernorImpl::quorum(self, time_point)
        }

        fn get_votes(&self, account: AccountId, time_point: Timestamp) -> u128 {
            GovernorImpl::get_votes(self, account, time_point)
        }

        fn get_votes_with_params(&self, account: AccountId, time_point: Timestamp, params: Vec<u8>) -> u128 {
            GovernorImpl::get_votes_with_params(self, account, time_point, params)
        }

        fn has_voted(&self, proposal_id: ProposalId, account: AccountId) -> bool {
            GovernorImpl::has_voted(self, proposal_id, account)
        }

        fn propose(
            &mut self,
            transactions: Vec<Transaction>,
            description: openbrush::traits::String,
        ) -> Result<ProposalId, GovernanceError> {
            GovernorImpl::propose(self, transactions, description)
        }

        fn execute(
            &mut self,
            transactions: Vec<Transaction>,
            description_hash: HashType,
        ) -> Result<ProposalId, GovernanceError> {
            GovernorImpl::execute(self, transactions, description_hash)
        }

        fn cancel(
            &mut self,
            transaction: Vec<Transaction>,
            description_hash: HashType,
        ) -> Result<ProposalId, GovernanceError> {
            GovernorImpl::cancel(self, transaction, description_hash)
        }

        fn cast_vote(&mut self, proposal_id: ProposalId, support: u8) -> Result<Balance, GovernanceError> {
            GovernorImpl::cast_vote(self, proposal_id, support)
        }

        fn cast_vote_with_reason(
            &mut self,
            proposal_id: ProposalId,
            support: u8,
            reason: openbrush::traits::String,
        ) -> Result<Balance, GovernanceError> {
            GovernorImpl::cast_vote_with_reason(self, proposal_id, support, reason)
        }

        fn cast_vote_with_reason_and_params(
            &mut self,
            proposal_id: ProposalId,
            support: u8,
            reason: openbrush::traits::String,
            params: Vec<u8>,
        ) -> Result<Balance, GovernanceError> {
            GovernorImpl::cast_vote_with_reason_and_params(self, proposal_id, support, reason, params)
        }

        fn cast_vote_with_signature(
            &mut self,
            proposal_id: ProposalId,
            support: u8,
            reason: openbrush::traits::String,
            signature: SignatureType,
        ) -> Result<Balance, GovernanceError> {
            GovernorImpl::cast_vote_with_signature(self, proposal_id, support, reason, signature)
        }

        fn cast_vote_with_signature_and_params(
            &mut self,
            proposal_id: ProposalId,
            support: u8,
            reason: openbrush::traits::String,
            signature: SignatureType,
            params: Vec<u8>,
        ) -> Result<Balance, GovernanceError> {
            GovernorImpl::cast_vote_with_signature_and_params(self, proposal_id, support, reason, signature, params)
        }
    }

    impl GovernorVotes for Contract {
        fn clock(&self) -> u64 {
            unimplemented!("clock")
        }

        fn get_votes(&self, account: AccountId) -> Balance {
            GovernorVotesImpl::get_votes(self, account)
        }

        fn get_past_votes(&self, account: AccountId, timestamp: Timestamp) -> Result<Balance, GovernanceError> {
            GovernorVotesImpl::get_past_votes(self, account, timestamp)
        }

        fn get_past_total_supply(&self, timestamp: Timestamp) -> Result<Balance, GovernanceError> {
            GovernorVotesImpl::get_past_total_supply(self, timestamp)
        }

        fn delegates(&mut self, delegator: AccountId) -> AccountId {
            GovernorVotesImpl::delegates(self, delegator)
        }

        fn delegate(&mut self, delegatee: AccountId) {
            GovernorVotesImpl::delegate(self, delegatee)
        }

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
