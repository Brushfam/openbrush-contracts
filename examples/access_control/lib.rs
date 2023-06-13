#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::implementation(PSP34, PSP34Burnable, PSP34Mintable, AccessControl)]
#[openbrush::contract]
pub mod my_access_control {
    use openbrush::{
        modifiers,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        access: access_control::Data,
    }

    // You can manually set the number for the role.
    // But better to use a hash of the variable name.
    // It will generate a unique identifier of this role.
    // And will reduce the chance to have overlapping roles.
    const MINTER: RoleType = ink::selector_id!("MINTER");

    #[default_impl(PSP34Burnable)]
    #[modifiers(only_role(MINTER))]
    fn burn() {}

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();

            let caller = instance.env().caller();
            access_control::Internal::_init_with_admin(&mut instance, caller);
            // We grant minter role to caller in constructor, so he can mint/burn tokens
            AccessControl::grant_role(&mut instance, MINTER, caller).expect("Should grant MINTER role");

            instance
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use openbrush::contracts::{
            access_control::accesscontrol_external::AccessControl,
            psp34::{
                extensions::{
                    burnable::psp34burnable_external::PSP34Burnable,
                    mintable::psp34mintable_external::PSP34Mintable,
                },
                psp34_external::PSP34,
            },
        };

        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::{build_message, PolkadotConfig};

        use openbrush::contracts::access_control::DEFAULT_ADMIN_ROLE;

        use test_helpers::{
            address_of,
            grant_role,
            has_role,
            mint,
            mint_dry_run,
            revoke_role,
        };

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn only_minter_role_is_allowed_to_mint(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(has_role!(client, address, MINTER, bob), false);

            assert!(matches!(mint_dry_run!(client, address, bob, bob, Id::U8(0)), Err(_)));

            assert_eq!(grant_role!(client, address, MINTER, bob), Ok(()));

            assert_eq!(has_role!(client, address, MINTER, bob), true);

            assert_eq!(mint!(client, address, bob, bob, Id::U8(0)), Ok(()));

            let owner_of = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.owner_of(Id::U8(0)));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(owner_of, Some(address_of!(bob)));

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_grant_initial_roles_to_default_signer(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(has_role!(client, address, MINTER, alice), true);
            assert_eq!(has_role!(client, address, DEFAULT_ADMIN_ROLE, alice), true);

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_not_grant_initial_roles_for_random_role(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(has_role!(client, address, MINTER, bob), false);
            assert_eq!(has_role!(client, address, DEFAULT_ADMIN_ROLE, bob), false);

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_grant_role(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(has_role!(client, address, MINTER, bob), false);

            assert_eq!(grant_role!(client, address, MINTER, bob), Ok(()));

            assert_eq!(has_role!(client, address, MINTER, bob), true);

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_not_change_old_roles_after_grant_role(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(has_role!(client, address, MINTER, bob), false);
            assert_eq!(has_role!(client, address, DEFAULT_ADMIN_ROLE, bob), false);
            assert_eq!(has_role!(client, address, DEFAULT_ADMIN_ROLE, alice), true);
            assert_eq!(has_role!(client, address, MINTER, alice), true);

            assert_eq!(grant_role!(client, address, MINTER, bob), Ok(()));

            assert_eq!(has_role!(client, address, MINTER, bob), true);
            assert_eq!(has_role!(client, address, DEFAULT_ADMIN_ROLE, bob), false);
            assert_eq!(has_role!(client, address, DEFAULT_ADMIN_ROLE, alice), true);
            assert_eq!(has_role!(client, address, MINTER, alice), true);

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_revoke_role(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(has_role!(client, address, MINTER, bob), false);

            assert_eq!(grant_role!(client, address, MINTER, bob), Ok(()));

            assert_eq!(has_role!(client, address, MINTER, bob), true);

            let revoke_role = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.revoke_role(MINTER, address_of!(bob)));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            }
            .return_value();

            assert_eq!(revoke_role, Ok(()));

            assert_eq!(has_role!(client, address, MINTER, bob), false);

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_renounce_role(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(has_role!(client, address, MINTER, alice), true);

            let renounce_role = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.renounce_role(MINTER, address_of!(alice)));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            }
            .return_value();

            assert_eq!(renounce_role, Ok(()));

            assert_eq!(has_role!(client, address, MINTER, alice), false);

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_reject_when_grant_or_revoke_not_by_admin_role(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(grant_role!(client, address, MINTER, bob), Ok(()));

            let grant_role = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.grant_role(MINTER, address_of!(charlie)));
                client.call_dry_run(&ink_e2e::bob(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(grant_role, Err(_)));

            let revoke_role = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.revoke_role(MINTER, address_of!(charlie)));
                client.call_dry_run(&ink_e2e::bob(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(revoke_role, Err(_)));

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_reject_when_renounce_not_self_role(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(grant_role!(client, address, MINTER, bob), Ok(()));
            assert_eq!(has_role!(client, address, MINTER, bob), true);

            let renounce_role = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.renounce_role(MINTER, address_of!(bob)));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(renounce_role, Err(_)));

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_reject_burn_if_no_minter_role(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(grant_role!(client, address, MINTER, bob), Ok(()));
            assert_eq!(has_role!(client, address, MINTER, bob), true);

            assert_eq!(mint!(client, address, bob, bob, Id::U8(0)), Ok(()));

            let owner_of = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.owner_of(Id::U8(0)));
                client.call_dry_run(&ink_e2e::bob(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(owner_of, Some(address_of!(bob)));

            assert_eq!(revoke_role!(client, address, MINTER, bob), Ok(()));
            assert_eq!(has_role!(client, address, MINTER, bob), false);

            let burn = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.burn(address_of!(bob), Id::U8(0)));
                client.call_dry_run(&ink_e2e::bob(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(burn, Err(_)));

            Ok(())
        }
    }
}
