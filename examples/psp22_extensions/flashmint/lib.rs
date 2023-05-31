#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
pub mod my_psp22_flashmint {
    use openbrush::{
        contracts::psp22::extensions::flashmint::*,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp22: psp22::Data,
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

    impl FlashLenderImpl for Contract {}

    impl FlashLender for Contract {
        #[ink(message)]
        fn max_flashloan(&mut self, token: AccountId) -> Balance {
            FlashLenderImpl::max_flashloan(self, token)
        }

        #[ink(message)]
        fn flash_fee(&self, token: AccountId, amount: Balance) -> Result<Balance, FlashLenderError> {
            FlashLenderImpl::flash_fee(self, token, amount)
        }

        #[ink(message)]
        fn flashloan(
            &mut self,
            receiver_account: AccountId,
            token: AccountId,
            amount: Balance,
            data: Vec<u8>,
        ) -> Result<(), FlashLenderError> {
            FlashLenderImpl::flashloan(self, receiver_account, token, amount, data)
        }
    }

    impl InternalImpl for Contract {}

    impl Internal for Contract {
        /// Override `get_fee` function to add 1% fee to the borrowed `amount`
        fn _get_fee(&self, amount: Balance) -> Balance {
            amount / 100
        }

        fn _on_flashloan(
            &mut self,
            receiver_account: AccountId,
            token: AccountId,
            fee: Balance,
            amount: Balance,
            data: Vec<u8>,
        ) -> Result<(), FlashLenderError> {
            InternalImpl::_on_flashloan(self, receiver_account, token, fee, amount, data)
        }
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self::default();
            psp22::Internal::_mint_to(&mut instance, Self::env().caller(), total_supply).expect("Should mint");

            instance
        }
    }
}
