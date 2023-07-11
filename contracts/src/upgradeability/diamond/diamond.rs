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
    diamond,
    ownable,
    traits::{
        diamond::*,
        ownable::*,
    },
};
pub use diamond::{
    Internal as _,
    InternalImpl as _,
};
use ink::{
    env::call::{
        ExecutionInput,
        Selector as InkSelector,
    },
    prelude::vec::Vec,
    primitives::Clear,
};
use openbrush::{
    modifiers,
    storage::Mapping,
    traits::{
        DefaultEnv,
        Flush,
        Hash,
        StorageAccess,
    },
    with_data,
};
pub use ownable::Internal as _;

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

// TODO: Add support of Erc165
#[derive(Default, Debug)]
#[openbrush::storage_item(STORAGE_KEY)]
pub struct Data {
    pub selector_to_hash: Mapping<Selector, Hash>,
    // Facet mapped to all functions it supports
    pub hash_to_selectors: Mapping<Hash, Vec<Selector>>,
}

#[cfg(feature = "upgradeable")]
pub type DataType = Lazy<Data, ResolverKey<AutoKey>>;
#[cfg(not(feature = "upgradeable"))]
pub type DataType = Data;

pub trait DiamondImpl: Internal + Sized + StorageAccess<ownable::Data> {
    #[modifiers(ownable::only_owner)]
    fn diamond_cut(&mut self, diamond_cut: Vec<FacetCut>, init: Option<InitCall>) -> Result<(), DiamondError> {
        self._diamond_cut(diamond_cut, init)
    }
}

pub trait Internal {
    fn _emit_diamond_cut_event(&self, diamond_cut: &Vec<FacetCut>, init: &Option<InitCall>);

    fn _diamond_cut(&mut self, diamond_cut: Vec<FacetCut>, init: Option<InitCall>) -> Result<(), DiamondError>;

    fn _diamond_cut_facet(&mut self, facet_cut: &FacetCut) -> Result<(), DiamondError>;

    fn _fallback(&self) -> !;

    fn _init_call(&self, call: InitCall) -> !;

    fn _remove_facet(&mut self, code_hash: Hash);

    fn _remove_selectors(&mut self, facet_cut: &FacetCut);
}

pub trait InternalImpl: Internal + StorageAccess<Data> + DiamondCut + Sized + Flush {
    fn _emit_diamond_cut_event(&self, _diamond_cut: &Vec<FacetCut>, _init: &Option<InitCall>) {}

    fn _diamond_cut(&mut self, diamond_cut: Vec<FacetCut>, init: Option<InitCall>) -> Result<(), DiamondError> {
        for facet_cut in diamond_cut.iter() {
            Internal::_diamond_cut_facet(self, facet_cut)?;
        }

        Internal::_emit_diamond_cut_event(self, &diamond_cut, &init);

        if init.is_some() {
            self.flush();
            Internal::_init_call(self, init.unwrap());
        }

        Ok(())
    }

    fn _diamond_cut_facet(&mut self, facet_cut: &FacetCut) -> Result<(), DiamondError> {
        let code_hash = facet_cut.hash;
        if code_hash.is_clear() {
            return Err(DiamondError::EmptyCodeHash)
        }
        if facet_cut.selectors.is_empty() {
            // means that we want to remove this facet
            Internal::_remove_facet(self, code_hash);
        } else {
            for selector in facet_cut.selectors.iter() {
                let selector_hash = self.get_or_default().selector_to_hash.get(&selector);

                if selector_hash.and_then(|hash| Some(hash == code_hash)).unwrap_or(false) {
                    // selector already registered to this hash -> no action
                    continue
                } else if selector_hash.is_some() {
                    // selector already registered to another hash -> error
                    return Err(DiamondError::ReplaceExisting(selector_hash.unwrap()))
                } else {
                    // map selector to its facet
                    with_data!(self, data, {
                        data.selector_to_hash.insert(&selector, &code_hash);
                    });
                }
            }

            if self.get_or_default().hash_to_selectors.get(&code_hash).is_none() {
                self._on_add_facet(code_hash);
            }
            // remove selectors from this facet which may be registered but will not be used anymore
            Internal::_remove_selectors(self, facet_cut);
            // map this code hash to its selectors
            with_data!(self, data, {
                data.hash_to_selectors.insert(&code_hash, &facet_cut.selectors);
            });
        }
        Ok(())
    }

    fn _fallback(&self) -> ! {
        let selector = ink::env::decode_input::<Selector>().unwrap_or_else(|_| panic!("Calldata error"));

        let delegate_code = self.get_or_default().selector_to_hash.get(&selector);

        if delegate_code.is_none() {
            panic!("Function is not registered");
        }

        let _ = ink::env::call::build_call::<ink::env::DefaultEnvironment>()
            .delegate(delegate_code.unwrap())
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
            .expect("Delegate call failed")
            .expect("Delegate call failed");
        unreachable!("the _fallback call will never return since `tail_call` was set");
    }

    fn _init_call(&self, call: InitCall) -> ! {
        let _ = ink::env::call::build_call::<ink::env::DefaultEnvironment>()
            .delegate(call.hash)
            .exec_input(ExecutionInput::new(InkSelector::new(call.selector)).push_arg(call.input))
            .call_flags(ink::env::CallFlags::default()
            // We don't plan to return back to that contract after execution, so we
            // marked delegated call as "tail", to end the execution of the contract.
            .set_tail_call(true))
            .returns::<()>()
            .try_invoke()
            .expect("Delegate call failed")
            .expect("Delegate call failed");
        unreachable!("the _init_call call will never return since `tail_call` was set");
    }

    fn _remove_facet(&mut self, code_hash: Hash) {
        let vec = self.get_or_default().hash_to_selectors.get(&code_hash).unwrap();
        vec.iter().for_each(|old_selector| {
            with_data!(self, data, {
                data.selector_to_hash.remove(&old_selector);
            });
        });
        with_data!(self, data, {
            data.hash_to_selectors.remove(&code_hash);
        });
        self._on_remove_facet(code_hash);
    }

    fn _remove_selectors(&mut self, facet_cut: &FacetCut) {
        let selectors = self
            .get_or_default()
            .hash_to_selectors
            .get(&facet_cut.hash)
            .unwrap_or(Vec::<Selector>::new());
        for selector in selectors.iter() {
            if !facet_cut.selectors.contains(&selector) {
                with_data!(self, data, {
                    data.selector_to_hash.remove(&selector);
                });
            }
        }
    }
}

pub trait DiamondCut {
    fn _on_add_facet(&mut self, code_hash: Hash);

    fn _on_remove_facet(&mut self, code_hash: Hash);
}
