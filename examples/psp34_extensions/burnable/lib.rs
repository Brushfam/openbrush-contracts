#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
pub mod my_psp34_burnable {
    use openbrush::{
        contracts::psp34::extensions::burnable::*,
        traits::Storage,
    };

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp34: psp34::Data,
    }

    impl PSP34Impl for Contract {}

    impl PSP34 for Contract {
        #[ink(message)]
        fn collection_id(&self) -> Id {
            PSP34Impl::collection_id(self)
        }

        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> u32 {
            PSP34Impl::balance_of(self, owner)
        }

        #[ink(message)]
        fn owner_of(&self, id: Id) -> Option<AccountId> {
            PSP34Impl::owner_of(self, id)
        }

        #[ink(message)]
        fn allowance(&self, owner: AccountId, operator: AccountId, id: Option<Id>) -> bool {
            PSP34Impl::allowance(self, owner, operator, id)
        }

        #[ink(message)]
        fn approve(&mut self, operator: AccountId, id: Option<Id>, approved: bool) -> Result<(), PSP34Error> {
            PSP34Impl::approve(self, operator, id, approved)
        }

        #[ink(message)]
        fn transfer(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), PSP34Error> {
            PSP34Impl::transfer(self, to, id, data)
        }

        #[ink(message)]
        fn total_supply(&self) -> Balance {
            PSP34Impl::total_supply(self)
        }
    }

    impl psp34::BalancesManagerImpl for Contract {}

    impl psp34::BalancesManager for Contract {
        fn _balance_of(&self, owner: &Owner) -> u32 {
            psp34::BalancesManagerImpl::_balance_of(self, owner)
        }

        fn _increase_balance(&mut self, owner: &Owner, id: &Id, increase_supply: bool) {
            psp34::BalancesManagerImpl::_increase_balance(self, owner, id, increase_supply)
        }

        fn _decrease_balance(&mut self, owner: &Owner, id: &Id, decrease_supply: bool) {
            psp34::BalancesManagerImpl::_decrease_balance(self, owner, id, decrease_supply)
        }

        fn _total_supply(&self) -> u128 {
            psp34::BalancesManagerImpl::_total_supply(self)
        }
    }

    impl psp34::InternalImpl for Contract {}

    impl psp34::Internal for Contract {
        fn _emit_transfer_event(&self, from: Option<AccountId>, to: Option<AccountId>, id: Id) {
            psp34::InternalImpl::_emit_transfer_event(self, from, to, id)
        }

        fn _emit_approval_event(&self, from: AccountId, to: AccountId, id: Option<Id>, approved: bool) {
            psp34::InternalImpl::_emit_approval_event(self, from, to, id, approved)
        }

        fn _approve_for(&mut self, to: AccountId, id: Option<Id>, approved: bool) -> Result<(), PSP34Error> {
            psp34::InternalImpl::_approve_for(self, to, id, approved)
        }

        fn _owner_of(&self, id: &Id) -> Option<AccountId> {
            psp34::InternalImpl::_owner_of(self, id)
        }

        fn _transfer_token(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), PSP34Error> {
            psp34::InternalImpl::_transfer_token(self, to, id, data)
        }

        fn _mint_to(&mut self, to: AccountId, id: Id) -> Result<(), PSP34Error> {
            psp34::InternalImpl::_mint_to(self, to, id)
        }

        fn _burn_from(&mut self, from: AccountId, id: Id) -> Result<(), PSP34Error> {
            psp34::InternalImpl::_burn_from(self, from, id)
        }

        fn _allowance(&self, owner: &Owner, operator: &Operator, id: &Option<&Id>) -> bool {
            psp34::InternalImpl::_allowance(self, owner, operator, id)
        }

        fn _check_token_exists(&self, id: &Id) -> Result<AccountId, PSP34Error> {
            psp34::InternalImpl::_check_token_exists(self, id)
        }

        fn _before_token_transfer(
            &mut self,
            from: Option<&AccountId>,
            to: Option<&AccountId>,
            id: &Id,
        ) -> Result<(), PSP34Error> {
            psp34::InternalImpl::_before_token_transfer(self, from, to, id)
        }

        fn _after_token_transfer(
            &mut self,
            from: Option<&AccountId>,
            to: Option<&AccountId>,
            id: &Id,
        ) -> Result<(), PSP34Error> {
            psp34::InternalImpl::_after_token_transfer(self, from, to, id)
        }
    }

    impl PSP34BurnableImpl for Contract {}

    impl PSP34Burnable for Contract {
        #[ink(message)]
        fn burn(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error> {
            PSP34BurnableImpl::burn(self, account, id)
        }
    }

    impl Contract {
        /// The constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();

            psp34::Internal::_mint_to(&mut instance, Self::env().caller(), Id::U8(0u8))
                .expect("Should mint token with id 0");
            psp34::Internal::_mint_to(&mut instance, Self::env().caller(), Id::U8(1u8))
                .expect("Should mint token with id 1");
            psp34::Internal::_mint_to(&mut instance, Self::env().caller(), Id::U8(2u8))
                .expect("Should mint token with id 2");

            instance
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use openbrush::contracts::psp34::{
            extensions::burnable::psp34burnable_external::PSP34Burnable,
            psp34_external::PSP34,
        };

        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::{build_message, PolkadotConfig};

        use test_helpers::{
            address_of,
            balance_of,
            owner_of,
        };

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn burn_wokrs(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_psp34_burnable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(balance_of!(client, address, alice), 3);

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.burn(address_of!(alice), Id::U8(0u8)));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            };

            assert_eq!(result.return_value(), Ok(()));
            assert_eq!(balance_of!(client, address, alice), 2);

            Ok(())
        }

        #[ink_e2e::test]
        async fn burn_from_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_psp34_burnable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(balance_of!(client, address, alice), 3);

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.burn(address_of!(alice), Id::U8(0u8)));
                client.call(&ink_e2e::bob(), _msg, 0, None).await.expect("call failed")
            };

            assert_eq!(result.return_value(), Ok(()));
            assert_eq!(balance_of!(client, address, alice), 2);

            Ok(())
        }
    }
}
