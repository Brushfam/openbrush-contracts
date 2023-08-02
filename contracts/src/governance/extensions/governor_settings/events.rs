pub trait GovernorSettingEvents {
    fn emit_voting_delay_set(&self, old_voting_delay: Option<u128>, new_voting_delay: u128);

    fn emit_voting_period_set(&self, old_voting_period: Option<u128>, new_voting_period: u128);

    fn emit_proposal_threshold_set(&self, old_proposal_threshold: Option<u128>, new_proposal_threshold: u128);
}
