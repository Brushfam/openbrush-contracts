// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::traits::access_control::*;
use openbrush::traits::AccountId;

#[openbrush::wrapper]
pub type AccessControlEnumerableRef = dyn AccessControlEnumerable;

/// Extension of AccessControl that allows enumerating the members of each role.
#[openbrush::trait_definition]
pub trait AccessControlEnumerable {
    /// Returns one of the accounts that have `role`.
    ///
    /// Role bearers are not sorted in any particular way, and their
    /// ordering may change at any point.
    #[ink(message)]
    fn get_role_member(&self, role: RoleType, index: u32) -> Option<AccountId>;

    /// Returns the number of accounts that have `role`.
    /// Can be used together with {get_role_member} to enumerate
    /// all bearers of a role.
    #[ink(message)]
    fn get_role_member_count(&self, role: RoleType) -> u32;
}
