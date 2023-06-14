#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::contract]
pub mod my_psp37 {
    use ink::prelude::vec::Vec;
    use openbrush::{
        contracts::psp37::extensions::metadata::*,
        traits::{
            Storage,
            String,
        },
    };

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp37: psp37::Data,
        #[storage_field]
        metadata: metadata::Data,
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

    impl metadata::InternalImpl for Contract {}

    impl metadata::Internal for Contract {
        fn _emit_attribute_set_event(&self, id: &Id, key: &String, data: &String) {
            metadata::InternalImpl::_emit_attribute_set_event(self, id, key, data);
        }

        fn _set_attribute(&mut self, id: &Id, key: &String, data: &String) -> Result<(), PSP37Error> {
            metadata::InternalImpl::_set_attribute(self, id, key, data)
        }

        fn _get_attribute(&self, id: &Id, key: &String) -> Option<String> {
            metadata::InternalImpl::_get_attribute(self, id, key)
        }
    }

    impl PSP37MetadataImpl for Contract {}

    impl PSP37Metadata for Contract {
        #[ink(message)]
        fn get_attribute(&self, id: Id, key: String) -> Option<String> {
            PSP37MetadataImpl::get_attribute(self, id, key)
        }
    }

    impl Contract {
        /// contract constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn set_attribute(&mut self, id: Id, key: String, data: String) -> Result<(), PSP37Error> {
            metadata::Internal::_set_attribute(self, &id, &key, &data)
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use openbrush::contracts::psp37::{
            extensions::metadata::psp37metadata_external::PSP37Metadata,
            psp37_external::PSP37,
        };

        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::{build_message, PolkadotConfig};

        use test_helpers::address_of;

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn metadata_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_psp37_metadata", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let id = Id::U8(0);
            let attr = String::from("https://www.727.ventures/");

            let attribute = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.get_attribute(id.clone(), attr.clone()));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(attribute, None);

            let set_attribute_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| {
                    contract.set_attribute(id.clone(), attr.clone(), String::from("https://www.727.ventures/"))
                });
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            }
            .return_value();

            assert_eq!(set_attribute_tx, Ok(()));

            let attribute = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.get_attribute(id.clone(), attr.clone()));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(attribute, Some(String::from("https://www.727.ventures/")));

            Ok(())
        }
    }
}
