#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

pub use accessors_attr::*;

#[openbrush::contract]
pub mod accessors_attr {
    use openbrush::{
        traits::{
            Storage,
            String,
        },
    };

    #[ink(storage)]
    #[derive(Storage)]
    pub struct Contract {
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
        dumb_g_s: u32,
        #[get]
        dumb_g_only: u32,
        #[set]
        dumb_s_only: u32,
    }

    pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(HatedLogic);

    impl HatedLogicAccessors for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let instance = Self {
                hated_logic: HatedLogic {
                    dumb_g_s: 0,
                    dumb_g_only: 0,
                    dumb_s_only: 0
                },
            };
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
        use crate::accessors_attr::hatedlogicaccessors_external::HatedLogicAccessors;
        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::{build_message, PolkadotConfig};

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn  get_and_set() -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("accessors_attr", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.get_dumb_g_s());
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("get_dumb_g_s failed")
            };

            assert!(matches!(result.return_value(), 0));

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.set_dumb_g_s(10));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("update_dumb_g_only failed")
            };

            assert!(matches!(result.return_value(), ()));

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.get_dumb_g_s());
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("get_dumb_g_only failed")
            };

            assert!(matches!(result.return_value(), 10));

            Ok(())
        }

        #[ink_e2e::test]
        async fn  only_set() -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("accessors_attr", &ink_e2e::alice(), constructor, 0, None)
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

        #[ink_e2e::test]
        async fn  only_get() -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("accessors_attr", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.get_dumb_g_only());
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("get_dumb_g_only failed")
            };

            assert!(matches!(result.return_value(), 0));

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.update_dumb_g(10));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("update_dumb_g_only failed")
            };

            assert!(matches!(result.return_value(), ()));

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.get_dumb_g_only());
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("get_dumb_g_only failed")
            };

            assert!(matches!(result.return_value(), 10));

            Ok(())
        }
    }
}
