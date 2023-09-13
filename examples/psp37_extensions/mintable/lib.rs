#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP37, PSP37Mintable)]
#[openbrush::contract]
pub mod my_psp37 {
    use openbrush::traits::Storage;

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp37: psp37::Data,
    }

    impl Contract {
        /// contract constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use openbrush::contracts::psp37::{
            extensions::mintable::psp37mintable_external::PSP37Mintable,
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
        async fn mint_works<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let contract = client
                .instantiate("my_psp37_mintable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let call = contract.call::<Contract>();

            let token_1 = Id::U8(0);
            let token_2 = Id::U8(1);

            let amount_1 = 1;
            let amount_2 = 2;

            assert_eq!(balance_of_37!(client, call, Alice, Some(token_1.clone())), 0);
            assert_eq!(balance_of_37!(client, call, Bob, Some(token_1.clone())), 0);
            assert_eq!(balance_of_37!(client, call, Alice, Some(token_2.clone())), 0);
            assert_eq!(balance_of_37!(client, call, Bob, Some(token_2.clone())), 0);

            let mint_tx = {
                let _msg = call.mint(address_of!(Alice), vec![(token_1.clone(), amount_1)]);
                client
                    .call(&ink_e2e::alice(), &_msg, 0, None)
                    .await
                    .expect("mint failed")
            }
            .return_value();

            assert_eq!(mint_tx, Ok(()));

            let mint_tx = {
                let _msg = call.mint(address_of!(Alice), vec![(token_2.clone(), amount_2)]);
                client
                    .call(&ink_e2e::alice(), &_msg, 0, None)
                    .await
                    .expect("mint failed")
            }
            .return_value();

            assert_eq!(mint_tx, Ok(()));

            let mint_tx = {
                let _msg = call.mint(
                        address_of!(Bob),
                        vec![(token_1.clone(), amount_1), (token_2.clone(), amount_2)],
                    );
                client
                    .call(&ink_e2e::alice(), &_msg, 0, None)
                    .await
                    .expect("mint failed")
            }
            .return_value();

            assert_eq!(mint_tx, Ok(()));

            assert_eq!(balance_of_37!(client, call, Alice, Some(token_1.clone())), amount_1);
            assert_eq!(balance_of_37!(client, call, Bob, Some(token_1.clone())), amount_1);
            assert_eq!(balance_of_37!(client, call, Alice, Some(token_2.clone())), amount_2);
            assert_eq!(balance_of_37!(client, call, Bob, Some(token_2.clone())), amount_2);

            Ok(())
        }
    }
}
