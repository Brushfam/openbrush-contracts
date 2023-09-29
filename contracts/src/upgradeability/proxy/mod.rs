// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

// Delegate calls were marked as a possible attack vector in ink!
// Therefore the proxy and diamond contracts will be disabled within OpenBrush until this is reimplemented in ink! 4.

pub use crate::{
    ownable,
    proxy,
    traits::{
        ownable::*,
        proxy::*,
    },
};
use openbrush::{
    modifiers,
    traits::{
        Hash,
        Storage,
    },
};
pub use ownable::{
    Internal as _,
    InternalImpl as _,
};
pub use proxy::{
    Internal as _,
    InternalImpl as _,
};

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    #[lazy]
    pub forward_to: Hash,
}

pub trait ProxyImpl: Storage<Data> + Storage<ownable::Data> + Internal {
    fn get_delegate_code(&self) -> Hash {
        self.data::<Data>().forward_to.get_or_default()
    }

    #[modifiers(ownable::only_owner)]
    fn change_delegate_code(&mut self, new_code_hash: Hash) -> Result<(), OwnableError> {
        let old_code_hash = self.data::<Data>().forward_to.get_or_default();
        self.data::<Data>().forward_to.set(&new_code_hash);
        self._emit_delegate_code_changed_event(Some(old_code_hash), Some(new_code_hash));
        Ok(())
    }
}

pub trait Internal {
    fn _emit_delegate_code_changed_event(&self, _previous: Option<Hash>, _new: Option<Hash>);

    fn _init_with_forward_to(&mut self, forward_to: Hash);

    fn _fallback(&self) -> !;
}

pub trait InternalImpl: Internal + Storage<Data> {
    fn _emit_delegate_code_changed_event(&self, _previous: Option<Hash>, _new: Option<Hash>) {}

    fn _init_with_forward_to(&mut self, forward_to: Hash) {
        self.data().forward_to.set(&forward_to);
        Internal::_emit_delegate_code_changed_event(self, None, Some(forward_to));
    }

    fn _fallback(&self) -> ! {
        ink::env::call::build_call::<ink::env::DefaultEnvironment>()
            .delegate(self.data().forward_to.get_or_default())
            .call_flags(
                ink::env::CallFlags::default()
                // We don't plan to use the input data after the delegated call, so the 
                // input data can be forwarded to delegated contract to reduce the gas usage.
                .set_forward_input(true)
                // We don't plan to return back to that contract after execution, so we 
                // marked delegated call as "tail", to end the execution of the contract.
                .set_tail_call(true),
            )
            .try_invoke()
            .unwrap_or_else(|err| {
                panic!(
                    "delegate call to {:?} failed due to {:?}",
                    self.data().forward_to.get_or_default().clone(),
                    err
                )
            })
            .unwrap_or_else(|err| {
                panic!(
                    "delegate call to {:?} failed due to {:?}",
                    self.data().forward_to.get_or_default().clone(),
                    err
                )
            });
        unreachable!("the _fallback call will never return since `tail_call` was set");
    }
}
