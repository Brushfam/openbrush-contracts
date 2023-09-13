#![cfg_attr(not(feature = "std"), no_std, no_main)]

use openbrush::traits::Storage;

// we declare the data struct outside of the contract
// since we need to expand the macros on it before the openbrush::contract macro expansion

#[openbrush::accessors(AccessDataAccessors)]
#[derive(Default, Debug)]
#[ink::storage_item]
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
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Storage, Default)]
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
        use test_helpers::{
            method_call_dry_run,
            method_call,
        };


        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn get_and_set() -> E2EResult<()> {
            let constructor = ContractRef::new();
            let contract = client
                .instantiate("accessors_attr", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let mut call = contract.call::<Contract>();

            assert!(matches!(method_call_dry_run!(client, call, get_read_write()), 0));

            method_call!(client, call, set_read_write(10));

            assert!(matches!(method_call_dry_run!(client, call, get_read_write()), 10));

            Ok(())
        }

        #[ink_e2e::test]
        async fn only_set() -> E2EResult<()> {
            let constructor = ContractRef::new();
            let contract = client
                .instantiate("accessors_attr", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let mut call = contract.call::<Contract>();

            assert!(matches!(method_call_dry_run!(client, call, get_write_only()), 0));

            method_call!(client, call, set_write_only(10));

            assert!(matches!(method_call_dry_run!(client, call, get_write_only()), 10));

            Ok(())
        }

        #[ink_e2e::test]
        async fn only_get() -> E2EResult<()> {
            let constructor = ContractRef::new();
            let contract = client
                .instantiate("accessors_attr", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let mut call = contract.call::<Contract>();

            assert!(matches!(method_call_dry_run!(client, call, get_read_only()), 0));

            method_call!(client, call, set_read_only(10));

            assert!(matches!(method_call_dry_run!(client, call, get_read_only()), 10));

            Ok(())
        }
    }
}
