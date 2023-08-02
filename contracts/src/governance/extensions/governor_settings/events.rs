use openbrush::traits::{AccountId, Balance, Timestamp};

pub trait SettingsEvents {
    fn emit_voting_delay_set_event(
        &self,
        old_voting_delay: Timestamp,
        new_voting_delay: Timestamp
    );

    fn emit_voting_period_set_event(
        &self,
        old_voting_period: Timestamp,
        new_voting_period: Timestamp,
    );

    fn emit_proposal_threshold_set_event(
        &self,
        old_proposal_threshold: u128,
        new_proposal_threshold: u128,
    );
}
