// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::{
    nonces,
    traits::nonces::*,
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
    pub nonces: Mapping<AccountId, u64>,
}

/// Provides tracking nonces for addresses. Nonces will only increment.
pub trait NoncesImpl: Storage<Data> {
    /// Returns the nonce of `account`.
    fn nonces(&self, account: &AccountId) -> u64 {
        self.data().nonces.get(account).unwrap_or_default()
    }

    /// Returns the next nonce of `account`, and increments the nonce.
    fn _use_nonce(&mut self, account: &AccountId) -> Result<u64, NoncesError> {
        let nonce = self.nonces(account);
        self.data()
            .nonces
            .insert(account, &(nonce.checked_add(1).ok_or(NoncesError::NonceOverflow)?));
        Ok(nonce)
    }

    /// Returns the next nonce of `account`, and increments the nonce if `nonce` matches the current nonce.
    fn _use_checked_nonce(&mut self, account: &AccountId, nonce: u64) -> Result<u64, NoncesError> {
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
