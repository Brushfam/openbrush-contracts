// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::traits::diamond::*;
use ink::prelude::vec::Vec;
use openbrush::traits::Hash;

#[openbrush::wrapper]
pub type DiamondLoupeRef = dyn DiamondLoupe;

/// Trait which implements functions of Diamond Loupe to lookup the functionality of the diamond contract
#[openbrush::trait_definition]
pub trait DiamondLoupe {
    /// Returns code hashes of all registered facets along with their registered function selectors
    #[ink(message)]
    fn facets(&self) -> Vec<FacetCut>;

    /// Returns all the function selectors supported by a specific facet
    #[ink(message)]
    fn facet_function_selectors(&self, facet: Hash) -> Vec<Selector>;

    /// Returns all the code hashes of facets used by the diamond
    #[ink(message)]
    fn facet_code_hashes(&self) -> Vec<Hash>;

    /// Returns the code hash of a facet which supports the given selector
    /// Returns empty hash if selector is not found
    #[ink(message)]
    fn facet_code_hash(&self, selector: Selector) -> Option<Hash>;
}
