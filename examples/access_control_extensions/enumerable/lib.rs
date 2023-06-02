#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
pub mod my_access_control {
    use openbrush::{
        contracts::access_control::extensions::enumerable::*,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        access: access_control::Data,
        #[storage_field]
        enumerable: enumerable::Data,
    }

    // You can manually set the number for the role.
    // But better to use a hash of the variable name.
    // It will generate a unique identifier of this role.
    // And will reduce the chance to have overlapping roles.
    const MINTER: RoleType = ink::selector_id!("MINTER");

    impl AccessControlImpl for Contract {}

    impl AccessControl for Contract {
        #[ink(message)]
        fn has_role(&self, role: RoleType, address: AccountId) -> bool {
            AccessControlImpl::has_role(self, role, address)
        }

        #[ink(message)]
        fn get_role_admin(&self, role: RoleType) -> RoleType {
            AccessControlImpl::get_role_admin(self, role)
        }

        #[ink(message)]
        fn grant_role(&mut self, role: RoleType, account: AccountId) -> Result<(), AccessControlError> {
            AccessControlImpl::grant_role(self, role, account)
        }

        #[ink(message)]
        fn revoke_role(&mut self, role: RoleType, account: AccountId) -> Result<(), AccessControlError> {
            AccessControlImpl::revoke_role(self, role, account)
        }

        #[ink(message)]
        fn renounce_role(&mut self, role: RoleType, account: AccountId) -> Result<(), AccessControlError> {
            AccessControlImpl::renounce_role(self, role, account)
        }
    }

    impl access_control::InternalImpl for Contract {}

    impl access_control::Internal for Contract {
        fn _emit_role_admin_changed(&mut self, _role: RoleType, _previous: RoleType, _new: RoleType) {
            access_control::InternalImpl::_emit_role_admin_changed(self, _role, _previous, _new);
        }

        fn _emit_role_granted(&mut self, _role: RoleType, _grantee: AccountId, _grantor: Option<AccountId>) {
            access_control::InternalImpl::_emit_role_granted(self, _role, _grantee, _grantor);
        }

        fn _emit_role_revoked(&mut self, _role: RoleType, _account: AccountId, _sender: AccountId) {
            access_control::InternalImpl::_emit_role_revoked(self, _role, _account, _sender);
        }

        fn _default_admin() -> RoleType {
            <Self as access_control::InternalImpl>::_default_admin()
        }

        fn _init_with_caller(&mut self) {
            access_control::InternalImpl::_init_with_caller(self);
        }

        fn _init_with_admin(&mut self, admin: AccountId) {
            access_control::InternalImpl::_init_with_admin(self, admin);
        }

        fn _setup_role(&mut self, role: RoleType, member: AccountId) {
            access_control::InternalImpl::_setup_role(self, role, member);
        }

        fn _do_revoke_role(&mut self, role: RoleType, account: AccountId) {
            access_control::InternalImpl::_do_revoke_role(self, role, account);
        }

        fn _set_role_admin(&mut self, role: RoleType, new_admin: RoleType) {
            access_control::InternalImpl::_set_role_admin(self, role, new_admin);
        }

        fn _check_role(&self, role: RoleType, account: AccountId) -> Result<(), AccessControlError> {
            access_control::InternalImpl::_check_role(self, role, account)
        }

        fn _get_role_admin(&self, role: RoleType) -> RoleType {
            access_control::InternalImpl::_get_role_admin(self, role)
        }
    }

    impl enumerable::MembersManagerImpl for Contract {}

    impl access_control::MembersManager for Contract {
        fn _has_role(&self, role: RoleType, address: &AccountId) -> bool {
            enumerable::MembersManagerImpl::_has_role(self, role, address)
        }

        fn _add(&mut self, role: RoleType, member: &AccountId) {
            enumerable::MembersManagerImpl::_add(self, role, member)
        }

        fn _remove(&mut self, role: RoleType, member: &AccountId) {
            enumerable::MembersManagerImpl::_remove(self, role, member)
        }
    }

    impl AccessControlEnumerableImpl for Contract {}

    impl AccessControlEnumerable for Contract {
        #[ink(message)]
        fn get_role_member(&self, role: RoleType, index: u32) -> Option<AccountId> {
            AccessControlEnumerableImpl::get_role_member(self, role, index)
        }

        #[ink(message)]
        fn get_role_member_count(&self, role: RoleType) -> u32 {
            AccessControlEnumerableImpl::get_role_member_count(self, role)
        }
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();

            let caller = Self::env().caller();
            access_control::Internal::_init_with_admin(&mut instance, caller);
            // We grant minter role to caller in constructor, so he can mint/burn tokens
            AccessControl::grant_role(&mut instance, MINTER, caller).expect("Should grant MINTER role");
            assert_eq!(AccessControlEnumerable::get_role_member_count(&instance, MINTER), 1);

            instance
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use openbrush::contracts::access_control::{
            accesscontrol_external::AccessControl,
            extensions::enumerable::accesscontrolenumerable_external::AccessControlEnumerable,
        };

        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::{build_message, PolkadotConfig};

        use test_helpers::{
            address_of,
            get_role_member,
            get_role_member_count,
            grant_role,
            has_role,
            revoke_role,
        };

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn should_have_not_member(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_access_control_enumerable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(get_role_member!(client, address, MINTER, 1), None);

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_get_role_member(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_access_control_enumerable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(get_role_member!(client, address, MINTER, 0), Some(address_of!(alice)));

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_grant_roles_and_get_role_members(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_access_control_enumerable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(has_role!(client, address, MINTER, bob), false);

            assert_eq!(grant_role!(client, address, MINTER, bob), Ok(()));

            assert_eq!(get_role_member!(client, address, MINTER, 1), Some(address_of!(bob)));

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_revoker_and_count_roles(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_access_control_enumerable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(has_role!(client, address, MINTER, bob), false);

            assert_eq!(has_role!(client, address, MINTER, alice), true);

            assert_eq!(get_role_member_count!(client, address, MINTER), 1);

            assert_eq!(revoke_role!(client, address, MINTER, alice), Ok(()));

            assert_eq!(get_role_member_count!(client, address, MINTER), 0);

            Ok(())
        }
    }
}
