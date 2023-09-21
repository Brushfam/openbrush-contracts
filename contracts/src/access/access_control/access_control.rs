// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::{
    access_control,
    traits::access_control::*,
};
pub use access_control::Internal as _;
use openbrush::{
    modifier_definition,
    modifiers,
    storage::{
        Mapping,
        TypeGuard,
        ValueGuard,
    },
    traits::{
        AccountId,
        DefaultEnv,
        Storage,
    },
};

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    pub admin_roles: Mapping<RoleType, RoleType, ValueGuard<RoleType>>,
    pub members: Mapping<(RoleType, Option<AccountId>), (), MembersKey>,
}

pub struct MembersKey;

impl<'a> TypeGuard<'a> for MembersKey {
    type Type = &'a (RoleType, &'a Option<AccountId>);
}

pub const DEFAULT_ADMIN_ROLE: RoleType = 0;

/// Modifier that checks that `caller` has a specific role.
#[modifier_definition]
pub fn only_role<T, F, R, E>(instance: &mut T, body: F, role: RoleType) -> Result<R, E>
where
    T: Internal,
    F: FnOnce(&mut T) -> Result<R, E>,
    E: From<AccessControlError>,
{
    if let Err(err) = instance._check_role(role, Some(T::env().caller())) {
        return Err(From::from(err))
    }
    body(instance)
}

pub trait AccessControlImpl: Internal + MembersManager + Sized {
    fn has_role(&self, role: RoleType, address: Option<AccountId>) -> bool {
        self._has_role(role, &address)
    }

    fn get_role_admin(&self, role: RoleType) -> RoleType {
        Internal::_get_role_admin(self, role)
    }

    #[modifiers(only_role(self.get_role_admin(role)))]
    fn grant_role(&mut self, role: RoleType, account: Option<AccountId>) -> Result<(), AccessControlError> {
        if self._has_role(role, &account) {
            return Err(AccessControlError::RoleRedundant)
        }
        self._add(role, &account);
        self._emit_role_granted(role, account, Some(Self::env().caller()));
        Ok(())
    }

    #[modifiers(only_role(self.get_role_admin(role)))]
    fn revoke_role(&mut self, role: RoleType, account: Option<AccountId>) -> Result<(), AccessControlError> {
        self._check_role(role, account)?;
        self._do_revoke_role(role, account);
        Ok(())
    }

    fn renounce_role(&mut self, role: RoleType, account: Option<AccountId>) -> Result<(), AccessControlError> {
        if account != Some(Self::env().caller()) {
            return Err(AccessControlError::InvalidCaller)
        }
        self._check_role(role, account)?;
        self._do_revoke_role(role, account);
        Ok(())
    }
}

pub trait MembersManager {
    fn _has_role(&self, role: RoleType, address: &Option<AccountId>) -> bool;

    fn _add(&mut self, role: RoleType, member: &Option<AccountId>);

    fn _remove(&mut self, role: RoleType, member: &Option<AccountId>);

    fn _get_role_admin(&self, role: RoleType) -> Option<RoleType>;

    fn _set_role_admin(&mut self, role: RoleType, new_admin: RoleType);
}

pub trait MembersManagerImpl: Storage<Data> {
    fn _has_role(&self, role: RoleType, address: &Option<AccountId>) -> bool {
        self.data().members.contains(&(role, address))
    }

    fn _add(&mut self, role: RoleType, member: &Option<AccountId>) {
        self.data().members.insert(&(role, member), &());
    }

    fn _remove(&mut self, role: RoleType, member: &Option<AccountId>) {
        self.data().members.remove(&(role, member));
    }

    fn _get_role_admin(&self, role: RoleType) -> Option<RoleType> {
        self.data().admin_roles.get(role)
    }

    fn _set_role_admin(&mut self, role: RoleType, new_admin: RoleType) {
        self.data().admin_roles.insert(role, &new_admin);
    }
}

pub trait Internal {
    /// The user must override those methods using their event definition.
    fn _emit_role_admin_changed(&mut self, role: RoleType, previous: RoleType, new: RoleType);

    fn _emit_role_granted(&mut self, role: RoleType, grantee: Option<AccountId>, grantor: Option<AccountId>);

    fn _emit_role_revoked(&mut self, role: RoleType, account: Option<AccountId>, sender: AccountId);

    fn _default_admin() -> RoleType;

    fn _init_with_caller(&mut self);

    fn _init_with_admin(&mut self, admin: Option<AccountId>);

    fn _setup_role(&mut self, role: RoleType, member: Option<AccountId>);

    fn _do_revoke_role(&mut self, role: RoleType, account: Option<AccountId>);

    fn _set_role_admin(&mut self, role: RoleType, new_admin: RoleType);

    fn _check_role(&self, role: RoleType, account: Option<AccountId>) -> Result<(), AccessControlError>;

    fn _get_role_admin(&self, role: RoleType) -> RoleType;
}

pub trait InternalImpl: Internal + MembersManager + Sized {
    fn _emit_role_admin_changed(&mut self, _role: RoleType, _previous: RoleType, _new: RoleType) {}

    fn _emit_role_granted(&mut self, _role: RoleType, _grantee: Option<AccountId>, _grantor: Option<AccountId>) {}

    fn _emit_role_revoked(&mut self, _role: RoleType, _account: Option<AccountId>, _sender: AccountId) {}

    fn _default_admin() -> RoleType {
        DEFAULT_ADMIN_ROLE
    }

    fn _init_with_caller(&mut self) {
        Internal::_init_with_admin(self, Some(Self::env().caller()));
    }

    fn _init_with_admin(&mut self, admin: Option<AccountId>) {
        Internal::_setup_role(self, <Self as Internal>::_default_admin(), admin);
    }

    fn _setup_role(&mut self, role: RoleType, member: Option<AccountId>) {
        if !self._has_role(role, &member) {
            self._add(role, &member);

            Internal::_emit_role_granted(self, role, member, None);
        }
    }

    fn _do_revoke_role(&mut self, role: RoleType, account: Option<AccountId>) {
        self._remove(role, &account);
        Internal::_emit_role_revoked(self, role, account, Self::env().caller());
    }

    fn _set_role_admin(&mut self, role: RoleType, new_admin: RoleType) {
        let old_admin = Internal::_get_role_admin(self, role);
        MembersManager::_set_role_admin(self, role, new_admin);
        Internal::_emit_role_admin_changed(self, role, old_admin, new_admin);
    }

    fn _check_role(&self, role: RoleType, account: Option<AccountId>) -> Result<(), AccessControlError> {
        if !self._has_role(role, &account) {
            return Err(AccessControlError::MissingRole)
        }
        Ok(())
    }

    fn _get_role_admin(&self, role: RoleType) -> RoleType {
        MembersManager::_get_role_admin(self, role).unwrap_or(<Self as Internal>::_default_admin())
    }
}
