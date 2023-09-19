// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::traits::errors::GovernanceError;

/// Extension of `Governor` for settings updatable through governance.
#[openbrush::trait_definition]
pub trait GovernorSettings {
    /// Sets the voting delay
    #[ink(message)]
    fn set_voting_delay(&mut self, new_voting_delay: u64) -> Result<(), GovernanceError>;

    /// Sets the voting period
    #[ink(message)]
    fn set_voting_period(&mut self, new_voting_period: u64) -> Result<(), GovernanceError>;

    /// Sets the proposal threshold
    #[ink(message)]
    fn set_proposal_threshold(&mut self, new_proposal_threshold: u128) -> Result<(), GovernanceError>;

    /// Returns the voting delay
    #[ink(message)]
    fn voting_delay(&self) -> u64;

    /// Returns the voting period
    #[ink(message)]
    fn voting_period(&self) -> u64;

    /// Returns the proposal threshold
    #[ink(message)]
    fn proposal_threshold(&self) -> u128;
}

#[openbrush::wrapper]
pub type GovernorSettingsRef = dyn GovernorSettings;
