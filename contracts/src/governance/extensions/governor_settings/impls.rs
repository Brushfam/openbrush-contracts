use crate::{
    governance::extensions::governor_settings::{
        Data,
        SettingsEvents
    },
    traits::{
        errors::GovernanceError,
    },
};
use openbrush::traits::{Storage, Timestamp};

pub trait GovernorSettingsImpl: Storage<Data> + SettingsEvents {
    fn voting_delay(&self) -> Timestamp {
        self.data().voting_delay
    }

    fn voting_period(&self) -> Timestamp {
        self.data().voting_period
    }

    fn proposal_threshold(&self) -> u128 {
        self.data().proposal_threshold
    }

    fn set_voting_delay(&mut self, new_voting_delay: Timestamp) {
        self.emit_voting_delay_set_event(self.data().voting_delay, new_voting_delay);
        self.data().voting_delay.set(new_voting_delay);
    }

    fn set_voting_period(&mut self, new_voting_period: Timestamp) -> Result<(), GovernanceError> {
        if new_voting_period == 0 {
            return Err(GovernanceError::InvalidVotingPeriod(0))
        }
        self.emit_voting_period_set_event(self.data().voting_period, new_voting_period);
        self.data().voting_period.set(new_voting_period);
        Ok(())
    }

    fn set_proposal_threshold(&mut self, new_proposal_threshold: u128) {
        self.emit_proposal_threshold_set_event(self.data().proposal_threshold, new_proposal_threshold);
        self.data().proposal_threshold.set(new_proposal_threshold);
    }
}
