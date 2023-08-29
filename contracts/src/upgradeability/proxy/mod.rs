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

// Delegate calls were marked as a possible attack vector in ink!
// Therefore the proxy and diamond contracts will be disabled within OpenBrush until this is reimplemented in ink! 4.

pub use crate::{
    ownable, proxy,
    traits::{ownable::*, proxy::*},
};
use openbrush::{
    modifiers,
    traits::{Hash, Storage},
};
pub use ownable::{Internal as _, InternalImpl as _};
pub use proxy::{Internal as _, InternalImpl as _};

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    pub forward_to: Hash,
}

pub trait ProxyImpl: Storage<Data> + Storage<ownable::Data> + Internal {
    fn get_delegate_code(&self) -> Hash {
        self.data::<Data>().forward_to
    }

    #[modifiers(ownable::only_owner)]
    fn change_delegate_code(&mut self, new_code_hash: Hash) -> Result<(), OwnableError> {
        let old_code_hash = self.data::<Data>().forward_to;
        self.data::<Data>().forward_to = new_code_hash;
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
        self.data().forward_to = forward_to;
        Internal::_emit_delegate_code_changed_event(self, None, Some(forward_to));
    }

    fn _fallback(&self) -> ! {
        ink::env::call::build_call::<ink::env::DefaultEnvironment>()
            .delegate(self.data().forward_to)
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
                    self.data().forward_to.clone(),
                    err
                )
            })
            .unwrap_or_else(|err| {
                panic!(
                    "delegate call to {:?} failed due to {:?}",
                    self.data().forward_to.clone(),
                    err
                )
            });
        unreachable!("the _fallback call will never return since `tail_call` was set");
    }
}
