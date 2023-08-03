pub trait QuorumEvents {
    fn emit_quorum_numerator_updated(&self, _old_quorum_numerator: u128, _new_quorum_numerator: u128) {
        unimplemented!("emit_quorum_numerator_updated")
    }

    fn emit_governor_invalid_quorum_fraction(&self, _quorum_numerator: u128, _quorum_denominator: u128) {
        unimplemented!("emit_governor_invalid_quorum_fraction")
    }
}
