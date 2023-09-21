// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::traits::errors::PausableError;

#[openbrush::wrapper]
pub type PausableRef = dyn Pausable;

/// Contract trait, which allows children to implement an emergency stop
/// mechanism that an authorized account can trigger.
#[openbrush::trait_definition]
pub trait Pausable {
    /// Returns true if the contract is paused, and false otherwise.
    #[ink(message)]
    fn paused(&self) -> bool;
}
