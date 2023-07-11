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
#[cfg(feature = "upgradeable")]
use openbrush::storage::Lazy;
use openbrush::{
    storage::{
        MultiMapping,
        ValueGuard,
    },
    traits::{
        AccountId,
        Storage,
        StorageAccess,
    },
    with_data,
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Members);

#[openbrush::storage_item(STORAGE_KEY)]
#[derive(Default, Debug)]
pub struct Data {
    pub role_members: MultiMapping<RoleType, Option<AccountId>, ValueGuard<RoleType>>,
    pub _reserved: Option<()>,
}

#[cfg(feature = "upgradeable")]
pub type DataType = Lazy<Data>;
#[cfg(not(feature = "upgradeable"))]
pub type DataType = Data;

pub trait MembersManagerImpl: StorageAccess<Data> {
    fn _has_role(&self, role: RoleType, address: &AccountId) -> bool {
        self.get_or_default().role_members.contains_value(role, address)
    }

    fn _add(&mut self, role: RoleType, member: &AccountId) {
        with_data!(self, data, {
            data.role_members.insert(role, member);
        });
    }

    fn _remove(&mut self, role: RoleType, member: &AccountId) {
        with_data!(self, data, {
            data.role_members.remove_value(role, member);
        });
    }
}

pub trait AccessControlEnumerableImpl: StorageAccess<Data> {
    fn get_role_member(&self, role: RoleType, index: u32) -> Option<AccountId> {
        self.get_or_default()
            .role_members
            .get_value(role, &(index as u128))
            .unwrap_or(None)
    }

    fn get_role_member_count(&self, role: RoleType) -> u32 {
        self.get_or_default().role_members.count(role) as u32
    }
}
