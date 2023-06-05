#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
pub mod my_psp22_facet_v2 {
    use ink::{
        codegen::Env,
        prelude::vec::Vec,
    };
    use openbrush::{
        contracts::{
            ownable::*,
            psp22::*,
        },
        traits::{
            Storage,
            ZERO_ADDRESS,
        },
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct PSP22FacetV2 {
        #[storage_field]
        psp22: psp22::Data,
        // Ownable is used only internally without exposing it to the world
        #[storage_field]
        ownable: ownable::Data,
    }

    impl psp22::InternalImpl for PSP22FacetV2 {}

    impl psp22::Internal for PSP22FacetV2 {
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

        // override this fn
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

    impl PSP22Impl for PSP22FacetV2 {}

    impl PSP22 for PSP22FacetV2 {
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
            let from = self.env().caller();
            let is_tax = to != ZERO_ADDRESS.into() && from != ZERO_ADDRESS.into();
            // we will burn 10% of transfer to and from non-zero accounts
            let burned = if is_tax { value / 10 } else { 0 };
            if is_tax {
                psp22::Internal::_burn_from(self, from, burned)?;
            }
            psp22::Internal::_transfer_from_to(self, from, to, value - burned, data)?;
            Ok(())
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

    impl PSP22FacetV2 {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }
}
