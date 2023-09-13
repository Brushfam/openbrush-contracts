#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP34, PSP34Mintable, PSP34Burnable, PSP34Enumerable)]
#[openbrush::contract]
pub mod my_psp34_enumerable {
    use openbrush::traits::Storage;

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        enumerable: enumerable::Data,
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
        async fn enumerable_works<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let contract = client
                .instantiate("my_psp34_enumerable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let call = contract.call::<Contract>();

            let owners_token_by_index_1 = {
                let _msg = call.owners_token_by_index(address_of!(Alice), 0);
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            let owners_token_by_index_2 = {
                let _msg = call.owners_token_by_index(address_of!(Bob), 0);
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(owners_token_by_index_1, Err(_)));
            assert!(matches!(owners_token_by_index_2, Err(_)));

            let psp34_id1 = Id::U8(1u8);
            let psp34_id2 = Id::U8(2u8);

            let mint_result_1 = {
                let _msg = call.mint(address_of!(Bob), psp34_id1.clone());
                client
                    .call(&ink_e2e::alice(), &_msg, 0, None)
                    .await
                    .expect("call failed")
            }
            .return_value();

            let mint_result_2 = {
                let _msg = call.mint(address_of!(Bob), psp34_id2.clone());
                client
                    .call(&ink_e2e::alice(), &_msg, 0, None)
                    .await
                    .expect("call failed")
            }
            .return_value();

            assert_eq!(mint_result_1, Ok(()));
            assert_eq!(mint_result_2, Ok(()));

            let owners_token_by_index_1 = {
                let _msg = call.owners_token_by_index(address_of!(Bob), 0);
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            let owners_token_by_index_2 = {
                let _msg = call.owners_token_by_index(address_of!(Bob), 1);
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(owners_token_by_index_1, Ok(psp34_id1.clone()));
            assert_eq!(owners_token_by_index_2, Ok(psp34_id2.clone()));

            let token_by_index_1 = {
                let _msg = call.token_by_index(0);
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            let token_by_index_2 = {
                let _msg = call.token_by_index(1);
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(token_by_index_1, Ok(psp34_id1.clone()));
            assert_eq!(token_by_index_2, Ok(psp34_id2.clone()));

            Ok(())
        }

        #[ink_e2e::test]
        async fn enumerable_works_after_burn<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let contract = client
                .instantiate("my_psp34_enumerable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let call = contract.call::<Contract>();

            let psp34_id1 = Id::U8(1u8);
            let psp34_id2 = Id::U8(2u8);

            let owners_token_by_index_1 = {
                let _msg = call.owners_token_by_index(address_of!(Alice), 0);
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            let owners_token_by_index_2 = {
                let _msg = call.owners_token_by_index(address_of!(Bob), 0);
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(owners_token_by_index_1, Err(_)));
            assert!(matches!(owners_token_by_index_2, Err(_)));

            let mint_result_1 = {
                let _msg = call.mint(address_of!(Bob), psp34_id1.clone());
                client
                    .call(&ink_e2e::alice(), &_msg, 0, None)
                    .await
                    .expect("call failed")
            }
            .return_value();

            let mint_result_2 = {
                let _msg = call.mint(address_of!(Bob), psp34_id2.clone());
                client
                    .call(&ink_e2e::alice(), &_msg, 0, None)
                    .await
                    .expect("call failed")
            }
            .return_value();

            assert_eq!(mint_result_1, Ok(()));
            assert_eq!(mint_result_2, Ok(()));

            let token_by_index_1 = {
                let _msg = call.token_by_index(0);
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            let token_by_index_2 = {
                let _msg = call.token_by_index(1);
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(token_by_index_1, Ok(psp34_id1.clone()));
            assert_eq!(token_by_index_2, Ok(psp34_id2.clone()));

            let burn_result_1 = {
                let _msg = call.burn(address_of!(Bob), psp34_id2.clone());
                client
                    .call(&ink_e2e::alice(), &_msg, 0, None)
                    .await
                    .expect("call failed")
            }
            .return_value();

            assert_eq!(burn_result_1, Ok(()));

            let owners_token_by_index_1 = {
                let _msg = call.owners_token_by_index(address_of!(Bob), 0);
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            let owners_token_by_index_2 = {
                let _msg = call.owners_token_by_index(address_of!(Bob), 1);
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(owners_token_by_index_1, Ok(psp34_id1.clone()));
            assert_eq!(owners_token_by_index_2, Err(PSP34Error::TokenNotExists));

            Ok(())
        }
    }
}
