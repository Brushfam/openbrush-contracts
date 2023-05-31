#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
pub mod my_psp22_wrapper {
    use ink::prelude::vec::Vec;
    use openbrush::{
        contracts::psp22::extensions::wrapper::*,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        wrapper: wrapper::Data,
    }

    impl psp22::Internal for Contract {
        fn _emit_transfer_event(&self, from: Option<AccountId>, to: Option<AccountId>, amount: Balance) {
            psp22::InternalImpl::_emit_transfer_event(self, from, to, amount)
        }

        fn _emit_approval_event(&self, owner: AccountId, spender: AccountId, amount: Balance) {
            psp22::InternalImpl::_emit_approval_event(self, owner, spender, amount)
        }

        fn _total_supply(&self) -> Balance {
            psp22::InternalImpl::_total_supply(self)
        }

        fn _balance_of(&self, owner: &AccountId) -> Balance {
            psp22::InternalImpl::_balance_of(self, owner)
        }

        fn _allowance(&self, owner: &AccountId, spender: &AccountId) -> Balance {
            psp22::InternalImpl::_allowance(self, owner, spender)
        }

        fn _transfer_from_to(
            &mut self,
            from: AccountId,
            to: AccountId,
            amount: Balance,
            data: Vec<u8>,
        ) -> Result<(), PSP22Error> {
            psp22::InternalImpl::_transfer_from_to(self, from, to, amount, data)
        }

        fn _approve_from_to(
            &mut self,
            owner: AccountId,
            spender: AccountId,
            amount: Balance,
        ) -> Result<(), PSP22Error> {
            psp22::InternalImpl::_approve_from_to(self, owner, spender, amount)
        }

        fn _mint_to(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            psp22::InternalImpl::_mint_to(self, account, amount)
        }

        fn _burn_from(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            psp22::InternalImpl::_burn_from(self, account, amount)
        }

        fn _before_token_transfer(
            &mut self,
            from: Option<&AccountId>,
            to: Option<&AccountId>,
            amount: &Balance,
        ) -> Result<(), PSP22Error> {
            psp22::InternalImpl::_before_token_transfer(self, from, to, amount)
        }

        fn _after_token_transfer(
            &mut self,
            from: Option<&AccountId>,
            to: Option<&AccountId>,
            amount: &Balance,
        ) -> Result<(), PSP22Error> {
            psp22::InternalImpl::_after_token_transfer(self, from, to, amount)
        }
    }

    impl psp22::InternalImpl for Contract {}

    impl PSP22Impl for Contract {}

    impl PSP22 for Contract {
        #[ink(message)]
        fn total_supply(&self) -> Balance {
            PSP22Impl::total_supply(self)
        }

        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> Balance {
            PSP22Impl::balance_of(self, owner)
        }

        #[ink(message)]
        fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            PSP22Impl::allowance(self, owner, spender)
        }

        #[ink(message)]
        fn transfer(&mut self, to: AccountId, value: Balance, data: Vec<u8>) -> Result<(), PSP22Error> {
            PSP22Impl::transfer(self, to, value, data)
        }

        #[ink(message)]
        fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
            data: Vec<u8>,
        ) -> Result<(), PSP22Error> {
            PSP22Impl::transfer_from(self, from, to, value, data)
        }

        #[ink(message)]
        fn approve(&mut self, spender: AccountId, value: Balance) -> Result<(), PSP22Error> {
            PSP22Impl::approve(self, spender, value)
        }

        #[ink(message)]
        fn increase_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), PSP22Error> {
            PSP22Impl::increase_allowance(self, spender, delta_value)
        }

        #[ink(message)]
        fn decrease_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), PSP22Error> {
            PSP22Impl::decrease_allowance(self, spender, delta_value)
        }
    }

    impl PSP22WrapperImpl for Contract {}

    impl PSP22Wrapper for Contract {
        #[ink(message)]
        fn deposit_for(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            PSP22WrapperImpl::deposit_for(self, account, amount)
        }

        #[ink(message)]
        fn withdraw_to(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            PSP22WrapperImpl::withdraw_to(self, account, amount)
        }
    }

    impl wrapper::InternalImpl for Contract {}

    impl wrapper::Internal for Contract {
        fn _recover(&mut self, account: AccountId) -> Result<Balance, PSP22Error> {
            wrapper::InternalImpl::_recover(self, account)
        }

        fn _deposit(&mut self, amount: Balance) -> Result<(), PSP22Error> {
            wrapper::InternalImpl::_deposit(self, amount)
        }

        fn _withdraw(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            wrapper::InternalImpl::_withdraw(self, account, amount)
        }

        fn _underlying_balance(&mut self) -> Balance {
            wrapper::InternalImpl::_underlying_balance(self)
        }

        fn _init(&mut self, underlying: AccountId) {
            wrapper::InternalImpl::_init(self, underlying)
        }

        fn _underlying(&mut self) -> &mut PSP22Ref {
            wrapper::InternalImpl::_underlying(self)
        }
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(token_address: AccountId) -> Self {
            let mut instance = Self::default();

            Internal::_init(&mut instance, token_address);

            instance
        }

        /// Exposes the `_recover` function for message caller
        #[ink(message)]
        pub fn recover(&mut self) -> Result<Balance, PSP22Error> {
            Internal::_recover(self, Self::env().caller())
        }
    }
}
