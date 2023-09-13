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
    nonces::*,
    psp22,
    psp22::extensions::permit,
    traits::psp22::{extensions::permit::*, *},
};
use openbrush::{
    traits::{AccountId, Balance, Storage},
    utils::crypto::hash_blake2b256,
};

pub use openbrush::utils::crypto::Signature;

pub use psp22::{Internal as _, InternalImpl as _, PSP22Impl};
use scale::Encode;

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    #[lazy]
    pub cached_domain_separator: [u8; 32],
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, scale::Encode, scale::Decode)]
pub struct PermitMessage {
    pub domain_separator: [u8; 32],
    pub owner: AccountId,
    pub spender: AccountId,
    pub amount: Balance,
    pub deadline: u64,
    pub nonce: u64,
}

pub trait PSP22PermitImpl: Internal {
    fn permit(
        &mut self,
        owner: AccountId,
        spender: AccountId,
        amount: Balance,
        deadline: u64,
        signature: Signature,
    ) -> Result<(), PSP22Error> {
        self._permit(owner, spender, amount, deadline, signature)
    }

    fn domain_separator(&mut self) -> [u8; 32] {
        self._domain_separator()
    }
}

pub trait Internal {
    fn _permit(
        &mut self,
        owner: AccountId,
        spender: AccountId,
        amount: Balance,
        deadline: u64,
        signature: Signature,
    ) -> Result<(), PSP22Error>;

    fn _domain_separator(&mut self) -> [u8; 32];
}

pub trait InternalImpl: Storage<Data> + psp22::Internal + NoncesImpl {
    fn _permit(
        &mut self,
        owner: AccountId,
        spender: AccountId,
        amount: Balance,
        deadline: u64,
        signature: Signature,
    ) -> Result<(), PSP22Error> {
        let block_time = Self::env().block_timestamp();
        if deadline < block_time {
            return Err(PSP22Error::PermitExpired);
        }

        let nonce = self._use_nonce(&owner)?;
        let domain_separator = self._domain_separator();

        let message = &scale::Encode::encode(&PermitMessage {
            domain_separator,
            owner,
            spender,
            amount,
            deadline,
            nonce,
        });

        if signature.verify(message, &owner) {
            self._approve_from_to(owner, spender, amount)?;
            Ok(())
        } else {
            Err(PSP22Error::PermitInvalidSignature)
        }
    }

    fn _domain_separator(&mut self) -> [u8; 32] {
        let cached = self.data::<Data>().cached_domain_separator.get_or_default();

        if self.data::<Data>().cached_domain_separator.get().is_none() {
            let account_hash = hash_blake2b256(&Self::env().account_id().encode());

            self.data::<Data>().cached_domain_separator.set(&account_hash);

            account_hash
        } else {
            cached
        }
    }
}
