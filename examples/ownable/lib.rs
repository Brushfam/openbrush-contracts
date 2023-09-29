#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(Ownable, PSP37, PSP37Burnable, PSP37Mintable)]
#[openbrush::contract]
pub mod ownable {
    use openbrush::{
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

    #[default_impl(PSP37Mintable)]
    #[modifiers(only_owner)]
    fn mint(&mut self) {}

    #[default_impl(PSP37Burnable)]
    #[modifiers(only_owner)]
    fn burn(&mut self) {}

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

            assert_eq!(owner, Some(address_of!(Alice)));

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

            assert_eq!(owner, Some(address_of!(Alice)));

            let mint_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(Bob), vec![(Id::U8(0), 1)]));
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

            assert_eq!(owner, Some(address_of!(Alice)));

            let mint_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(Bob), ids_amounts.clone()));
                client.call_dry_run(&ink_e2e::bob(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(mint_tx, Err(_)));

            let balance_before = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.balance_of(address_of!(Bob), Some(token.clone())));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(balance_before, 0);

            let transfer_ownership_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer_ownership(Some(address_of!(Bob))));
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

            assert_eq!(owner, Some(address_of!(Bob)));

            let mint_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(Bob), ids_amounts.clone()));
                client.call(&ink_e2e::bob(), _msg, 0, None).await.expect("mint failed")
            }
            .return_value();

            assert_eq!(mint_tx, Ok(()));

            let balance_after = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.balance_of(address_of!(Bob), Some(token.clone())));
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

            assert_eq!(owner, Some(address_of!(Alice)));

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

            assert_eq!(owner, None);

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

            assert_eq!(owner, Some(address_of!(Alice)));

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

            assert_eq!(owner, Some(address_of!(Alice)));

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

            assert_eq!(owner, Some(address_of!(Alice)));

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

            assert_eq!(owner, Some(address_of!(Alice)));

            Ok(())
        }
    }
}
