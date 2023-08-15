#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP22, PSP22Mintable)]
#[openbrush::contract]
pub mod my_psp22_votes {
    use openbrush::{
        contracts::{
            governance::utils::{
                votes,
                votes::*,
            },
            governor::TimestampProvider,
            nonces,
            nonces::*,
            psp22::extensions::votes::*,
            traits::{
                errors::GovernanceError,
                governance::utils::*,
                psp22::{
                    extensions::votes::*,
                    *,
                },
                types::SignatureType,
            },
            utils::checkpoint::Checkpoint,
        },
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        votes: votes::Data,
        #[storage_field]
        nonces: nonces::Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self::default();

            psp22::Internal::_mint_to(&mut instance, Self::env().caller(), total_supply).expect("Should mint");

            instance
        }
    }

    impl NoncesImpl for Contract {}

    impl VotesEvents for Contract {}

    impl VotesInternal for Contract {
        fn _get_voting_units(&self, account: &AccountId) -> Balance {
            PSP22VotesInternal::_get_voting_units(self, account)
        }
    }

    impl TimestampProvider for Contract {}

    impl VotesImpl for Contract {}

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
        fn delegates(&mut self, delegator: AccountId) -> Option<AccountId> {
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

    impl PSP22VotesImpl for Contract {}
    impl PSP22VotesInternal for Contract {}

    impl PSP22Votes for Contract {
        #[ink(message)]
        fn num_checkpoints(&self, account: AccountId) -> Result<u32, GovernanceError> {
            PSP22VotesImpl::num_checkpoints(self, account)
        }

        #[ink(message)]
        fn checkpoints(&self, account: AccountId, pos: u32) -> Result<Checkpoint, GovernanceError> {
            PSP22VotesImpl::checkpoints(self, account, pos)
        }
    }
}
