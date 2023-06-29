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
    diamond,
    diamond::extensions::diamond_loupe,
    ownable,
    traits::{
        diamond::extensions::diamond_loupe::*,
        ownable::*,
    },
};
pub use diamond::{
    DiamondImpl,
    Internal as _,
    InternalImpl as _,
};
pub use ink::prelude::vec::Vec;
use openbrush::{
    storage::{
        Mapping,
        ValueGuard,
    },
    traits::{
        DefaultEnv,
        Hash,
        StorageAccess,
    },
    with_data,
};
pub use ownable::{
    Internal as _,
    InternalImpl as _,
    OwnableImpl,
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::storage_item(STORAGE_KEY)]
pub struct Data {
    // number of registered code hashes
    pub code_hashes: u32,
    // mapping of facet to its position in all facets list
    pub hash_to_id: Mapping<Hash, u32>,
    // mapping of facet id to its facet
    pub id_to_hash: Mapping<u32, Hash, ValueGuard<u32>>,
    pub _reserved: Option<()>,
}

#[cfg(feature = "upgradeable")]
pub type DataType = Lazy<Data, ResolverKey<AutoKey>>;
#[cfg(not(feature = "upgradeable"))]
pub type DataType = Data;

pub trait DiamondCutLoupeImpl: StorageAccess<Data> + Sized {
    #[inline(always)]
    fn _on_add_facet(&mut self, code_hash: Hash) {
        let hash_id = self.get_or_default().code_hashes;
        with_data!(self, data, {
            data.hash_to_id.insert(&code_hash, &hash_id);
            data.id_to_hash.insert(hash_id, &code_hash);
            data.code_hashes += 1;
        });
    }

    fn _on_remove_facet(&mut self, code_hash: Hash) {
        let new_hash_id = self.get_or_default().code_hashes - 1;
        let removed_hash_id = self.get_or_default().hash_to_id.get(&code_hash).unwrap();
        let last_hash = self.get_or_default().id_to_hash.get(new_hash_id).unwrap();

        if last_hash != code_hash {
            with_data!(self, data, {
                data.id_to_hash.insert(removed_hash_id, &last_hash);
                data.hash_to_id.insert(&last_hash, &removed_hash_id);
                data.id_to_hash.remove(new_hash_id);
            });
        } else {
            with_data!(self, data, {
                data.hash_to_id.remove(&last_hash);
            });
        }

        with_data!(self, data, {
            data.hash_to_id.remove(&code_hash);
            data.code_hashes = new_hash_id;
        });
    }
}

pub trait DiamondLoupeImpl:
    Storage<diamond::DataType> + StorageAccess<diamond::Data> + StorageAccess<Data> + Sized
{
    fn facets(&self) -> Vec<FacetCut> {
        let mut out_vec = Vec::new();
        for i in 0..StorageAccess::<Data>::get_or_default(self).code_hashes {
            let hash = StorageAccess::<Data>::get_or_default(self).id_to_hash.get(i).unwrap();
            let selectors = StorageAccess::<diamond::Data>::get_or_default(self)
                .hash_to_selectors
                .get(&hash)
                .unwrap();
            out_vec.push(FacetCut { hash, selectors })
        }
        out_vec
    }

    fn facet_function_selectors(&self, facet: Hash) -> Vec<Selector> {
        StorageAccess::<diamond::Data>::get_or_default(self)
            .hash_to_selectors
            .get(&facet)
            .unwrap_or(Vec::<Selector>::new())
    }

    fn facet_code_hashes(&self) -> Vec<Hash> {
        let mut out_vec = Vec::new();
        for i in 0..StorageAccess::<Data>::get_or_default(self).code_hashes {
            out_vec.push(StorageAccess::<Data>::get_or_default(self).id_to_hash.get(i).unwrap())
        }
        out_vec
    }

    fn facet_code_hash(&self, selector: Selector) -> Option<Hash> {
        StorageAccess::<diamond::Data>::get_or_default(self)
            .selector_to_hash
            .get(&selector)
    }
}
