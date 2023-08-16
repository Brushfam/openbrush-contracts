// Copyright (c) 2023 Brushfam
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

use crate::{
    extensions::governor_settings::{
        Data,
        GovernorSettingsInternal,
    },
    governance::governor::only_governance,
    governor::GovernorInternal,
    traits::errors::GovernanceError,
};
use openbrush::{
    modifiers,
    traits::Storage,
};

/// Extension of {Governor} for settings updatable through governance.
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
