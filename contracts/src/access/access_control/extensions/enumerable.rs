// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::{
    access_control,
    access_control::extensions::enumerable,
    traits::access_control::{
        extensions::enumerable::*,
        *,
    },
};
pub use access_control::{
    AccessControlImpl,
    Internal as _,
    InternalImpl as _,
};
use openbrush::{
    storage::{
        Mapping,
        MultiMapping,
        ValueGuard,
    },
    traits::{
        AccountId,
        Storage,
    },
};

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    pub admin_roles: Mapping<RoleType, RoleType, ValueGuard<RoleType>>,
    pub role_members: MultiMapping<RoleType, Option<AccountId>, ValueGuard<RoleType>>,
}

pub trait MembersManagerImpl: Storage<Data> {
    fn _has_role(&self, role: RoleType, address: &Option<AccountId>) -> bool {
        self.data().role_members.contains_value(role, address)
    }

    fn _add(&mut self, role: RoleType, member: &Option<AccountId>) {
        self.data().role_members.insert(role, member);
    }

    fn _remove(&mut self, role: RoleType, member: &Option<AccountId>) {
        self.data().role_members.remove_value(role, member);
    }

    fn _get_role_admin(&self, role: RoleType) -> Option<RoleType> {
        self.data().admin_roles.get(role)
    }

    fn _set_role_admin(&mut self, role: RoleType, new_admin: RoleType) {
        self.data().admin_roles.insert(role, &new_admin);
    }
}

pub trait AccessControlEnumerableImpl: Storage<Data> {
    fn get_role_member(&self, role: RoleType, index: u32) -> Option<AccountId> {
        self.data()
            .role_members
            .get_value(role, &(index as u128))
            .unwrap_or(None)
    }

    fn get_role_member_count(&self, role: RoleType) -> u32 {
        self.data().role_members.count(role) as u32
    }
}
