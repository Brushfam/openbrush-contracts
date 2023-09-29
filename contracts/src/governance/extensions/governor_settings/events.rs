// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub trait GovernorSettingsEvents {
    /// Emits this event when the voting delay is set
    fn emit_voting_delay_set(&self, _old_voting_delay: Option<u64>, _new_voting_delay: u64) {}

    /// Emits this event when the voting period is set
    fn emit_voting_period_set(&self, _old_voting_period: Option<u64>, _new_voting_period: u64) {}

    /// Emits this event when the proposal threshold is set
    fn emit_proposal_threshold_set(&self, _old_proposal_threshold: Option<u128>, _new_proposal_threshold: u128) {}
}
