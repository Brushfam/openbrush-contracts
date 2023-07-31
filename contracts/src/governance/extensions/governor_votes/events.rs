use openbrush::traits::{AccountId, Balance};

pub trait VotesEvents {
    fn emit_delegate_changed_event(
        &self,
        delegator: &AccountId,
        from_delegate: &AccountId,
        to_delegate: &AccountId,
    );

    fn emit_delegate_votes_changed_event(
        &self,
        delegate: &AccountId,
        previous_votes: Balance,
        new_votes: Balance,
    );
}