#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
pub mod my_psp34_enumerable {
    use openbrush::{
        contracts::psp34::extensions::{
            burnable::*,
            enumerable::*,
            mintable::*,
        },
        traits::Storage,
    };

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        enumerable: enumerable::Data,
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

    impl enumerable::BalancesManagerImpl for Contract {}

    impl psp34::BalancesManager for Contract {
        fn _balance_of(&self, owner: &Owner) -> u32 {
            enumerable::BalancesManagerImpl::_balance_of(self, owner)
        }

        fn _increase_balance(&mut self, owner: &Owner, id: &Id, increase_supply: bool) {
            enumerable::BalancesManagerImpl::_increase_balance(self, owner, id, increase_supply)
        }

        fn _decrease_balance(&mut self, owner: &Owner, id: &Id, decrease_supply: bool) {
            enumerable::BalancesManagerImpl::_decrease_balance(self, owner, id, decrease_supply)
        }

        fn _total_supply(&self) -> u128 {
            enumerable::BalancesManagerImpl::_total_supply(self)
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

    impl PSP34MintableImpl for Contract {}

    impl PSP34Mintable for Contract {
        #[ink(message)]
        fn mint(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error> {
            PSP34MintableImpl::mint(self, account, id)
        }
    }

    impl PSP34EnumerableImpl for Contract {}

    impl PSP34Enumerable for Contract {
        #[ink(message)]
        fn owners_token_by_index(&self, owner: AccountId, index: u128) -> Result<Id, PSP34Error> {
            PSP34EnumerableImpl::owners_token_by_index(self, owner, index)
        }

        #[ink(message)]
        fn token_by_index(&self, index: u128) -> Result<Id, PSP34Error> {
            PSP34EnumerableImpl::token_by_index(self, index)
        }
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use openbrush::contracts::psp34::extensions::{
            burnable::psp34burnable_external::PSP34Burnable,
            enumerable::psp34enumerable_external::PSP34Enumerable,
            mintable::psp34mintable_external::PSP34Mintable,
        };

        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::{build_message, PolkadotConfig};

        use test_helpers::address_of;

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn enumerable_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_psp34_enumerable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let owners_token_by_index_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(alice), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            let owners_token_by_index_2 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(bob), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(owners_token_by_index_1, Err(_)));
            assert!(matches!(owners_token_by_index_2, Err(_)));

            let psp34_id1 = Id::U8(1u8);
            let psp34_id2 = Id::U8(2u8);

            let mint_result_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(bob), psp34_id1.clone()));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            }
            .return_value();

            let mint_result_2 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(bob), psp34_id2.clone()));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            }
            .return_value();

            assert_eq!(mint_result_1, Ok(()));
            assert_eq!(mint_result_2, Ok(()));

            let owners_token_by_index_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(bob), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            let owners_token_by_index_2 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(bob), 1));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(owners_token_by_index_1, Ok(psp34_id1.clone()));
            assert_eq!(owners_token_by_index_2, Ok(psp34_id2.clone()));

            let token_by_index_1 = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.token_by_index(0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            let token_by_index_2 = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.token_by_index(1));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(token_by_index_1, Ok(psp34_id1.clone()));
            assert_eq!(token_by_index_2, Ok(psp34_id2.clone()));

            Ok(())
        }

        #[ink_e2e::test]
        async fn enumerable_works_after_burn(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_psp34_enumerable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let psp34_id1 = Id::U8(1u8);
            let psp34_id2 = Id::U8(2u8);

            let owners_token_by_index_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(alice), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            let owners_token_by_index_2 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(bob), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(owners_token_by_index_1, Err(_)));
            assert!(matches!(owners_token_by_index_2, Err(_)));

            let mint_result_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(bob), psp34_id1.clone()));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            }
            .return_value();

            let mint_result_2 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(bob), psp34_id2.clone()));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            }
            .return_value();

            assert_eq!(mint_result_1, Ok(()));
            assert_eq!(mint_result_2, Ok(()));

            let token_by_index_1 = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.token_by_index(0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            let token_by_index_2 = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.token_by_index(1));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(token_by_index_1, Ok(psp34_id1.clone()));
            assert_eq!(token_by_index_2, Ok(psp34_id2.clone()));

            let burn_result_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.burn(address_of!(bob), psp34_id2.clone()));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            }
            .return_value();

            assert_eq!(burn_result_1, Ok(()));

            let owners_token_by_index_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(bob), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            let owners_token_by_index_2 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(bob), 1));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(owners_token_by_index_1, Ok(psp34_id1.clone()));
            assert_eq!(owners_token_by_index_2, Err(PSP34Error::TokenNotExists));

            Ok(())
        }
    }
}
