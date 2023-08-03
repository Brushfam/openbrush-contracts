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
