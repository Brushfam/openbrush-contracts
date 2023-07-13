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
    psp22,
    psp22::extensions::permit,
    traits::psp22::{extensions::permit::*, *},
};
use openbrush::storage::Mapping;
use openbrush::traits::{AccountId, Balance};
pub use psp22::{Internal as _, InternalImpl as _, PSP22Impl};

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    pub nonces: Mapping<AccountId, u32>,
}

pub trait PSP22PermitImpl: Internal {
    fn permit(
        &mut self,
        owner: AccountId,
        spender: AccountId,
        amount: Balance,
        deadline: u32,
        v: u8,
        r: [u8; 32],
        s: [u8; 32],
    ) -> Result<(), PSP22Error> {
        self._permit(owner, spender, amount, deadline, v, r, s)
    }

    fn nonces(&self, owner: AccountId) -> u32 {
        self._nonces(owner)
    }

    fn domain_separator(&self) -> [u8; 32] {
        self._domain_separator()
    }
}

pub trait Internal {
    fn _permit(
        &mut self,
        owner: AccountId,
        spender: AccountId,
        amount: Balance,
        deadline: u32,
        v: u8,
        r: [u8; 32],
        s: [u8; 32],
    ) -> Result<(), PSP22Error>;

    fn _nonces(&self, owner: AccountId) -> u32;

    fn _domain_separator(&self) -> [u8; 32];
}

pub trait InternalImpl {
    fn _permit(
        &mut self,
        owner: AccountId,
        spender: AccountId,
        amount: Balance,
        deadline: u32,
        v: u8,
        r: [u8; 32],
        s: [u8; 32],
    ) -> Result<(), PSP22Error> {
        Ok(())
    }

    fn _nonces(&self, owner: AccountId) -> u32 {
        0
    }

    fn _domain_separator(&self) -> [u8; 32] {
        [0; 32]
    }
}
