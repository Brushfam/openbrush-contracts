// Copyright (c) 2023 Brushfam
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

pub use crate::traits::errors::NoncesError;
pub use crate::{
    nonces,
    traits::utils::nonces::*,
};
use openbrush::{
    storage::Mapping,
    traits::{
        AccountId,
        Storage,
    },
};

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    pub nonces: Mapping<AccountId, u128>,
}

/// Provides tracking nonces for addresses. Nonces will only increment.
pub trait NoncesImpl: Storage<Data> {
    /// Returns the nonce of `account`.
    fn nonces(&self, account: &AccountId) -> u128 {
        self.data().nonces.get(account).unwrap_or_default()
    }

    /// Returns the next nonce of `account`, and increments the nonce.
    fn _use_nonce(&mut self, account: &AccountId) -> Result<u128, NoncesError> {
        let nonce = self.nonces(account);
        self.data()
            .nonces
            .insert(account, &(nonce.checked_add(1).ok_or(NoncesError::NonceOverflow)?));
        Ok(nonce)
    }

    /// Returns the next nonce of `account`, and increments the nonce if `nonce` matches the current nonce.
    fn _use_checked_nonce(&mut self, account: &AccountId, nonce: u128) -> Result<u128, NoncesError> {
        let current_nonce = self.nonces(&account);
        if nonce != current_nonce {
            return Err(NoncesError::InvalidAccountNonce(account.clone(), current_nonce))
        }
        self.data()
            .nonces
            .insert(account, &(nonce.checked_add(1).ok_or(NoncesError::NonceOverflow)?));
        Ok(nonce)
    }
}
