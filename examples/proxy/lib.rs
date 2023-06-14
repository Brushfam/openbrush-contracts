#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::contract]
pub mod proxy {
    use openbrush::{
        contracts::{
            ownable::*,
            proxy::*,
        },
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        proxy: proxy::Data,
        #[storage_field]
        ownable: ownable::Data,
    }

    impl ownable::InternalImpl for Contract {}

    impl ownable::Internal for Contract {
        fn _emit_ownership_transferred_event(&self, previous: Option<AccountId>, new: Option<AccountId>) {
            ownable::InternalImpl::_emit_ownership_transferred_event(self, previous, new)
        }

        fn _init_with_owner(&mut self, owner: AccountId) {
            ownable::InternalImpl::_init_with_owner(self, owner)
        }
    }

    impl OwnableImpl for Contract {}

    impl Ownable for Contract {
        #[ink(message)]
        fn owner(&self) -> AccountId {
            OwnableImpl::owner(self)
        }

        #[ink(message)]
        fn renounce_ownership(&mut self) -> Result<(), OwnableError> {
            OwnableImpl::renounce_ownership(self)
        }

        #[ink(message)]
        fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<(), OwnableError> {
            OwnableImpl::transfer_ownership(self, new_owner)
        }
    }

    impl proxy::InternalImpl for Contract {}

    impl proxy::Internal for Contract {
        fn _emit_delegate_code_changed_event(&self, previous: Option<Hash>, new: Option<Hash>) {
            proxy::InternalImpl::_emit_delegate_code_changed_event(self, previous, new)
        }

        fn _init_with_forward_to(&mut self, forward_to: Hash) {
            proxy::InternalImpl::_init_with_forward_to(self, forward_to)
        }

        fn _fallback(&self) -> ! {
            proxy::InternalImpl::_fallback(self)
        }
    }

    impl ProxyImpl for Contract {}

    impl Proxy for Contract {
        #[ink(message)]
        fn get_delegate_code(&self) -> Hash {
            ProxyImpl::get_delegate_code(self)
        }

        #[ink(message)]
        fn change_delegate_code(&mut self, new_code_hash: Hash) -> Result<(), OwnableError> {
            ProxyImpl::change_delegate_code(self, new_code_hash)
        }
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(forward_to: Hash) -> Self {
            let mut instance = Self::default();
            proxy::Internal::_init_with_forward_to(&mut instance, forward_to);
            ownable::Internal::_init_with_owner(&mut instance, Self::env().caller());

            instance
        }
        #[ink(message, payable, selector = _)]
        pub fn forward(&self) {
            proxy::Internal::_fallback(self)
        }
    }
}
