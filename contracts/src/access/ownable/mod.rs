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
    ownable,
    traits::ownable::*,
};
use openbrush::{
    modifier_definition,
    modifiers,
    traits::{
        AccountId,
        DefaultEnv,
        Storage,
        StorageAccess,
    },
    with_data,
};
pub use ownable::Internal as _;

#[cfg(feature = "upgradeable")]
use openbrush::storage::Lazy;

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::storage_item(STORAGE_KEY)]
pub struct Data {
    pub owner: Option<AccountId>,
    pub _reserved: Option<()>,
}

#[cfg(feature = "upgradeable")]
pub type DataType = Lazy<Data>;
#[cfg(not(feature = "upgradeable"))]
pub type DataType = Data;

/// Throws if called by any account other than the owner.
#[modifier_definition]
pub fn only_owner<T, F, R, E>(instance: &mut T, body: F) -> Result<R, E>
where
    T: StorageAccess<Data> + Sized,
    F: FnOnce(&mut T) -> Result<R, E>,
    E: From<OwnableError>,
{
    if instance.data().owner != Some(T::env().caller()) {
        return Err(From::from(OwnableError::CallerIsNotOwner))
    }
    body(instance)
}

pub trait OwnableImpl: StorageAccess<Data> + Sized + Internal {
    fn owner(&self) -> Option<AccountId> {
        self.get_or_default().owner.clone()
    }

    #[modifiers(only_owner)]
    fn renounce_ownership(&mut self) -> Result<(), OwnableError> {
        let old_owner = self.get_or_default().owner.clone();
        with_data!(self, data, {
            data.owner = None;
        });
        self._emit_ownership_transferred_event(old_owner, None);
        Ok(())
    }

    #[modifiers(only_owner)]
    fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<(), OwnableError> {
        if new_owner.is_zero() {
            return Err(OwnableError::NewOwnerIsZero)
        }
        let old_owner = self.get_or_default().owner.clone();
        with_data!(self, data, {
            data.owner = new_owner.clone();
        });
        self._emit_ownership_transferred_event(old_owner, Some(new_owner));
        Ok(())
    }
}

pub trait Internal {
    /// User must override this method in their contract.
    fn _emit_ownership_transferred_event(&self, _previous: Option<AccountId>, _new: Option<AccountId>);

    fn _init_with_owner(&mut self, owner: AccountId);
}

pub trait InternalImpl: StorageAccess<Data> + Internal + Sized {
    fn _emit_ownership_transferred_event(&self, _previous: Option<AccountId>, _new: Option<AccountId>) {}

    fn _init_with_owner(&mut self, owner: AccountId) {
        with_data!(self, data, {
            data.owner = Some(owner);
        });
        Internal::_emit_ownership_transferred_event(self, None, Some(owner));
    }
}
