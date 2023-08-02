use openbrush::traits::{
    AccountId,
    Balance,
};

pub trait VotesQuorumFractionEvents {
    fn emit_quorum_numerator_updated_event(&self, old_quorum_numerator: u128, new_quorum_numerator: u128);
}
