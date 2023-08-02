pub trait QuorumEvents {
    fn emit_quorum_numerator_updated(&self, old_quorum_numerator: u128, new_quorum_numerator: u128);

    fn emit_governor_invalid_quorum_fraction(&self, quorum_numerator: u128, quorum_denominator: u128);
}
