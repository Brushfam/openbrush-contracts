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
    pausable,
    traits::pausable::*,
};
use openbrush::{
    modifier_definition,
    modifiers,
    traits::{
        AccountId,
        DefaultEnv,
        StorageAccess,
    },
    with_data,
};
pub use pausable::{
    Internal as _,
    InternalImpl as _,
    PausableImpl as _,
};

#[cfg(feature = "upgradeable")]
use openbrush::storage::Lazy;

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::storage_item(STORAGE_KEY)]
pub struct Data {
    pub paused: bool,
    pub _reserved: Option<()>,
}

#[cfg(feature = "upgradeable")]
pub type DataType = Lazy<Data>;
#[cfg(not(feature = "upgradeable"))]
pub type DataType = Data;

/// Modifier to make a function callable only when the contract is paused.
#[modifier_definition]
pub fn when_paused<T, F, R, E>(instance: &mut T, body: F) -> Result<R, E>
where
    T: StorageAccess<Data> + Sized,
    F: FnOnce(&mut T) -> Result<R, E>,
    E: From<PausableError>,
{
    if !instance.get_or_default().paused {
        return Err(From::from(PausableError::NotPaused))
    }
    body(instance)
}

/// Modifier to make a function callable only when the contract is not paused.
#[modifier_definition]
pub fn when_not_paused<T, F, R, E>(instance: &mut T, body: F) -> Result<R, E>
where
    T: StorageAccess<Data> + Sized,
    F: FnOnce(&mut T) -> Result<R, E>,
    E: From<PausableError>,
{
    if instance.get_or_default().paused {
        return Err(From::from(PausableError::Paused))
    }
    body(instance)
}

pub trait PausableImpl: StorageAccess<Data> + Internal + Sized {
    fn paused(&self) -> bool {
        self._paused()
    }
}

pub trait Internal {
    /// User must override those methods in their contract.
    fn _emit_paused_event(&self, _account: AccountId);

    fn _emit_unpaused_event(&self, _account: AccountId);

    fn _paused(&self) -> bool;

    /// Triggers stopped state.
    ///
    /// On success a `Paused` event is emitted.
    fn _pause(&mut self) -> Result<(), PausableError>;

    /// Returns to normal state.
    ///
    /// On success a `Unpaused` event is emitted.
    fn _unpause(&mut self) -> Result<(), PausableError>;

    /// Function which changes state to unpaused if paused and vice versa
    fn _switch_pause(&mut self) -> Result<(), PausableError>;
}

pub trait InternalImpl: StorageAccess<Data> + Internal + Sized {
    fn _emit_paused_event(&self, _account: AccountId) {}

    fn _emit_unpaused_event(&self, _account: AccountId) {}

    fn _paused(&self) -> bool {
        self.get_or_default().paused
    }

    #[modifiers(when_not_paused)]
    fn _pause(&mut self) -> Result<(), PausableError> {
        with_data!(self, data, {
            data.paused = true;
        });
        Internal::_emit_paused_event(self, Self::env().caller());
        Ok(())
    }

    #[modifiers(when_paused)]
    fn _unpause(&mut self) -> Result<(), PausableError> {
        with_data!(self, data, {
            data.paused = false;
        });
        Internal::_emit_unpaused_event(self, Self::env().caller());
        Ok(())
    }

    fn _switch_pause(&mut self) -> Result<(), PausableError> {
        if Internal::_paused(self) {
            Internal::_unpause(self)
        } else {
            Internal::_pause(self)
        }
    }
}
