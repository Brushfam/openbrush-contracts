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
        GovernorSettingsEvents,
    },
    traits::errors::GovernanceError,
};
use openbrush::traits::Storage;

pub trait GovernorSettingsInternal: Storage<Data> + GovernorSettingsEvents {
    fn _set_voting_delay(&mut self, new_voting_delay: u64) -> Result<(), GovernanceError> {
        let old_voting_delay = self.data().voting_delay.get();
        self.data().voting_delay.set(&new_voting_delay);
        self.emit_voting_delay_set(old_voting_delay, new_voting_delay);
        Ok(())
    }

    fn _set_voting_period(&mut self, new_voting_period: u64) -> Result<(), GovernanceError> {
        let old_voting_period = self.data().voting_period.get();
        self.data().voting_period.set(&new_voting_period);
        self.emit_voting_period_set(old_voting_period, new_voting_period);
        Ok(())
    }

    fn _set_proposal_threshold(&mut self, new_proposal_threshold: u128) -> Result<(), GovernanceError> {
        let old_proposal_threshold = self.data().proposal_threshold.get();
        self.data().proposal_threshold.set(&new_proposal_threshold);
        self.emit_proposal_threshold_set(old_proposal_threshold, new_proposal_threshold);
        Ok(())
    }

    fn _init_governor_settings(
        &mut self,
        voting_delay: u64,
        voting_period: u64,
        proposal_threshold: u128,
    ) -> Result<(), GovernanceError> {
        self._set_voting_delay(voting_delay)?;
        self._set_voting_period(voting_period)?;
        self._set_proposal_threshold(proposal_threshold)?;
        Ok(())
    }
}
