use crate::{
    extensions::governor_settings::{
        Data,
        GovernorSettingsInternal,
    },
    traits::errors::GovernanceError,
};
use openbrush::traits::Storage;

pub trait GovernorSettingsImpl: Storage<Data> + GovernorSettingsInternal {
    fn set_voting_delay(&mut self, new_voting_delay: u128) -> Result<(), GovernanceError> {
        self._set_voting_delay(new_voting_delay)
    }

    fn set_voting_period(&mut self, new_voting_period: u128) -> Result<(), GovernanceError> {
        self._set_voting_period(new_voting_period)
    }

    fn set_proposal_threshold(&mut self, new_proposal_threshold: u128) -> Result<(), GovernanceError> {
        self._set_proposal_threshold(new_proposal_threshold)
    }

    fn voting_delay(&self) -> u128 {
        self.data().voting_delay.get_or_default()
    }

    fn voting_period(&self) -> u128 {
        self.data().voting_period.get_or_default()
    }

    fn proposal_threshold(&self) -> u128 {
        self.data().proposal_threshold.get_or_default()
    }
}
