#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::contract]
pub mod my_timelock_controller {
    use openbrush::{
        contracts::{
            nonces,
            nonces::{
                NoncesError,
                NoncesImpl,
            },
            traits::utils::nonces::*,
        },
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        nonces: nonces::Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn use_nonce(&mut self, account: AccountId) -> Result<u128, NoncesError> {
            NoncesImpl::_use_nonce(self, &account)
        }

        #[ink(message)]
        pub fn use_checked_nonce(&mut self, account: AccountId, nonce: u128) -> Result<u128, NoncesError> {
            NoncesImpl::_use_checked_nonce(self, &account, nonce)
        }
    }

    impl NoncesImpl for Contract {}

    impl Nonces for Contract {
        #[ink(message)]
        fn nonces(&self, account: AccountId) -> u128 {
            NoncesImpl::nonces(self, &account)
        }
    }
}
