// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

#[cfg(feature = "access_control")]
#[openbrush::implementation(AccessControl, AccessControlEnumerable)]
#[openbrush::contract]
mod access_control_enumerable {
    use ::ink::env::DefaultEnvironment;
    use ink::env::test::DefaultAccounts;
    use openbrush::{test_utils::accounts, traits::Storage};

    // You can manually set the number for the role.
    // But better to use a hash of the variable name.
    // It will generate a unique identifier of this role.
    // And will reduce the chance to have overlapping roles.
    const MINTER: RoleType = ink::selector_id!("MINTER");
    const PAUSER: RoleType = ink::selector_id!("PAUSER");

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct AccessControlStruct {
        #[storage_field]
        access: access_control::Data,
        #[storage_field]
        enumerable: enumerable::Data,
    }

    impl AccessControlStruct {
        #[ink(constructor)]
        pub fn new(admin: AccountId) -> Self {
            let mut instance = Self::default();

            access_control::Internal::_init_with_admin(&mut instance, Some(admin));

            instance
        }
    }

    fn setup() -> DefaultAccounts<DefaultEnvironment> {
        let accounts = accounts();

        accounts
    }

    #[ink::test]
    fn should_change_role_member_count() {
        let accounts = setup();
        let alice = accounts.alice;
        let mut access_control = AccessControlStruct::new(alice);

        assert_eq!(
            AccessControlEnumerable::get_role_member_count(&mut access_control, PAUSER),
            0
        );

        assert!(AccessControl::grant_role(&mut access_control, PAUSER, Some(alice)).is_ok());
        assert_eq!(
            AccessControlEnumerable::get_role_member_count(&mut access_control, PAUSER),
            1
        );

        assert!(AccessControl::grant_role(&mut access_control, PAUSER, Some(accounts.bob)).is_ok());
        assert_eq!(
            AccessControlEnumerable::get_role_member_count(&mut access_control, PAUSER),
            2
        );

        assert!(AccessControl::revoke_role(&mut access_control, PAUSER, Some(alice)).is_ok());
        assert!(AccessControl::grant_role(&mut access_control, MINTER, Some(alice)).is_ok());
        assert_eq!(
            AccessControlEnumerable::get_role_member_count(&mut access_control, PAUSER),
            1
        );
        assert_eq!(
            AccessControlEnumerable::get_role_member_count(&mut access_control, MINTER),
            1
        );
    }

    #[ink::test]
    fn should_return_role_member() {
        let accounts = setup();
        let alice = accounts.alice;
        let mut access_control = AccessControlStruct::new(alice);

        assert!(AccessControl::grant_role(&mut access_control, PAUSER, Some(accounts.bob)).is_ok());
        assert!(AccessControl::grant_role(&mut access_control, PAUSER, Some(alice)).is_ok());
        assert!(AccessControl::grant_role(&mut access_control, PAUSER, Some(accounts.eve)).is_ok());

        assert_eq!(
            AccessControlEnumerable::get_role_member(&mut access_control, PAUSER, 1),
            Some(alice)
        )
    }

    #[ink::test]
    fn get_role_member_fails() {
        let accounts = setup();
        let alice = accounts.alice;
        let mut access_control = AccessControlStruct::new(alice);

        assert!(AccessControl::grant_role(&mut access_control, PAUSER, Some(accounts.bob)).is_ok());
        assert!(AccessControl::grant_role(&mut access_control, PAUSER, Some(alice)).is_ok());
        assert_eq!(
            AccessControlEnumerable::get_role_member(&mut access_control, PAUSER, 1),
            Some(alice)
        );

        assert!(AccessControl::revoke_role(&mut access_control, PAUSER, Some(alice)).is_ok());
        assert_eq!(
            AccessControlEnumerable::get_role_member(&mut access_control, PAUSER, 1),
            None
        )
    }
}
