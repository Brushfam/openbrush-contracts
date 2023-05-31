#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
pub mod my_psp22_token_timelock {
    use openbrush::{
        contracts::psp22::utils::token_timelock::*,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        timelock: token_timelock::Data,
    }

    impl token_timelock::InternalImpl for Contract {}

    impl token_timelock::Internal for Contract {
        fn _withdraw(&mut self, amount: Balance) -> Result<(), PSP22TokenTimelockError> {
            token_timelock::InternalImpl::_withdraw(self, amount)
        }

        fn _contract_balance(&mut self) -> Balance {
            token_timelock::InternalImpl::_contract_balance(self)
        }

        fn _init(
            &mut self,
            token: AccountId,
            beneficiary: AccountId,
            release_time: Timestamp,
        ) -> Result<(), PSP22TokenTimelockError> {
            token_timelock::InternalImpl::_init(self, token, beneficiary, release_time)
        }

        fn _token(&mut self) -> &mut PSP22Ref {
            token_timelock::InternalImpl::_token(self)
        }

        fn _beneficiary(&self) -> AccountId {
            token_timelock::InternalImpl::_beneficiary(self)
        }
    }

    impl PSP22TokenTimelockImpl for Contract {}

    impl PSP22TokenTimelock for Contract {
        #[ink(message)]
        fn token(&self) -> AccountId {
            PSP22TokenTimelockImpl::token(self)
        }

        #[ink(message)]
        fn beneficiary(&self) -> AccountId {
            PSP22TokenTimelockImpl::beneficiary(self)
        }

        #[ink(message)]
        fn release_time(&self) -> Timestamp {
            PSP22TokenTimelockImpl::release_time(self)
        }

        #[ink(message)]
        fn release(&mut self) -> Result<(), PSP22TokenTimelockError> {
            PSP22TokenTimelockImpl::release(self)
        }
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(token_address: AccountId, beneficiary: AccountId, release_time: Timestamp) -> Self {
            let mut instance = Self::default();

            token_timelock::Internal::_init(&mut instance, token_address, beneficiary, release_time)
                .expect("Should init");

            instance
        }
    }
}
