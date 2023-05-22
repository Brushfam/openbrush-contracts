#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

pub use my_psp22::*;

#[openbrush::contract]
pub mod my_psp22 {
    use openbrush::{
        contracts::psp22::*,
        traits::{
            Storage,
            StorageAsMut,
            String,
        },
    };

    #[ink(storage)]
    #[derive(Storage)]
    pub struct Contract {
        #[storage_field]
        psp22: psp22::Data,
        // fields for hater logic
        #[storage_field]
        hated_logic: HatedLogic,
    }

    #[openbrush::upgradeable_storage(STORAGE_KEY)]
    #[openbrush::accessors(HatedLogicAccessors)]
    #[derive(Storage)]
    #[derive(Debug)]
    pub struct HatedLogic {
        #[get]
        #[set]
        hated_account: AccountId,
        #[get]
        dumb_g_only: u32,
        #[set]
        dumb_s_only: u32,
    }

    pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(HatedLogic);

    impl HatedLogicAccessors for Contract {}

    impl Transfer for Contract {
        // Let's override method to reject transactions to bad account
        fn _before_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            to: Option<&AccountId>,
            _amount: &Balance,
        ) -> Result<(), PSP22Error> {
            if to == Some(&self.data::<HatedLogic>().hated_account) {
                return Err(PSP22Error::Custom(String::from("I hate this account!")))
            }
            Ok(())
        }
    }

    impl PSP22 for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self {
                psp22: Default::default(),
                hated_logic: HatedLogic {
                    hated_account: [255; 32].into(),
                    dumb_g_only: 0,
                    dumb_s_only: 0
                },
            };

            instance
                ._mint_to(Self::env().caller(), total_supply)
                .expect("Should mint");

            instance
        }
        #[ink(message)]
        pub fn update_dumb_g(&mut self, value: u32) {
            self.hated_logic.dumb_g_only = value
        }
        #[ink(message)]
        pub fn return_dumb_s(&self) -> u32 {
            self.hated_logic.dumb_s_only
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use openbrush::contracts::psp22::psp22_external::PSP22;
        use crate::my_psp22::hatedlogicaccessors_external::HatedLogicAccessors;
        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::{build_message, PolkadotConfig};

        use test_helpers::{
            address_of,
            balance_of,
        };

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn assigns_initial_balance(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(100);
            let address = client
                .instantiate("my_psp22", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.balance_of(address_of!(alice)));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            };

            assert!(matches!(result.return_value(), 100));

            Ok(())
        }

        #[ink_e2e::test]
        async fn transfer_adds_amount_to_destination_account(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(100);
            let address = client
                .instantiate("my_psp22", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of!(bob), 50, vec![]));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("transfer failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let balance_of_alice = balance_of!(client, address, alice);

            let balance_of_bob = balance_of!(client, address, bob);

            assert_eq!(balance_of_bob, 50, "Bob should have 50 tokens");
            assert_eq!(balance_of_alice, 50, "Alice should have 50 tokens");

            Ok(())
        }

        #[ink_e2e::test]
        async fn cannot_transfer_above_the_amount(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(100);
            let address = client
                .instantiate("my_psp22", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of!(bob), 101, vec![]));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            };

            assert!(matches!(result.return_value(), Err(PSP22Error::InsufficientBalance)));

            Ok(())
        }

        #[ink_e2e::test]
        async fn cannot_transfer_to_hated_account(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(100);
            let address = client
                .instantiate("my_psp22", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of!(bob), 10, vec![]));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("transfer failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let balance_of_bob = balance_of!(client, address, bob);

            assert!(matches!(balance_of_bob, 10));

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.set_hated_account(address_of!(bob)));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("set_hated_account failed")
            };

            assert!(matches!(result.return_value(), ()));

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of!(bob), 10, vec![]));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            };

            assert!(matches!(result.return_value(), Err(PSP22Error::Custom(_))));

            let balance_of_bob = balance_of!(client, address, bob);

            assert!(matches!(balance_of_bob, 10));

            Ok(())
        }

        #[ink_e2e::test]
        async fn  only_set() -> E2EResult<()> {
            let constructor = ContractRef::new(0);
            let address = client
                .instantiate("my_psp22", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.return_dumb_s());
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("return_dumb_s failed")
            };

            assert!(matches!(result.return_value(), 0));

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.set_dumb_s_only(10));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("set_dumb_s_only failed")
            };

            assert!(matches!(result.return_value(), ()));

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.return_dumb_s());
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("return_dumb_s failed")
            };

            assert!(matches!(result.return_value(), 10));

            Ok(())
        }

    }
}
