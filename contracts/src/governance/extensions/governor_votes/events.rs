use openbrush::traits::{
    AccountId,
    Balance,
};

pub trait VotesEvents {
    fn emit_delegate_changed_event(
        &self,
        _delegator: &AccountId,
        _from_delegate: &AccountId,
        _to_delegate: &AccountId,
    ) {
        unimplemented!("emit_delegate_changed_event")
    }

    fn emit_delegate_votes_changed_event(&self, _delegate: &AccountId, _previous_votes: Balance, _new_votes: Balance) {
        unimplemented!("emit_delegate_votes_changed_event")
    }
}
