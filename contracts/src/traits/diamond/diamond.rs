// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::traits::{
    errors::{
        DiamondError,
        OwnableError,
    },
    ownable::*,
};
use ink::prelude::vec::Vec;
use openbrush::traits::Hash;

#[openbrush::wrapper]
pub type DiamondRef = dyn Diamond;

pub type Selector = [u8; 4];

/// Struct which we use to initialize/update/remove a facet in the diamond
#[derive(Default, Debug, Clone, PartialEq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct FacetCut {
    /// The `hash` of the code that should be executed.
    pub hash: Hash,
    /// The selector bytes that identify the function that should be called.
    pub selectors: Vec<Selector>,
}

/// Struct which we use to initialize the diamond contract
#[derive(Default, Debug, Clone, PartialEq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct InitCall {
    /// The `hash` of the code that should be executed.
    pub hash: Hash,
    /// The selector bytes that identify the function that should be called.
    pub selector: Selector,
    /// The SCALE encoded parameters that are passed to the called function.
    pub input: Vec<u8>,
}

/// Trait to be implemented in the contract which holds the diamond storage
#[openbrush::trait_definition]
pub trait Diamond {
    /// This function is used to add, replace and remove facets from the diamond
    ///
    /// `cuts` vector of facet cuts, each cut contains the code hash of the facet
    /// as well as the selectors of functions.
    /// If `cuts` is empty, we will remove this facet from diamond
    /// If `cuts` contains a selector which already exists for a different facet we will return an error (user should remove this facet first)
    /// If `cuts` does not contain some selectors which are already registered for this facet, those selectors will be removed from diamond
    /// `init` optional struct which identifies a call to be executed, this struct contains the code hash
    /// of the executed contract, selector of the executed function and input data to be passed to the called
    #[ink(message)]
    fn diamond_cut(&mut self, cuts: Vec<FacetCut>, init: Option<InitCall>) -> Result<(), DiamondError>;
}
