// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub trait QuorumEvents {
    /// Emitted when the quorum denominator is updated
    fn emit_quorum_numerator_updated(&self, _old_quorum_numerator: u128, _new_quorum_numerator: u128) {}

    /// Emitted when the governor quorum is invalid
    fn emit_governor_invalid_quorum_fraction(&self, _quorum_numerator: u128, _quorum_denominator: u128) {}
}
