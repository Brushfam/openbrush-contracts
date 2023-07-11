#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(AccessData);
// we declare the data struct outside of the contract
// since we need to expand the macroes on it before the openbrush::contract macro expansion
#[openbrush::storage_item(STORAGE_KEY)]
#[openbrush::accessors(AccessDataAccessors)]
#[derive(Default, Debug)]
pub struct AccessData {
    #[get]
    #[set]
    read_write: u32,
    #[get]
    read_only: u32,
    #[set]
    write_only: u32,
}

#[openbrush::contract]
pub mod accessors_attr {
    use crate::*;

    #[ink(storage)]
    #[derive(Default)]
    #[openbrush::storage]
    pub struct Contract {
        #[storage_field]
        hated_logic: AccessData,
    }

    impl AccessDataAccessors for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Default::default()
        }

        #[ink(message)]
        pub fn set_read_only(&mut self, value: u32) {
            self.hated_logic.read_only = value
        }

        #[ink(message)]
        pub fn get_write_only(&self) -> u32 {
            self.hated_logic.write_only
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use super::*;
        use crate::accessdataaccessors_external::AccessDataAccessors;
        use ink_e2e::build_message;

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn get_and_set() -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("accessors_attr", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let result = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.get_read_write());
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("get_read_write failed")
            };

            assert!(matches!(result.return_value(), 0));

            let result = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.set_read_write(10));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("update_read_only failed")
            };

            assert!(matches!(result.return_value(), ()));

            let result = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.get_read_write());
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("get_read_only failed")
            };

            assert!(matches!(result.return_value(), 10));

            Ok(())
        }

        #[ink_e2e::test]
        async fn only_set() -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("accessors_attr", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let result = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.get_write_only());
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("get_write_only failed")
            };

            assert!(matches!(result.return_value(), 0));

            let result = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.set_write_only(10));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("set_write_only failed")
            };

            assert!(matches!(result.return_value(), ()));

            let result = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.get_write_only());
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("get_write_only failed")
            };

            assert!(matches!(result.return_value(), 10));

            Ok(())
        }

        #[ink_e2e::test]
        async fn only_get() -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("accessors_attr", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let result = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.get_read_only());
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("get_read_only failed")
            };

            assert!(matches!(result.return_value(), 0));

            let result = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.set_read_only(10));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("set_read_only failed")
            };

            assert!(matches!(result.return_value(), ()));

            let result = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.get_read_only());
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("get_read_only failed")
            };

            assert!(matches!(result.return_value(), 10));

            Ok(())
        }
    }
}
