#![cfg_attr(not(feature = "std"), no_std, no_main)]

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

    #[default_impl(PSP34Mintable)]
    #[modifiers(only_role(MINTER))]
    fn mint() {}

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();

            let caller = instance.env().caller();
            access_control::Internal::_init_with_admin(&mut instance, Some(caller));
            // We grant minter role to caller in constructor, so he can mint/burn tokens
            AccessControl::grant_role(&mut instance, MINTER, Some(caller)).expect("Should grant MINTER role");

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

        use openbrush::contracts::access_control::DEFAULT_ADMIN_ROLE;

        use test_helpers::{
            address_of,
            grant_role,
            has_role,
            mint,
            revoke_role,
            method_call_dry_run,
            method_call,
        };
        use ink_e2e::ContractsBackend;


        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn only_minter_role_is_allowed_to_mint<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let constructor = ContractRef::new();

            let contract = client
                .instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let mut call = contract.call::<Contract>();
            assert_eq!(has_role!(client, call, MINTER, Bob), false);
            assert_eq!(has_role!(client, call, DEFAULT_ADMIN_ROLE, Bob), false);
            assert_eq!(has_role!(client, call, DEFAULT_ADMIN_ROLE, Alice), true);
            assert_eq!(has_role!(client, call, MINTER, Alice), true);

            assert_eq!(grant_role!(client, call, MINTER, Bob), Ok(()));

            assert_eq!(has_role!(client, call, MINTER, Bob), true);
            assert_eq!(has_role!(client, call, DEFAULT_ADMIN_ROLE, Bob), false);
            assert_eq!(has_role!(client, call, DEFAULT_ADMIN_ROLE, Alice), true);
            assert_eq!(has_role!(client, call, MINTER, Alice), true);

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_grant_initial_roles_to_default_signer<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let constructor = ContractRef::new();

            let contract = client
                .instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let  call = contract.call::<Contract>();

            assert_eq!(has_role!(client, call, MINTER, Alice), true);
            assert_eq!(has_role!(client, call, DEFAULT_ADMIN_ROLE, Alice), true);

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_not_grant_initial_roles_for_random_role<Client: E2EBackend>(
            mut client: Client,
        ) -> E2EResult<()> {
            let constructor = ContractRef::new();

            let contract = client
                .instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let call = contract.call::<Contract>();

            assert_eq!(has_role!(client, call, MINTER, Bob), false);
            assert_eq!(has_role!(client, call, DEFAULT_ADMIN_ROLE, Bob), false);

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_grant_role<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let constructor = ContractRef::new();

            let contract = client
                .instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let mut call = contract.call::<Contract>();

            assert_eq!(has_role!(client, call, MINTER, Bob), false);

            assert_eq!(grant_role!(client, call, MINTER, Bob), Ok(()));

            assert_eq!(has_role!(client, call, MINTER, Bob), true);

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_not_change_old_roles_after_grant_role<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let constructor = ContractRef::new();

            let contract = client
                .instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let mut call = contract.call::<Contract>();

            assert_eq!(has_role!(client, call, MINTER, Bob), false);
            assert_eq!(has_role!(client, call, DEFAULT_ADMIN_ROLE, Bob), false);
            assert_eq!(has_role!(client, call, DEFAULT_ADMIN_ROLE, Alice), true);
            assert_eq!(has_role!(client, call, MINTER, Alice), true);

            assert_eq!(grant_role!(client, call, MINTER, Bob), Ok(()));

            assert_eq!(has_role!(client, call, MINTER, Bob), true);
            assert_eq!(has_role!(client, call, DEFAULT_ADMIN_ROLE, Bob), false);
            assert_eq!(has_role!(client, call, DEFAULT_ADMIN_ROLE, Alice), true);
            assert_eq!(has_role!(client, call, MINTER, Alice), true);

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_revoke_role<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let constructor = ContractRef::new();

            let contract = client
                .instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let mut call = contract.call::<Contract>();

            assert_eq!(has_role!(client, call, MINTER, Bob), false);

            assert_eq!(grant_role!(client, call, MINTER, Bob), Ok(()));

            assert_eq!(has_role!(client, call, MINTER, Bob), true);

            let revoke_role = method_call!(client, call, revoke_role(MINTER, Some(address_of!(Bob))));

            assert_eq!(revoke_role, Ok(()));

            assert_eq!(has_role!(client, call, MINTER, Bob), false);

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_renounce_role<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let constructor = ContractRef::new();

            let contract = client
                .instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let mut call = contract.call::<Contract>();

            assert_eq!(has_role!(client, call, MINTER, Alice), true);

            let _revoke_role = method_call!(client, call, renounce_role(MINTER, Some(address_of!(Alice))));

            assert_eq!(has_role!(client, call, MINTER, Alice), false);

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_reject_when_grant_or_revoke_not_by_admin_role(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let constructor = ContractRef::new();

            let contract = client
                .instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let mut call = contract.call::<Contract>();

            assert_eq!(grant_role!(client, call, MINTER, Bob), Ok(()));

            let grant_role = method_call_dry_run!(client, call, bob, grant_role(MINTER, Some(address_of!(Charlie))));

            assert!(matches!(grant_role, Err(_)));

            let revoke_role = method_call_dry_run!(client, call, bob, revoke_role(MINTER, Some(address_of!(Charlie))));

            assert!(matches!(revoke_role, Err(_)));

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_reject_when_renounce_not_self_role<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let constructor = ContractRef::new();

            let contract = client
                .instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let mut call = contract.call::<Contract>();

            assert_eq!(grant_role!(client, call, MINTER, Bob), Ok(()));
            assert_eq!(has_role!(client, call, MINTER, Bob), true);

            let renounce_role = method_call_dry_run!(client, call, renounce_role(MINTER, Some(address_of!(Bob))));

            assert!(matches!(renounce_role, Err(_)));

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_reject_burn_if_no_minter_role<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let constructor = ContractRef::new();

            let contract = client
                .instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let mut call = contract.call::<Contract>();

            assert_eq!(grant_role!(client, call, MINTER, Bob), Ok(()));
            assert_eq!(has_role!(client, call, MINTER, Bob), true);

            assert_eq!(mint!(client, call, bob, Bob, Id::U8(0)), Ok(()));

            let owner_of =  method_call_dry_run!(client, call, bob, owner_of(Id::U8(0)));

            assert_eq!(owner_of, Some(address_of!(Bob)));

            assert_eq!(revoke_role!(client, call, MINTER, Bob), Ok(()));
            assert_eq!(has_role!(client, call, MINTER, Bob), false);

            let burn = method_call_dry_run!(client, call, bob, burn(address_of!(Bob), Id::U8(0)));

            assert!(matches!(burn, Err(_)));

            Ok(())
        }
    }
}
