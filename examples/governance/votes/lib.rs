#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(AccessControl)]
#[openbrush::contract]
pub mod my_governor {
    use ink::prelude::vec::Vec;
    use openbrush::{
        contracts::{
            governance::utils::{
                votes,
                votes::*,
            },
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
                    Transaction,
                    VoteType,
                },
                types::SignatureType,
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
        votes: votes::Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            instance
        }
    }
    impl VotesEvents for Contract {}
    impl VotesImpl for Contract {}
    impl VotesInternal for Contract {
        fn _get_voting_units(&self, _account: &AccountId) -> u128 {
            MAGIC_NUMBER
        }
    }

    impl Votes for Contract {
        #[ink(message)]
        fn get_votes(&self, account: AccountId) -> Result<Balance, GovernanceError> {
            VotesImpl::get_votes(self, account)
        }

        #[ink(message)]
        fn get_past_votes(&self, account: AccountId, timestamp: Timestamp) -> Result<Balance, GovernanceError> {
            VotesImpl::get_past_votes(self, account, timestamp)
        }

        #[ink(message)]
        fn get_past_total_supply(&self, timestamp: Timestamp) -> Result<Balance, GovernanceError> {
            VotesImpl::get_past_total_supply(self, timestamp)
        }

        #[ink(message)]
        fn delegates(&mut self, delegator: AccountId) -> AccountId {
            VotesImpl::delegates(self, delegator)
        }

        #[ink(message)]
        fn delegate(&mut self, delegatee: AccountId) -> Result<(), GovernanceError> {
            VotesImpl::delegate(self, delegatee)
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
            VotesImpl::delegate_by_signature(self, signer, delegatee, nonce, expiry, signature)
        }
    }
}
