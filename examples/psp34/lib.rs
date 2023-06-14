#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::contract]
pub mod my_psp34 {
    use openbrush::{
        contracts::psp34::extensions::metadata::*,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        metadata: Data,
        next_id: u8,
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

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn mint_token(&mut self) -> Result<(), PSP34Error> {
            psp34::Internal::_mint_to(self, Self::env().caller(), Id::U8(self.next_id))?;
            self.next_id += 1;
            Ok(())
        }

        #[ink(message)]
        pub fn mint(&mut self, id: Id) -> Result<(), PSP34Error> {
            psp34::Internal::_mint_to(self, Self::env().caller(), id)
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use openbrush::contracts::psp34::psp34_external::PSP34;
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
        async fn return_collection_id_of_account(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_psp34", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let account_id = ink::primitives::AccountId::from(address);

            let expected_collection_id = Id::Bytes(AsRef::<[u8]>::as_ref(&account_id).to_vec());
            let actual_collection_id = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.collection_id());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(expected_collection_id, actual_collection_id);

            Ok(())
        }

        #[ink_e2e::test]
        async fn returns_total_supply(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_psp34", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let expected_total_supply = 0;
            let actual_total_supply = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.total_supply());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            };

            assert_eq!(expected_total_supply, actual_total_supply.return_value());

            for _ in 0..3 {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.mint_token());
                let result = client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint_token failed");

                assert_eq!(result.return_value(), Ok(()));
            }

            let expected_total_supply = 3;
            let actual_total_supply = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.total_supply());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            };

            assert_eq!(expected_total_supply, actual_total_supply.return_value());

            Ok(())
        }

        #[ink_e2e::test]
        async fn transfer_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_psp34", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let mint_result = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.mint_token());
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint_token failed")
            }
            .return_value();

            assert_eq!(mint_result, Ok(()));

            let expected_balance = 1;
            let actual_balance = balance_of!(client, address, alice);

            assert_eq!(expected_balance, actual_balance);
            assert_eq!(0, balance_of!(client, address, bob));

            let transfer_result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of!(bob), Id::U8(0), vec![]));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("transfer_from failed")
            }
            .return_value();

            assert_eq!(transfer_result, Ok(()));

            assert_eq!(0, balance_of!(client, address, alice));
            assert_eq!(1, balance_of!(client, address, bob));

            Ok(())
        }

        #[ink_e2e::test]
        async fn approved_transfer_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_psp34", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let mint_result = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.mint_token());
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint_token failed")
            }
            .return_value();

            assert_eq!(mint_result, Ok(()));

            let expected_balance = 1;
            let actual_balance = balance_of!(client, address, alice);

            assert_eq!(expected_balance, actual_balance);
            assert_eq!(0, balance_of!(client, address, bob));

            let approve_result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.approve(address_of!(bob), Some(Id::U8(0)), true));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("approve failed")
            }
            .return_value();

            assert_eq!(approve_result, Ok(()));

            let transfer_result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of!(bob), Id::U8(0), vec![]));
                client
                    .call(&ink_e2e::bob(), _msg, 0, None)
                    .await
                    .expect("transfer_from failed")
            }
            .return_value();

            assert_eq!(transfer_result, Ok(()));

            assert_eq!(0, balance_of!(client, address, alice));
            assert_eq!(1, balance_of!(client, address, bob));

            Ok(())
        }

        #[ink_e2e::test]
        async fn approved_operator_transfer_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_psp34", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let mint_result = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.mint_token());
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint_token failed")
            }
            .return_value();

            assert_eq!(mint_result, Ok(()));

            let expected_balance = 1;
            let actual_balance = balance_of!(client, address, alice);

            assert_eq!(expected_balance, actual_balance);
            assert_eq!(0, balance_of!(client, address, bob));

            let approve_result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.approve(address_of!(bob), None, true));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("approve failed")
            }
            .return_value();

            assert_eq!(approve_result, Ok(()));

            let transfer_result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of!(bob), Id::U8(0), vec![]));
                client
                    .call(&ink_e2e::bob(), _msg, 0, None)
                    .await
                    .expect("transfer_from failed")
            }
            .return_value();

            assert_eq!(transfer_result, Ok(()));

            assert_eq!(0, balance_of!(client, address, alice));
            assert_eq!(1, balance_of!(client, address, bob));

            Ok(())
        }

        #[ink_e2e::test]
        async fn psp34_transfer_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_psp34", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let mint_result = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.mint_token());
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint_token failed")
            }
            .return_value();

            assert_eq!(mint_result, Ok(()));

            assert_eq!(owner_of!(client, address, Id::U8(0)), Some(address_of!(alice)));

            let transfer_result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of!(bob), Id::U8(0), vec![]));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("transfer_from failed")
            }
            .return_value();

            assert_eq!(transfer_result, Ok(()));

            assert_eq!(owner_of!(client, address, Id::U8(0)), Some(address_of!(bob)));

            Ok(())
        }

        #[ink_e2e::test]
        async fn can_nextot_transfer_non_existing_token(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_psp34", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(balance_of!(client, address, alice), 0);

            let transfer_result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of!(bob), Id::U8(0), vec![]));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(transfer_result, Err(PSP34Error::TokenNotExists)));
            assert_eq!(balance_of!(client, address, alice), 0);

            Ok(())
        }

        #[ink_e2e::test]
        async fn cannot_transfer_without_allowance(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_psp34", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let mint_result = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.mint_token());
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint_token failed")
            }
            .return_value();

            assert_eq!(mint_result, Ok(()));

            let transfer_result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of!(bob), Id::U8(0), vec![]));
                client.call_dry_run(&ink_e2e::bob(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(transfer_result, Err(PSP34Error::NotApproved)));
            assert_eq!(balance_of!(client, address, alice), 1);
            assert_eq!(balance_of!(client, address, bob), 0);

            Ok(())
        }

        #[ink_e2e::test]
        async fn can_mint_any_id(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_psp34", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(balance_of!(client, address, alice), 0);

            let ids = vec![
                Id::U8(0),
                Id::U16(0),
                Id::U32(0),
                Id::U64(0),
                Id::U128(0),
                Id::Bytes(vec![0]),
            ];

            for id in ids {
                let mint_result = {
                    let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.mint(id.clone()));
                    client
                        .call(&ink_e2e::alice(), _msg, 0, None)
                        .await
                        .expect("mint failed")
                }
                .return_value();

                assert_eq!(mint_result, Ok(()));
            }

            assert_eq!(balance_of!(client, address, alice), 6);

            Ok(())
        }
    }
}
