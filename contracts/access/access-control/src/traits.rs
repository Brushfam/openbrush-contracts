pub use access_control_derive::AccessControlStorage;
use brush::{
    declare_storage_trait,
    modifier_definition,
    modifiers,
    traits::{
        AccountId,
        InkStorage,
    },
};
pub use common::errors::AccessControlError;
use ink_storage::{
    collections::HashMap as StorageHashMap,
    traits::SpreadLayout,
};

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct AccessControlData {
    pub admin_roles: StorageHashMap<RoleType, RoleType>,
    pub members: StorageHashMap<(RoleType, AccountId), ()>,
}

declare_storage_trait!(AccessControlStorage, AccessControlData);

pub type RoleType = u32;

/// Modifier that checks that `caller` has a specific role.
#[modifier_definition]
pub fn only_role<T, F, R, E>(instance: &mut T, body: F, role: RoleType) -> Result<R, E>
where
    T: AccessControl,
    F: FnOnce(&mut T) -> Result<R, E>,
    E: From<AccessControlError>,
{
    if let Err(err) = instance._check_role(&role, &T::env().caller()) {
        return Err(From::from(err))
    }
    body(instance)
}

#[brush::wrapper]
pub type AccessControlWrapper = dyn AccessControl;

/// Contract module that allows children to implement role-based access
/// control mechanisms. This is a lightweight version that doesn't allow enumerating role
/// members except through off-chain means by accessing the contract event logs.
///
/// Roles can be granted and revoked dynamically via the `grant_role` and
/// `revoke_role`. functions. Each role has an associated admin role, and only
/// accounts that have a role's admin role can call `grant_role` and `revoke_role`.
///
/// This module is used through embedding of `AccessControlData` and implementation of `AccessControl` and
/// `AccessControlStorage` traits.
#[brush::trait_definition]
pub trait AccessControl: AccessControlStorage {
    const DEFAULT_ADMIN_ROLE: RoleType = 0;

    /// Returns `true` if `account` has been granted `role`.
    #[ink(message)]
    fn has_role(&self, role: RoleType, address: AccountId) -> bool {
        self._has_role(&role, &address)
    }

    /// Returns the admin role that controls `role`. See `grant_role` and `revoke_role`.
    #[ink(message)]
    fn get_role_admin(&self, role: RoleType) -> RoleType {
        self._get_role_admin(&role)
    }

    /// Grants `role` to `account`.
    ///
    /// On success a `RoleGranted` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns with `MissingRole` error if caller can't grant the role.
    /// Returns with `RoleRedundant` error `account` has `role`.
    #[ink(message)]
    #[modifiers(only_role(self._get_role_admin(&role)))]
    fn grant_role(&mut self, role: RoleType, account: AccountId) -> Result<(), AccessControlError> {
        if self._has_role(&role, &account) {
            return Err(AccessControlError::RoleRedundant)
        }
        self.get_mut().members.insert((role.clone(), account.clone()), ());
        self._emit_role_granted(role, account, Some(Self::env().caller()));
        Ok(())
    }

    /// Revokes `role` from `account`.
    ///
    /// On success a `RoleRevoked` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns with `MissingRole` error if caller can't grant the `role` or if `account` doesn't have `role`.
    #[ink(message)]
    #[modifiers(only_role(self._get_role_admin(&role)))]
    fn revoke_role(&mut self, role: RoleType, account: AccountId) -> Result<(), AccessControlError> {
        self._check_role(&role, &account)?;
        self._do_revoke_role(role, account);
        Ok(())
    }

    /// Revokes `role` from the calling account.
    /// Roles are often managed via `grant_role` and `revoke_role`: this function's
    /// purpose is to provide a mechanism for accounts to lose their privileges
    /// if they are compromised (such as when a trusted device is misplaced).
    ///
    /// On success a `RoleRevoked` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns with `InvalidCaller` error if caller is not `account`.
    /// Returns with `MissingRole` error if `account` doesn't have `role`.
    #[ink(message)]
    fn renounce_role(&mut self, role: RoleType, account: AccountId) -> Result<(), AccessControlError> {
        if Self::env().caller() != account {
            return Err(AccessControlError::InvalidCaller)
        }
        self._check_role(&role, &account)?;
        self._do_revoke_role(role, account);
        Ok(())
    }

    // Helper functions

    /// The user must override this function using their event definition.
    fn _emit_role_admin_changed(&mut self, _role: RoleType, _previous_admin_role: RoleType, _new_admin_role: RoleType) {
    }

    /// The user must override this function using their event definition.
    fn _emit_role_granted(&mut self, _role: RoleType, _grantee: AccountId, _grantor: Option<AccountId>) {}

    /// The user must override this function using their event definition.
    fn _emit_role_revoked(&mut self, _role: RoleType, _account: AccountId, _sender: AccountId) {}

    fn _init_with_caller(&mut self) {
        let caller = Self::env().caller();
        self._init_with_admin(caller);
    }

    fn _init_with_admin(&mut self, admin: AccountId) {
        self._setup_role(Self::DEFAULT_ADMIN_ROLE, admin);
    }

    fn _setup_role(&mut self, role: RoleType, admin: AccountId) {
        if !self._has_role(&role, &admin) {
            self.get_mut().members.insert((role.clone(), admin.clone()), ());

            self._emit_role_granted(role, admin, None);
        }
    }

    fn _has_role(&self, role: &RoleType, address: &AccountId) -> bool {
        self.get().members.contains_key(&(role.clone(), address.clone()))
    }

    fn _get_role_admin(&self, role: &RoleType) -> RoleType {
        self.get()
            .admin_roles
            .get(role)
            .cloned()
            .unwrap_or(Self::DEFAULT_ADMIN_ROLE)
    }

    fn _do_revoke_role(&mut self, role: RoleType, account: AccountId) {
        self.get_mut().members.take(&(role, account));
        self._emit_role_revoked(role, account, Self::env().caller());
    }

    fn _check_role(&self, role: &RoleType, account: &AccountId) -> Result<(), AccessControlError> {
        if !self._has_role(role, account) {
            return Err(AccessControlError::MissingRole)
        }
        Ok(())
    }

    fn _set_role_admin(&mut self, role: RoleType, new_admin: RoleType) {
        let entry = self
            .get_mut()
            .admin_roles
            .entry(role)
            .or_insert(Self::DEFAULT_ADMIN_ROLE);
        let old_admin = entry.clone();
        *entry = new_admin;
        self._emit_role_admin_changed(role, old_admin, new_admin);
    }
}
