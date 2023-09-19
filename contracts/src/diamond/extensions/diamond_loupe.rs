// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

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
use ink::prelude::vec::Vec;
use openbrush::{
    storage::{
        Mapping,
        ValueGuard,
    },
    traits::{
        Hash,
        Storage,
    },
};
pub use ownable::{
    Internal as _,
    InternalImpl as _,
    OwnableImpl,
};

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    // number of registered code hashes
    #[lazy]
    pub code_hashes: u32,
    // mapping of facet to its position in all facets list
    pub hash_to_id: Mapping<Hash, u32>,
    // mapping of facet id to its facet
    pub id_to_hash: Mapping<u32, Hash, ValueGuard<u32>>,
}

pub trait DiamondCutLoupeImpl: Storage<Data> {
    #[inline(always)]
    fn _on_add_facet(&mut self, code_hash: Hash) {
        let hash_id = self.data().code_hashes.get_or_default();
        self.data().hash_to_id.insert(&code_hash, &hash_id);
        self.data().id_to_hash.insert(hash_id, &code_hash);
        self.data().code_hashes.set(&(hash_id + 1));
    }

    fn _on_remove_facet(&mut self, code_hash: Hash) {
        let new_hash_id = self.data().code_hashes.get_or_default() - 1;
        let removed_hash_id = self.data().hash_to_id.get(&code_hash).unwrap();
        let last_hash = self.data().id_to_hash.get(new_hash_id).unwrap();

        if last_hash != code_hash {
            self.data().id_to_hash.insert(removed_hash_id, &last_hash);
            self.data().hash_to_id.insert(&last_hash, &removed_hash_id);
            self.data().id_to_hash.remove(new_hash_id);
        } else {
            self.data().id_to_hash.remove(removed_hash_id);
        }

        self.data().hash_to_id.remove(&code_hash);
        self.data().code_hashes.set(&new_hash_id);
    }
}

pub trait DiamondLoupeImpl: Storage<diamond::Data> + Storage<Data> {
    fn facets(&self) -> Vec<FacetCut> {
        let mut out_vec = Vec::new();
        for i in 0..self.data::<Data>().code_hashes.get_or_default() {
            let hash = self.data::<Data>().id_to_hash.get(i).unwrap();
            let selectors = self.data::<diamond::Data>().hash_to_selectors.get(&hash).unwrap();
            out_vec.push(FacetCut { hash, selectors })
        }
        out_vec
    }

    fn facet_function_selectors(&self, facet: Hash) -> Vec<Selector> {
        self.data::<diamond::Data>()
            .hash_to_selectors
            .get(&facet)
            .unwrap_or(Vec::<Selector>::new())
    }

    fn facet_code_hashes(&self) -> Vec<Hash> {
        let mut out_vec = Vec::new();
        for i in 0..self.data::<Data>().code_hashes.get_or_default() {
            out_vec.push(self.data::<Data>().id_to_hash.get(i).unwrap())
        }
        out_vec
    }

    fn facet_code_hash(&self, selector: Selector) -> Option<Hash> {
        self.data::<diamond::Data>().selector_to_hash.get(&selector)
    }
}
