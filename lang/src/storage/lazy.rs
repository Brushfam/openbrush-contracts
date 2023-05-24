// Copyright 2018-2022 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::traits::StorageAccess;
use core::marker::PhantomData;
use ink::{
    primitives::Key,
    storage::traits::{
        AutoKey,
        Storable,
        StorableHint,
        StorageKey,
        StorageLayout,
    },
};
use scale::{
    Error,
    Input,
    Output,
};

#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Lazy<V, KeyType: StorageKey = AutoKey> {
    _marker: PhantomData<fn() -> (V, KeyType)>,
}

/// We implement this manually because the derived implementation adds trait bounds.
impl<V, KeyType> Default for Lazy<V, KeyType>
where
    KeyType: StorageKey,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<V, KeyType> Lazy<V, KeyType>
where
    KeyType: StorageKey,
{
    /// Creates a new empty `Lazy`.
    pub const fn new() -> Self {
        Self { _marker: PhantomData }
    }
}

impl<V, KeyType> core::fmt::Debug for Lazy<V, KeyType>
where
    KeyType: StorageKey,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lazy").field("key", &KeyType::KEY).finish()
    }
}

impl<V, KeyType> Lazy<V, KeyType>
where
    V: Storable,
    KeyType: StorageKey,
{
    /// Reads the `value` from the contract storage, if it exists.
    pub fn get(&self) -> Option<V> {
        match ink::env::get_contract_storage::<Key, V>(&KeyType::KEY) {
            Ok(Some(value)) => Some(value),
            _ => None,
        }
    }

    /// Writes the given `value` to the contract storage.
    pub fn set(&mut self, value: &V) {
        ink::env::set_contract_storage::<Key, V>(&KeyType::KEY, value);
    }
}

impl<V, KeyType> Lazy<V, KeyType>
where
    V: Storable + Default,
    KeyType: StorageKey,
{
    /// Reads the `value` from the contract storage.
    ///
    /// Returns the default value for the storage type if no `value` exists.
    pub fn get_or_default(&self) -> V {
        match ink::env::get_contract_storage::<Key, V>(&KeyType::KEY) {
            Ok(Some(value)) => value,
            _ => {
                let mut instance = Default::default();

                crate::traits::Initializable::initialize(&mut instance);

                instance
            }
        }
    }
}

impl<V, KeyType> StorageAccess<V> for Lazy<V, KeyType>
where
    V: Storable + Default,
    KeyType: StorageKey,
{
    fn get(&self) -> Option<V> {
        self.get()
    }

    fn set(&mut self, value: &V) {
        self.set(value)
    }

    fn get_or_default(&self) -> V {
        self.get_or_default()
    }
}

impl<V, KeyType> Storable for Lazy<V, KeyType>
where
    KeyType: StorageKey,
{
    #[inline(always)]
    fn encode<T: Output + ?Sized>(&self, _dest: &mut T) {}

    #[inline(always)]
    fn decode<I: Input>(_input: &mut I) -> Result<Self, Error> {
        Ok(Default::default())
    }
}

impl<V, Key, InnerKey> StorableHint<Key> for Lazy<V, InnerKey>
where
    Key: StorageKey,
    InnerKey: StorageKey,
    V: StorableHint<Key>,
{
    type Type = Lazy<V::Type, Key>;
    type PreferredKey = InnerKey;
}

impl<V, KeyType> StorageKey for Lazy<V, KeyType>
where
    KeyType: StorageKey,
{
    const KEY: Key = KeyType::KEY;
}

#[cfg(feature = "std")]
const _: () = {
    use ink::{
        metadata::layout::{
            Layout,
            LayoutKey,
            RootLayout,
        },
        storage::traits::StorageLayout,
    };

    impl<V, KeyType> StorageLayout for Lazy<V, KeyType>
    where
        V: StorageLayout + scale_info::TypeInfo + 'static,
        KeyType: StorageKey + scale_info::TypeInfo + 'static,
    {
        fn layout(_: &Key) -> Layout {
            Layout::Root(RootLayout::new(
                LayoutKey::from(&KeyType::KEY),
                <V as StorageLayout>::layout(&KeyType::KEY),
            ))
        }
    }
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::{
        Lazy,
        ValueGuard,
    };
    use ink::storage::traits::ManualKey;

    #[test]
    fn set_and_get_work() {
        ink::env::test::run_test::<ink::env::DefaultEnvironment, _>(|_| {
            let mut storage: Lazy<u8> = Lazy::new();
            storage.set(&2);
            assert_eq!(storage.get(), Some(2));

            Ok(())
        })
        .unwrap()
    }

    #[test]
    fn set_and_get_work_for_two_lazy_with_same_manual_key() {
        ink::env::test::run_test::<ink::env::DefaultEnvironment, _>(|_| {
            let mut storage: Lazy<u8, ValueGuard<u8>, ManualKey<123>> = Lazy::new();
            storage.set(&2);
            let storage2: Lazy<u8, ValueGuard<u8>, ManualKey<123>> = Lazy::new();
            assert_eq!(storage2.get(), Some(2));

            Ok(())
        })
        .unwrap()
    }

    #[test]
    fn gets_or_default_if_no_key_set() {
        ink::env::test::run_test::<ink::env::DefaultEnvironment, _>(|_| {
            let storage: Lazy<u8> = Lazy::new();
            assert_eq!(storage.get_or_default(), 0);

            Ok(())
        })
        .unwrap()
    }

    #[test]
    fn gets_returns_none_if_no_value_was_set() {
        ink::env::test::run_test::<ink::env::DefaultEnvironment, _>(|_| {
            let storage: Lazy<u8> = Lazy::new();
            assert_eq!(storage.get(), None);

            Ok(())
        })
        .unwrap()
    }
}
