pub trait GovernorSettingsEvents {
    fn emit_voting_delay_set(&self, _old_voting_delay: Option<u64>, _new_voting_delay: u64) {
        unimplemented!("emit_voting_delay_set")
    }

    fn emit_voting_period_set(&self, _old_voting_period: Option<u64>, _new_voting_period: u64) {
        unimplemented!("emit_voting_period_set")
    }

    fn emit_proposal_threshold_set(&self, _old_proposal_threshold: Option<u128>, _new_proposal_threshold: u128) {
        unimplemented!("emit_proposal_threshold_set")
    }
}
