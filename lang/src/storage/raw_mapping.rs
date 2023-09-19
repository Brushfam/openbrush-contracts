// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

use core::marker::PhantomData;
use ink::{
    primitives::Key,
    storage::traits::Packed,
};

#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct RawMapping<K, V, T = Key> {
    prefix: T,
    _marker: PhantomData<fn() -> (K, V)>,
}

/// It is the implementation of `Mapping` functionality without storing it as a storage field.
/// It can be used to store value under the key manually.
impl<K, V, T> RawMapping<K, V, T> {
    /// Creates a new empty `RawMapping`.
    #[inline(always)]
    pub fn new(prefix: T) -> Self {
        Self {
            prefix,
            _marker: Default::default(),
        }
    }
}

impl<K, V, T> RawMapping<K, V, T>
where
    K: scale::Encode,
    V: Packed,
    T: scale::Encode + Copy,
{
    /// Insert the given `value` to the contract storage.
    #[inline(always)]
    pub fn insert(&mut self, key: K, value: &V) {
        ink::env::set_contract_storage(&self.storage_key(key), value);
    }

    /// Get the `value` at `key` from the contract storage.
    ///
    /// Returns `None` if no `value` exists at the given `key`.
    #[inline(always)]
    pub fn get(&self, key: K) -> Option<V> {
        ink::env::get_contract_storage(&self.storage_key(key))
            .unwrap_or_else(|error| panic!("Failed to get value in RawMapping: {:?}", error))
    }

    /// Get the size of a value stored at `key` in the contract storage.
    ///
    /// Returns `None` if no `value` exists at the given `key`.
    #[inline(always)]
    pub fn size(&self, key: K) -> Option<u32> {
        ink::env::contains_contract_storage(&self.storage_key(key))
    }

    /// Checks if a value is stored at the given `key` in the contract storage.
    ///
    /// Returns `None` if no `value` exists at the given `key`.
    #[inline(always)]
    pub fn contains(&self, key: K) -> bool {
        ink::env::contains_contract_storage(&self.storage_key(key)).is_some()
    }

    /// Clears the value at `key` from storage.
    #[inline(always)]
    pub fn remove(&self, key: K)
    where
        K: scale::Encode,
    {
        ink::env::clear_contract_storage(&self.storage_key(key));
    }

    /// Returns a `Key` pointer used internally by the storage API.
    ///
    /// This key is a combination of the `RawMapping`'s internal `offset_key`
    /// and the user provided `key`.
    #[inline(always)]
    fn storage_key(&self, key: K) -> (T, K)
    where
        K: scale::Encode,
    {
        (self.prefix, key)
    }
}
