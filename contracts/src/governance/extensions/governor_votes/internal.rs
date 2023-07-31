use crate::utils::checkpoint::Checkpoints;
use openbrush::traits::{
    AccountId,
    Balance,
};

pub trait VotesInternal {
    fn _get_total_supply(&self) -> Balance;

    fn _delegate(&mut self, delegator: &AccountId, delegatee: &AccountId);

    fn _transfer_voting_units(&mut self, from: &AccountId, to: &AccountId, amount: Balance);

    fn _move_delegate_votes(&mut self, from: &AccountId, to: &AccountId, amount: Balance);

    fn _num_checkpoints(&self, account: &AccountId) -> u32;

    fn _checkpoints(&self, account: &AccountId, pos: u32) -> Checkpoints;

    fn _push(&mut self, store: &mut Checkpoints, op: fn(u128, u128) -> u128, delta: Balance);

    fn _add(&mut self, a: u128, b: u128) -> u128;

    fn _sub(&mut self, a: u128, b: u128) -> u128;

    fn _get_voting_units(&self, account: &AccountId) -> Balance;
}
