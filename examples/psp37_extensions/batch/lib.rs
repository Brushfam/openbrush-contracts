#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::contract]
pub mod my_psp37 {
    use ink::prelude::vec::Vec;
    use openbrush::{
        contracts::psp37::extensions::batch::*,
        traits::Storage,
    };

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp37: psp37::Data,
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

    impl psp37::BalancesManagerImpl for Contract {}

    impl psp37::BalancesManager for Contract {
        fn _balance_of(&self, owner: &AccountId, id: &Option<&Id>) -> Balance {
            psp37::BalancesManagerImpl::_balance_of(self, owner, id)
        }

        fn _total_supply(&self, id: &Option<&Id>) -> Balance {
            psp37::BalancesManagerImpl::_total_supply(self, id)
        }

        fn _increase_balance(
            &mut self,
            owner: &AccountId,
            id: &Id,
            amount: &Balance,
            mint: bool,
        ) -> Result<(), PSP37Error> {
            psp37::BalancesManagerImpl::_increase_balance(self, owner, id, amount, mint)
        }

        fn _decrease_balance(
            &mut self,
            owner: &AccountId,
            id: &Id,
            amount: &Balance,
            burn: bool,
        ) -> Result<(), PSP37Error> {
            psp37::BalancesManagerImpl::_decrease_balance(self, owner, id, amount, burn)
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

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn mint(&mut self, to: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP37Error> {
            psp37::Internal::_mint_to(self, to, ids_amounts)
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use openbrush::contracts::psp37::{
            extensions::batch::psp37batch_external::PSP37Batch,
            psp37_external::PSP37,
        };

        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::{build_message, PolkadotConfig};

        use test_helpers::{
            address_of,
            balance_of_37,
        };

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn batch_transfer_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_psp37_batch", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let token_1 = Id::U8(0);
            let token_2 = Id::U8(1);

            let amount_1 = 1;
            let amount_2 = 20;

            let mint_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| {
                    contract.mint(
                        address_of!(alice),
                        vec![(token_1.clone(), amount_1.clone()), (token_2.clone(), amount_2.clone())],
                    )
                });
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }
            .return_value();

            assert_eq!(mint_tx, Ok(()));

            let batch_transfer_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| {
                    contract.batch_transfer(
                        address_of!(bob),
                        vec![(token_1.clone(), amount_1.clone()), (token_2.clone(), amount_2.clone())],
                        vec![],
                    )
                });
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }
            .return_value();

            assert_eq!(batch_transfer_tx, Ok(()));

            assert_eq!(balance_of_37!(client, address, bob, Some(token_1.clone())), amount_1);
            assert_eq!(balance_of_37!(client, address, bob, Some(token_2.clone())), amount_2);
            assert_eq!(balance_of_37!(client, address, alice, Some(token_1.clone())), 0);
            assert_eq!(balance_of_37!(client, address, alice, Some(token_2.clone())), 0);

            Ok(())
        }

        #[ink_e2e::test]
        async fn batch_transfer_from_should_work(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_psp37_batch", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let token_1 = Id::U8(0);
            let token_2 = Id::U8(1);

            let amount_1 = 1;
            let amount_2 = 20;

            let mint_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| {
                    contract.mint(
                        address_of!(alice),
                        vec![(token_1.clone(), amount_1.clone()), (token_2.clone(), amount_2.clone())],
                    )
                });
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }
            .return_value();

            assert_eq!(mint_tx, Ok(()));

            assert_eq!(balance_of_37!(client, address, alice, Some(token_1.clone())), amount_1);
            assert_eq!(balance_of_37!(client, address, alice, Some(token_2.clone())), amount_2);
            assert_eq!(balance_of_37!(client, address, bob, Some(token_1.clone())), 0);
            assert_eq!(balance_of_37!(client, address, bob, Some(token_2.clone())), 0);

            let batch_transfer_from_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| {
                    contract.batch_transfer_from(
                        address_of!(alice),
                        address_of!(bob),
                        vec![(token_1.clone(), amount_1.clone()), (token_2.clone(), amount_2.clone())],
                        vec![],
                    )
                });
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }
            .return_value();

            assert_eq!(batch_transfer_from_tx, Ok(()));

            assert_eq!(balance_of_37!(client, address, bob, Some(token_1.clone())), amount_1);
            assert_eq!(balance_of_37!(client, address, bob, Some(token_2.clone())), amount_2);
            assert_eq!(balance_of_37!(client, address, alice, Some(token_1.clone())), 0);
            assert_eq!(balance_of_37!(client, address, alice, Some(token_2.clone())), 0);

            Ok(())
        }

        #[ink_e2e::test]
        async fn batch_transfer_from_with_no_approve_should_fail(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_psp37_batch", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let token_1 = Id::U8(0);
            let token_2 = Id::U8(1);

            let amount_1 = 1;
            let amount_2 = 20;

            let mint_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| {
                    contract.mint(
                        address_of!(alice),
                        vec![(token_1.clone(), amount_1.clone()), (token_2.clone(), amount_2.clone())],
                    )
                });
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }
            .return_value();

            assert_eq!(mint_tx, Ok(()));

            let batch_transfer_from_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| {
                    contract.batch_transfer_from(
                        address_of!(bob),
                        address_of!(alice),
                        vec![(token_1.clone(), amount_1.clone()), (token_2.clone(), amount_2.clone())],
                        vec![],
                    )
                });
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(batch_transfer_from_tx, Err(_)));

            Ok(())
        }

        #[ink_e2e::test]
        async fn batch_transfer_from_with_approve_should_work(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_psp37_batch", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let token_1 = Id::U8(0);
            let token_2 = Id::U8(1);

            let amount_1 = 1;
            let amount_2 = 20;

            let mint_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| {
                    contract.mint(
                        address_of!(alice),
                        vec![(token_1.clone(), amount_1.clone()), (token_2.clone(), amount_2.clone())],
                    )
                });
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }
            .return_value();

            assert_eq!(mint_tx, Ok(()));

            let approve_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.approve(address_of!(bob), None, 1));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("approve failed")
            }
            .return_value();

            assert_eq!(approve_tx, Ok(()));

            let batch_transfer_from_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| {
                    contract.batch_transfer_from(
                        address_of!(alice),
                        address_of!(bob),
                        vec![(token_1.clone(), amount_1.clone()), (token_2.clone(), amount_2.clone())],
                        vec![],
                    )
                });
                client
                    .call(&ink_e2e::bob(), _msg, 0, None)
                    .await
                    .expect("transfer failed")
            }
            .return_value();

            assert_eq!(batch_transfer_from_tx, Ok(()));

            assert_eq!(balance_of_37!(client, address, bob, Some(token_1.clone())), amount_1);
            assert_eq!(balance_of_37!(client, address, bob, Some(token_2.clone())), amount_2);
            assert_eq!(balance_of_37!(client, address, alice, Some(token_1.clone())), 0);
            assert_eq!(balance_of_37!(client, address, alice, Some(token_2.clone())), 0);

            Ok(())
        }
    }
}
