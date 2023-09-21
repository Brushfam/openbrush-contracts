// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

use crate::governance::{
    extensions::governor_settings::{
        Data,
        GovernorSettingsInternal,
    },
    governor::{
        only_governance,
        GovernorInternal,
    },
};
pub use crate::{
    governance::extensions::governor_settings,
    traits::governance::extensions::governor_settings::*,
};
use openbrush::{
    modifiers,
    traits::Storage,
};

/// Extension of `Governor` for settings updatable through governance.
pub trait GovernorSettingsImpl: Storage<Data> + GovernorSettingsInternal + GovernorInternal {
    /// Sets the voting delay
    #[modifiers(only_governance)]
    fn set_voting_delay(&mut self, new_voting_delay: u64) -> Result<(), GovernanceError> {
        self._set_voting_delay(new_voting_delay)
    }

    /// Sets the voting period
    #[modifiers(only_governance)]
    fn set_voting_period(&mut self, new_voting_period: u64) -> Result<(), GovernanceError> {
        self._set_voting_period(new_voting_period)
    }

    /// Sets the proposal threshold
    #[modifiers(only_governance)]
    fn set_proposal_threshold(&mut self, new_proposal_threshold: u128) -> Result<(), GovernanceError> {
        self._set_proposal_threshold(new_proposal_threshold)
    }

    /// Returns the voting delay
    fn voting_delay(&self) -> u64 {
        self.data::<Data>().voting_delay.get_or_default()
    }

    /// Returns the voting period
    fn voting_period(&self) -> u64 {
        self.data::<Data>().voting_period.get_or_default()
    }

    /// Returns the proposal threshold
    fn proposal_threshold(&self) -> u128 {
        self.data::<Data>().proposal_threshold.get_or_default()
    }
}
