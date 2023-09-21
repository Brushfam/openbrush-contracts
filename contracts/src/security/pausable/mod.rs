// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::{
    pausable,
    traits::pausable::*,
};
use openbrush::{
    modifier_definition,
    modifiers,
    traits::{
        AccountId,
        Storage,
    },
};
pub use pausable::{
    Internal as _,
    InternalImpl as _,
    PausableImpl as _,
};

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    #[lazy]
    pub paused: bool,
}

/// Modifier to make a function callable only when the contract is paused.
#[modifier_definition]
pub fn when_paused<T, F, R, E>(instance: &mut T, body: F) -> Result<R, E>
where
    T: Storage<Data>,
    F: FnOnce(&mut T) -> Result<R, E>,
    E: From<PausableError>,
{
    if !instance.data().paused.get_or_default() {
        return Err(From::from(PausableError::NotPaused))
    }
    body(instance)
}

/// Modifier to make a function callable only when the contract is not paused.
#[modifier_definition]
pub fn when_not_paused<T, F, R, E>(instance: &mut T, body: F) -> Result<R, E>
where
    T: Storage<Data>,
    F: FnOnce(&mut T) -> Result<R, E>,
    E: From<PausableError>,
{
    if instance.data().paused.get_or_default() {
        return Err(From::from(PausableError::Paused))
    }
    body(instance)
}

pub trait PausableImpl: Storage<Data> + Internal {
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

pub trait InternalImpl: Storage<Data> + Internal {
    fn _emit_paused_event(&self, _account: AccountId) {}

    fn _emit_unpaused_event(&self, _account: AccountId) {}

    fn _paused(&self) -> bool {
        self.data().paused.get_or_default()
    }

    #[modifiers(when_not_paused)]
    fn _pause(&mut self) -> Result<(), PausableError> {
        self.data().paused.set(&true);
        Internal::_emit_paused_event(self, Self::env().caller());
        Ok(())
    }

    #[modifiers(when_paused)]
    fn _unpause(&mut self) -> Result<(), PausableError> {
        self.data().paused.set(&false);
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
