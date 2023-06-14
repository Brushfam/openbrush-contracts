#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::contract]
pub mod my_psp34_metadata {
    use openbrush::{
        contracts::psp34::extensions::metadata::*,
        traits::{
            Storage,
            String,
        },
    };

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    impl PSP34MetadataImpl for Contract {}

    impl PSP34Metadata for Contract {
        #[ink(message)]
        fn get_attribute(&self, id: Id, key: String) -> Option<String> {
            PSP34MetadataImpl::get_attribute(self, id, key)
        }
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

    impl metadata::InternalImpl for Contract {}

    impl metadata::Internal for Contract {
        fn _emit_attribute_set_event(&self, id: Id, key: String, data: String) {
            metadata::InternalImpl::_emit_attribute_set_event(self, id, key, data)
        }

        fn _set_attribute(&mut self, id: Id, key: String, value: String) {
            metadata::InternalImpl::_set_attribute(self, id, key, value)
        }
    }

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
        /// A constructor which mints the first token to the owner
        #[ink(constructor)]
        pub fn new(id: Id, name: String, symbol: String) -> Self {
            let mut instance = Self::default();

            let name_key = String::from("name");
            let symbol_key = String::from("symbol");
            metadata::Internal::_set_attribute(&mut instance, id.clone(), name_key, name);
            metadata::Internal::_set_attribute(&mut instance, id, symbol_key, symbol);

            instance
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use openbrush::contracts::psp34::extensions::metadata::psp34metadata_external::PSP34Metadata;

        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::build_message;

        use openbrush::traits::String;

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn metadata_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let id = Id::U8(0);
            let name = String::from("My PSP34");
            let symbol = String::from("MPS34");

            let constructor = ContractRef::new(id.clone(), name.clone(), symbol.clone());
            let address = client
                .instantiate("my_psp34_metadata", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let result_name = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.get_attribute(id.clone(), String::from("name")));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            let result_symbol = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.get_attribute(id.clone(), String::from("symbol")));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(result_name, Some(name));
            assert_eq!(result_symbol, Some(symbol));

            Ok(())
        }
    }
}
