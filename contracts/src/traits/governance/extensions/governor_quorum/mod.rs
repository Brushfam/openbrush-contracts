// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::traits::errors::GovernanceError;
use openbrush::traits::Timestamp;

/// Extension of `Governor` for voting weight extraction from an `PSP22Votes` token and a quorum expressed as a
/// fraction of the total supply.
#[openbrush::trait_definition]
pub trait Quorum {
    /// Returns the current quorum numerator
    #[ink(message)]
    fn quorum_numerator(&self) -> u128;

    /// Returns the quorum numerator at a given timestamp
    #[ink(message)]
    fn quorum_numerator_at(&self, timestamp: Timestamp) -> u128;

    /// Returns the current quorum denominator
    #[ink(message)]
    fn quorum_denominator(&self) -> u128;

    /// Returns the quorum at a given timestamp
    #[ink(message)]
    fn quorum(&self, time_point: Timestamp) -> Result<u128, GovernanceError>;

    /// Updates the quorum numerator
    #[ink(message)]
    fn update_quorum_numerator(&mut self, numerator: u128) -> Result<(), GovernanceError>;
}

#[openbrush::wrapper]
pub type QuorumRef = dyn Quorum;
