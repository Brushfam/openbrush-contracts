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

pub trait GovernorSettingsImpl: Storage<Data> + GovernorSettingsInternal + GovernorInternal {
    #[modifiers(only_governance)]
    fn set_voting_delay(&mut self, new_voting_delay: u64) -> Result<(), GovernanceError> {
        self._set_voting_delay(new_voting_delay)
    }

    #[modifiers(only_governance)]
    fn set_voting_period(&mut self, new_voting_period: u64) -> Result<(), GovernanceError> {
        self._set_voting_period(new_voting_period)
    }

    #[modifiers(only_governance)]
    fn set_proposal_threshold(&mut self, new_proposal_threshold: u128) -> Result<(), GovernanceError> {
        self._set_proposal_threshold(new_proposal_threshold)
    }

    fn voting_delay(&self) -> u64 {
        self.data::<Data>().voting_delay.get_or_default()
    }

    fn voting_period(&self) -> u64 {
        self.data::<Data>().voting_period.get_or_default()
    }

    fn proposal_threshold(&self) -> u128 {
        self.data::<Data>().proposal_threshold.get_or_default()
    }
}
