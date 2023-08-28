#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP34)]
#[openbrush::contract]
pub mod my_psp34 {
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp34: psp34::Data,
        next_id: u8,
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
            let actual_balance = balance_of!(client, address, Alice);

            assert_eq!(expected_balance, actual_balance);
            assert_eq!(0, balance_of!(client, address, Bob));

            let transfer_result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of!(Bob), Id::U8(0), vec![]));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("transfer_from failed")
            }
            .return_value();

            assert_eq!(transfer_result, Ok(()));

            assert_eq!(0, balance_of!(client, address, Alice));
            assert_eq!(1, balance_of!(client, address, Bob));

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
            let actual_balance = balance_of!(client, address, Alice);

            assert_eq!(expected_balance, actual_balance);
            assert_eq!(0, balance_of!(client, address, Bob));

            let approve_result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.approve(address_of!(Bob), Some(Id::U8(0)), true));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("approve failed")
            }
            .return_value();

            assert_eq!(approve_result, Ok(()));

            let transfer_result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of!(Bob), Id::U8(0), vec![]));
                client
                    .call(&ink_e2e::bob(), _msg, 0, None)
                    .await
                    .expect("transfer_from failed")
            }
            .return_value();

            assert_eq!(transfer_result, Ok(()));

            assert_eq!(0, balance_of!(client, address, Alice));
            assert_eq!(1, balance_of!(client, address, Bob));

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
            let actual_balance = balance_of!(client, address, Alice);

            assert_eq!(expected_balance, actual_balance);
            assert_eq!(0, balance_of!(client, address, Bob));

            let approve_result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.approve(address_of!(Bob), None, true));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("approve failed")
            }
            .return_value();

            assert_eq!(approve_result, Ok(()));

            let transfer_result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of!(Bob), Id::U8(0), vec![]));
                client
                    .call(&ink_e2e::bob(), _msg, 0, None)
                    .await
                    .expect("transfer_from failed")
            }
            .return_value();

            assert_eq!(transfer_result, Ok(()));

            assert_eq!(0, balance_of!(client, address, Alice));
            assert_eq!(1, balance_of!(client, address, Bob));

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

            assert_eq!(owner_of!(client, address, Id::U8(0)), Some(address_of!(Alice)));

            let transfer_result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of!(Bob), Id::U8(0), vec![]));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("transfer_from failed")
            }
            .return_value();

            assert_eq!(transfer_result, Ok(()));

            assert_eq!(owner_of!(client, address, Id::U8(0)), Some(address_of!(Bob)));

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

            assert_eq!(balance_of!(client, address, Alice), 0);

            let transfer_result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of!(Bob), Id::U8(0), vec![]));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(transfer_result, Err(PSP34Error::TokenNotExists)));
            assert_eq!(balance_of!(client, address, Alice), 0);

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
                    .call(|contract| contract.transfer(address_of!(Bob), Id::U8(0), vec![]));
                client.call_dry_run(&ink_e2e::bob(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(transfer_result, Err(PSP34Error::NotApproved)));
            assert_eq!(balance_of!(client, address, Alice), 1);
            assert_eq!(balance_of!(client, address, Bob), 0);

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

            assert_eq!(balance_of!(client, address, Alice), 0);

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

            assert_eq!(balance_of!(client, address, Alice), 6);

            Ok(())
        }
    }
}
