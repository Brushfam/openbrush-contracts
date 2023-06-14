#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::contract]
pub mod ownable {
    use ink::prelude::vec::Vec;
    use openbrush::{
        contracts::{
            ownable::*,
            psp37::extensions::{
                burnable::*,
                mintable::*,
            },
        },
        modifiers,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp37: psp37::Data,
        #[storage_field]
        ownable: ownable::Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            ownable::Internal::_init_with_owner(&mut instance, Self::env().caller());
            instance
        }
    }

    impl ownable::InternalImpl for Contract {}

    impl ownable::Internal for Contract {
        fn _emit_ownership_transferred_event(&self, previous: Option<AccountId>, new: Option<AccountId>) {
            ownable::InternalImpl::_emit_ownership_transferred_event(self, previous, new)
        }

        fn _init_with_owner(&mut self, owner: AccountId) {
            ownable::InternalImpl::_init_with_owner(self, owner)
        }
    }

    impl OwnableImpl for Contract {}

    impl Ownable for Contract {
        #[ink(message)]
        fn owner(&self) -> AccountId {
            OwnableImpl::owner(self)
        }

        #[ink(message)]
        fn renounce_ownership(&mut self) -> Result<(), OwnableError> {
            OwnableImpl::renounce_ownership(self)
        }

        #[ink(message)]
        fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<(), OwnableError> {
            OwnableImpl::transfer_ownership(self, new_owner)
        }
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

    impl PSP37MintableImpl for Contract {}

    impl PSP37Mintable for Contract {
        #[ink(message)]
        #[modifiers(only_owner)]
        fn mint(&mut self, to: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP37Error> {
            PSP37MintableImpl::mint(self, to, ids_amounts)
        }
    }

    impl PSP37BurnableImpl for Contract {}

    impl PSP37Burnable for Contract {
        #[ink(message)]
        #[modifiers(only_owner)]
        fn burn(&mut self, from: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP37Error> {
            PSP37BurnableImpl::burn(self, from, ids_amounts)
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use openbrush::contracts::{
            ownable::ownable_external::Ownable,
            psp37::{
                extensions::mintable::psp37mintable_external::PSP37Mintable,
                psp37_external::PSP37,
            },
        };

        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::{build_message, PolkadotConfig};

        use test_helpers::address_of;

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn owner_is_by_default_contract_deployer(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_ownable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let owner = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.owner());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(owner, address_of!(alice));

            Ok(())
        }

        #[ink_e2e::test]
        async fn only_owner_is_allowed_to_mint(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_ownable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let owner = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.owner());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(owner, address_of!(alice));

            let mint_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(bob), vec![(Id::U8(0), 1)]));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }
            .return_value();

            assert_eq!(mint_tx, Ok(()));

            Ok(())
        }

        #[ink_e2e::test]
        async fn transfer_ownership_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_ownable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let token = Id::U8(1);
            let ids_amounts = vec![(token.clone(), 123)];

            let owner = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.owner());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(owner, address_of!(alice));

            let mint_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(bob), ids_amounts.clone()));
                client.call_dry_run(&ink_e2e::bob(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(mint_tx, Err(_)));

            let balance_before = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.balance_of(address_of!(bob), Some(token.clone())));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(balance_before, 0);

            let transfer_ownership_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer_ownership(address_of!(bob)));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("transfer_ownership failed")
            }
            .return_value();

            assert_eq!(transfer_ownership_tx, Ok(()));

            let owner = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.owner());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(owner, address_of!(bob));

            let mint_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(bob), ids_amounts.clone()));
                client.call(&ink_e2e::bob(), _msg, 0, None).await.expect("mint failed")
            }
            .return_value();

            assert_eq!(mint_tx, Ok(()));

            let balance_after = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.balance_of(address_of!(bob), Some(token.clone())));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(balance_after, 123);

            Ok(())
        }

        #[ink_e2e::test]
        async fn renounce_ownership_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_ownable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let owner = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.owner());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(owner, address_of!(alice));

            let renounce_ownership_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.renounce_ownership());
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("renounce_ownership failed")
            }
            .return_value();

            assert_eq!(renounce_ownership_tx, Ok(()));

            let owner = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.owner());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(owner, AccountId::from([0xff; 32]));

            Ok(())
        }

        #[ink_e2e::test]
        async fn cannot_renounce_ownership_if_not_owner(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_ownable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let owner = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.owner());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(owner, address_of!(alice));

            let renounce_ownership_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.renounce_ownership());
                client.call_dry_run(&ink_e2e::bob(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(renounce_ownership_tx, Err(_)));

            let owner = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.owner());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(owner, address_of!(alice));

            Ok(())
        }

        #[ink_e2e::test]
        async fn cannot_transfer_ownership_if_not_owner(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_ownable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let owner = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.owner());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(owner, address_of!(alice));

            let renounce_ownership_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.renounce_ownership());
                client.call_dry_run(&ink_e2e::bob(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(renounce_ownership_tx, Err(_)));

            let owner = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.owner());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(owner, address_of!(alice));

            Ok(())
        }
    }
}
