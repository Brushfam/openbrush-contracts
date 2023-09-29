// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::{
    ownable,
    traits::ownable::*,
};
use openbrush::{
    modifier_definition,
    modifiers,
    traits::{
        AccountId,
        Storage,
    },
};
pub use ownable::Internal as _;

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    #[lazy]
    pub owner: Option<AccountId>,
}

/// Throws if called by any account other than the owner.
#[modifier_definition]
pub fn only_owner<T, F, R, E>(instance: &mut T, body: F) -> Result<R, E>
where
    T: Storage<Data>,
    F: FnOnce(&mut T) -> Result<R, E>,
    E: From<OwnableError>,
{
    if instance.data().owner.get_or_default() != Some(T::env().caller()) {
        return Err(From::from(OwnableError::CallerIsNotOwner))
    }
    body(instance)
}

pub trait OwnableImpl: Storage<Data> + Internal {
    fn owner(&self) -> Option<AccountId> {
        self.data().owner.get_or_default()
    }

    #[modifiers(only_owner)]
    fn renounce_ownership(&mut self) -> Result<(), OwnableError> {
        self._transfer_ownership(None)?;
        Ok(())
    }

    #[modifiers(only_owner)]
    fn transfer_ownership(&mut self, new_owner: Option<AccountId>) -> Result<(), OwnableError> {
        if new_owner == None {
            return Err(OwnableError::NewOwnerIsNotSet)
        }
        self._transfer_ownership(new_owner)?;
        Ok(())
    }
}

pub trait Internal {
    /// User must override this method in their contract.
    fn _emit_ownership_transferred_event(&self, _previous: Option<AccountId>, _new: Option<AccountId>);

    fn _init_with_owner(&mut self, owner: AccountId);

    fn _transfer_ownership(&mut self, new_owner: Option<AccountId>) -> Result<(), OwnableError>;
}

pub trait InternalImpl: Storage<Data> + Internal {
    fn _emit_ownership_transferred_event(&self, _previous: Option<AccountId>, _new: Option<AccountId>) {}

    fn _init_with_owner(&mut self, owner: AccountId) {
        self.data().owner.set(&Some(owner));
        Internal::_emit_ownership_transferred_event(self, None, Some(owner));
    }

    fn _transfer_ownership(&mut self, new_owner: Option<AccountId>) -> Result<(), OwnableError> {
        let old_owner = self.data().owner.get_or_default();
        self.data().owner.set(&new_owner);
        Internal::_emit_ownership_transferred_event(self, old_owner, new_owner);
        Ok(())
    }
}
