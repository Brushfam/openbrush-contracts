#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
pub mod my_psp22_upgradeable {
    use openbrush::{
        contracts::{
            ownable::*,
            psp22::*,
        },
        modifiers,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct MyPSP22 {
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        psp22: psp22::Data,
    }

    impl ownable::InternalImpl for MyPSP22 {}

    impl ownable::Internal for MyPSP22 {
        fn _emit_ownership_transferred_event(&self, previous: Option<AccountId>, new: Option<AccountId>) {
            ownable::InternalImpl::_emit_ownership_transferred_event(self, previous, new)
        }

        fn _init_with_owner(&mut self, owner: AccountId) {
            ownable::InternalImpl::_init_with_owner(self, owner)
        }
    }

    impl OwnableImpl for MyPSP22 {}

    impl Ownable for MyPSP22 {
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

    impl psp22::InternalImpl for MyPSP22 {}

    impl psp22::Internal for MyPSP22 {
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

    impl PSP22Impl for MyPSP22 {}

    impl PSP22 for MyPSP22 {
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

    impl MyPSP22 {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self::default();

            ownable::Internal::_init_with_owner(&mut instance, Self::env().caller());
            instance.initialize(total_supply).ok().unwrap();

            instance
        }

        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn initialize(&mut self, total_supply: Balance) -> Result<(), OwnableError> {
            psp22::Internal::_mint_to(self, Ownable::owner(self), total_supply).expect("Should mint");
            Ok(())
        }
    }
}
