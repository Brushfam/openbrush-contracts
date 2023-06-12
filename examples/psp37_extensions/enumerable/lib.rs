#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
pub mod my_psp37_enumerable {
    use openbrush::{
        contracts::psp37::extensions::{
            batch::*,
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
        psp37: psp37::Data,
        #[storage_field]
        enumerable: enumerable::Data,
    }

    impl PSP37Impl for Contract {}

    impl PSP37 for Contract {
        #[ink(message)]
        fn balance_of(&self, owner: AccountId, id: Option<Id>) -> Balance {
            PSP37Impl::balance_of(self, owner, id)
        }

        #[ink(message)]
        fn total_supply(&self, id: Option<Id>) -> Balance {
            PSP37Impl::total_supply(self, id)
        }

        #[ink(message)]
        fn allowance(&self, owner: AccountId, operator: AccountId, id: Option<Id>) -> Balance {
            PSP37Impl::allowance(self, owner, operator, id)
        }

        #[ink(message)]
        fn approve(&mut self, operator: AccountId, id: Option<Id>, value: Balance) -> Result<(), PSP37Error> {
            PSP37Impl::approve(self, operator, id, value)
        }

        #[ink(message)]
        fn transfer(&mut self, to: AccountId, id: Id, value: Balance, data: Vec<u8>) -> Result<(), PSP37Error> {
            PSP37Impl::transfer(self, to, id, value, data)
        }

        #[ink(message)]
        fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            id: Id,
            value: Balance,
            data: Vec<u8>,
        ) -> Result<(), PSP37Error> {
            PSP37Impl::transfer_from(self, from, to, id, value, data)
        }
    }

    impl psp37::InternalImpl for Contract {}

    impl psp37::Internal for Contract {
        fn _emit_transfer_event(&self, from: Option<AccountId>, to: Option<AccountId>, id: Id, amount: Balance) {
            psp37::InternalImpl::_emit_transfer_event(self, from, to, id, amount)
        }

        fn _emit_transfer_batch_event(
            &self,
            from: Option<AccountId>,
            to: Option<AccountId>,
            ids_amounts: Vec<(Id, Balance)>,
        ) {
            psp37::InternalImpl::_emit_transfer_batch_event(self, from, to, ids_amounts)
        }

        fn _emit_approval_event(&self, owner: AccountId, operator: AccountId, id: Option<Id>, value: Balance) {
            psp37::InternalImpl::_emit_approval_event(self, owner, operator, id, value)
        }

        fn _mint_to(&mut self, to: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP37Error> {
            psp37::InternalImpl::_mint_to(self, to, ids_amounts)
        }

        fn _burn_from(&mut self, from: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP37Error> {
            psp37::InternalImpl::_burn_from(self, from, ids_amounts)
        }

        fn _transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            id: Id,
            amount: Balance,
            data: Vec<u8>,
        ) -> Result<(), PSP37Error> {
            psp37::InternalImpl::_transfer_from(self, from, to, id, amount, data)
        }

        fn _get_allowance(&self, account: &AccountId, operator: &AccountId, id: &Option<&Id>) -> Balance {
            psp37::InternalImpl::_get_allowance(self, account, operator, id)
        }

        fn _approve_for(&mut self, operator: AccountId, id: Option<Id>, value: Balance) -> Result<(), PSP37Error> {
            psp37::InternalImpl::_approve_for(self, operator, id, value)
        }

        fn _decrease_allowance(
            &mut self,
            owner: &AccountId,
            operator: &AccountId,
            id: &Id,
            value: Balance,
        ) -> Result<(), PSP37Error> {
            psp37::InternalImpl::_decrease_allowance(self, owner, operator, id, value)
        }

        fn _transfer_token(
            &mut self,
            from: &AccountId,
            to: &AccountId,
            id: Id,
            amount: Balance,
            data: &Vec<u8>,
        ) -> Result<(), PSP37Error> {
            psp37::InternalImpl::_transfer_token(self, from, to, id, amount, data)
        }

        fn _before_token_transfer(
            &mut self,
            from: Option<&AccountId>,
            to: Option<&AccountId>,
            ids: &Vec<(Id, Balance)>,
        ) -> Result<(), PSP37Error> {
            psp37::InternalImpl::_before_token_transfer(self, from, to, ids)
        }

        fn _after_token_transfer(
            &mut self,
            from: Option<&AccountId>,
            to: Option<&AccountId>,
            ids: &Vec<(Id, Balance)>,
        ) -> Result<(), PSP37Error> {
            psp37::InternalImpl::_after_token_transfer(self, from, to, ids)
        }
    }

    impl enumerable::BalancesManagerImpl for Contract {}

    impl psp37::BalancesManager for Contract {
        fn _balance_of(&self, owner: &AccountId, id: &Option<&Id>) -> Balance {
            enumerable::BalancesManagerImpl::_balance_of(self, owner, id)
        }

        fn _total_supply(&self, id: &Option<&Id>) -> Balance {
            enumerable::BalancesManagerImpl::_total_supply(self, id)
        }

        fn _increase_balance(
            &mut self,
            owner: &AccountId,
            id: &Id,
            amount: &Balance,
            mint: bool,
        ) -> Result<(), PSP37Error> {
            enumerable::BalancesManagerImpl::_increase_balance(self, owner, id, amount, mint)
        }

        fn _decrease_balance(
            &mut self,
            owner: &AccountId,
            id: &Id,
            amount: &Balance,
            burn: bool,
        ) -> Result<(), PSP37Error> {
            enumerable::BalancesManagerImpl::_decrease_balance(self, owner, id, amount, burn)
        }
    }

    impl PSP37MintableImpl for Contract {}

    impl PSP37Mintable for Contract {
        #[ink(message)]
        fn mint(&mut self, to: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP37Error> {
            PSP37MintableImpl::mint(self, to, ids_amounts)
        }
    }

    impl PSP37BurnableImpl for Contract {}

    impl PSP37Burnable for Contract {
        #[ink(message)]
        fn burn(&mut self, from: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP37Error> {
            PSP37BurnableImpl::burn(self, from, ids_amounts)
        }
    }

    impl PSP37BatchImpl for Contract {}

    impl PSP37Batch for Contract {
        #[ink(message)]
        fn batch_transfer(
            &mut self,
            to: AccountId,
            ids_amounts: Vec<(Id, Balance)>,
            data: Vec<u8>,
        ) -> Result<(), PSP37Error> {
            PSP37BatchImpl::batch_transfer(self, to, ids_amounts, data)
        }

        #[ink(message)]
        fn batch_transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            ids_amounts: Vec<(Id, Balance)>,
            data: Vec<u8>,
        ) -> Result<(), PSP37Error> {
            PSP37BatchImpl::batch_transfer_from(self, from, to, ids_amounts, data)
        }
    }

    impl batch::InternalImpl for Contract {}

    impl batch::Internal for Contract {
        fn _batch_transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            ids_amounts: Vec<(Id, Balance)>,
            data: Vec<u8>,
        ) -> Result<(), PSP37Error> {
            batch::InternalImpl::_batch_transfer_from(self, from, to, ids_amounts, data)
        }
    }

    impl PSP37EnumerableImpl for Contract {}

    impl PSP37Enumerable for Contract {
        #[ink(message)]
        fn owners_token_by_index(&self, owner: AccountId, index: u128) -> Option<Id> {
            PSP37EnumerableImpl::owners_token_by_index(self, owner, index)
        }

        #[ink(message)]
        fn token_by_index(&self, index: u128) -> Option<Id> {
            PSP37EnumerableImpl::token_by_index(self, index)
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
        use openbrush::contracts::psp37::{
            extensions::{
                burnable::psp37burnable_external::PSP37Burnable,
                enumerable::psp37enumerable_external::PSP37Enumerable,
                mintable::psp37mintable_external::PSP37Mintable,
            },
            psp37_external::PSP37,
        };

        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::{build_message, PolkadotConfig};

        use test_helpers::address_of;

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn enumerable_should_fail(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_psp37_enumerable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let owners_token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(alice), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(owners_token_by_index, None);

            let owners_token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(bob), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(owners_token_by_index, None);

            Ok(())
        }

        #[ink_e2e::test]
        async fn enumerable_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_psp37_enumerable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let owners_token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(alice), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(owners_token_by_index, None);

            let owners_token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(bob), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(owners_token_by_index, None);

            let token_1 = Id::U8(0);
            let token_2 = Id::U8(1);

            let amount_1 = 1;
            let amount_2 = 2;

            let mint_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| {
                    contract.mint(
                        address_of!(alice),
                        vec![(token_1.clone(), amount_1), (token_2.clone(), amount_2)],
                    )
                });
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }
            .return_value();

            assert_eq!(mint_tx, Ok(()));

            let token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.token_by_index(0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(token_by_index, Some(token_1.clone()));

            let token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.token_by_index(1));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(token_by_index, Some(token_2.clone()));

            let owners_token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(alice), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(owners_token_by_index, Some(token_1.clone()));

            let owners_token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(bob), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(owners_token_by_index, None);

            Ok(())
        }

        #[ink_e2e::test]
        async fn enumerable_works_after_burn(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_psp37_enumerable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let owners_token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(alice), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(owners_token_by_index, None);

            let owners_token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(bob), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(owners_token_by_index, None);

            let token_1 = Id::U8(0);
            let token_2 = Id::U8(1);

            let amount_1 = 1;
            let amount_2 = 2;

            let mint_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| {
                    contract.mint(
                        address_of!(alice),
                        vec![(token_1.clone(), amount_1), (token_2.clone(), amount_2)],
                    )
                });
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }
            .return_value();

            assert_eq!(mint_tx, Ok(()));

            let token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.token_by_index(0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(token_by_index, Some(token_1.clone()));

            let token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.token_by_index(1));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(token_by_index, Some(token_2.clone()));

            let burn_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.burn(address_of!(alice), vec![(token_2.clone(), amount_2)]));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("burn failed")
            }
            .return_value();

            assert_eq!(burn_tx, Ok(()));

            let owners_token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(alice), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(owners_token_by_index, Some(token_1.clone()));

            let owners_token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(bob), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(owners_token_by_index, None);

            Ok(())
        }

        #[ink_e2e::test]
        async fn enumerable_transfer_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_psp37_enumerable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let owners_token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(alice), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(owners_token_by_index, None);

            let owners_token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(bob), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(owners_token_by_index, None);

            let token_1 = Id::U8(0);
            let token_2 = Id::U8(1);

            let amount_1 = 1;
            let amount_2 = 20;

            let mint_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| {
                    contract.mint(
                        address_of!(alice),
                        vec![(token_1.clone(), amount_1), (token_2.clone(), amount_2)],
                    )
                });
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }
            .return_value();

            assert_eq!(mint_tx, Ok(()));

            let token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.token_by_index(0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(token_by_index, Some(token_1.clone()));

            let token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.token_by_index(1));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(token_by_index, Some(token_2.clone()));

            let transfer_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of!(bob), token_2.clone(), amount_2, vec![]));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("transfer failed")
            }
            .return_value();

            assert_eq!(transfer_tx, Ok(()));

            let owners_token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(alice), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(owners_token_by_index, Some(token_1.clone()));

            let owners_token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(bob), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(owners_token_by_index, Some(token_2.clone()));

            Ok(())
        }
    }
}
