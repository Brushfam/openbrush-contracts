#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
pub mod my_access_control {
    use openbrush::{
        contracts::{
            access_control::*,
            psp34::extensions::{
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
        psp34: psp34::Data,
        #[storage_field]
        access: access_control::Data,
    }

    // You can manually set the number for the role.
    // But better to use a hash of the variable name.
    // It will generate a unique identifier of this role.
    // And will reduce the chance to have overlapping roles.
    const MINTER: RoleType = ink::selector_id!("MINTER");

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

    impl PSP34BurnableImpl for Contract {}

    impl PSP34Burnable for Contract {
        #[ink(message)]
        #[modifiers(only_role(MINTER))]
        fn burn(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error> {
            PSP34BurnableImpl::burn(self, account, id)
        }
    }

    impl PSP34MintableImpl for Contract {}

    impl PSP34Mintable for Contract {
        #[ink(message)]
        fn mint(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error> {
            PSP34MintableImpl::mint(self, account, id)
        }
    }

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

    impl access_control::MembersManagerImpl for Contract {}

    impl access_control::MembersManager for Contract {
        fn _has_role(&self, role: RoleType, address: &AccountId) -> bool {
            access_control::MembersManagerImpl::_has_role(self, role, address)
        }

        fn _add(&mut self, role: RoleType, member: &AccountId) {
            access_control::MembersManagerImpl::_add(self, role, member)
        }

        fn _remove(&mut self, role: RoleType, member: &AccountId) {
            access_control::MembersManagerImpl::_remove(self, role, member)
        }
    }

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
