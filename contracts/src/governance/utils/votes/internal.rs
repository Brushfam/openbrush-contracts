use crate::{
    governance::utils::votes::{
        Data,
        VotesEvents,
    },
    traits::errors::{
        CheckpointsError,
        GovernanceError,
    },
    utils::checkpoint::{
        Checkpoint,
        Checkpoints,
    },
};
use openbrush::traits::{
    AccountId,
    Balance,
    Storage,
};

pub trait VotesInternal: Storage<Data> + VotesEvents {
    fn _get_total_supply(&self) -> Balance {
        self.data::<Data>().total_checkpoints.get_or_default().latest()
    }

    fn _delegates(&self, delegator: &Option<AccountId>) -> Option<AccountId> {
        self.data::<Data>().delegation.get(&delegator)
    }

    fn _delegate(
        &mut self,
        delegator: &Option<AccountId>,
        delegatee: &Option<AccountId>,
    ) -> Result<(), GovernanceError> {
        let old_delegate = self._delegates(&delegator);

        self.data::<Data>()
            .delegation
            .insert(&delegator, &delegatee.ok_or(GovernanceError::InvalidInput)?);

        self.emit_delegate_changed_event(&delegator, &old_delegate, delegatee);

        self._move_delegate_votes(
            &old_delegate,
            delegatee,
            self._get_voting_units(&delegator.ok_or(GovernanceError::InvalidInput)?),
        )
    }

    fn _transfer_voting_units(
        &mut self,
        from: &Option<AccountId>,
        to: &Option<AccountId>,
        amount: Balance,
    ) -> Result<(), GovernanceError> {
        let mut store = self.data::<Data>().total_checkpoints.get_or_default();
        if from.is_none() {
            self._push(&mut store, Self::_add, amount)?;
        }
        if to.is_none() {
            self._push(&mut store, Self::_sub, amount)?;
        }
        self._move_delegate_votes(&self._delegates(from), &self._delegates(to), amount)
    }

    fn _move_delegate_votes(
        &mut self,
        from: &Option<AccountId>,
        to: &Option<AccountId>,
        amount: Balance,
    ) -> Result<(), GovernanceError> {
        if from != to && amount > 0 {
            if from.is_some() {
                let from_addr = from.unwrap();

                let mut store = self
                    .data::<Data>()
                    .delegate_checkpoints
                    .get(&from_addr)
                    .unwrap_or_default();

                let (old_value, new_value) = self._push(&mut store, Self::_sub, amount)?;
                self.data::<Data>().delegate_checkpoints.insert(&from_addr, &store);
                self.emit_delegate_votes_changed_event(&from_addr, old_value, new_value);
            }
            if to.is_some() {
                let to_addr = to.unwrap();

                let mut store = self
                    .data::<Data>()
                    .delegate_checkpoints
                    .get(&to_addr)
                    .unwrap_or_default();

                let (old_value, new_value) = self._push(&mut store, Self::_add, amount)?;
                self.data::<Data>().delegate_checkpoints.insert(&to_addr, &store);
                self.emit_delegate_votes_changed_event(&to_addr, old_value, new_value);
            }
        }
        Ok(())
    }

    fn _num_checkpoints(&self, account: &AccountId) -> Result<u32, GovernanceError> {
        Ok(self
            .data::<Data>()
            .delegate_checkpoints
            .get(&account)
            .unwrap_or_default()
            .len() as u32)
    }

    fn _checkpoints(&self, account: &AccountId, pos: u32) -> Result<Checkpoint, GovernanceError> {
        let checkpoints = self
            .data::<Data>()
            .delegate_checkpoints
            .get(&account)
            .unwrap_or_default();
        checkpoints
            .at(pos as usize)
            .ok_or(GovernanceError::IndexOutOfRange)
            .cloned()
    }

    fn _push(
        &mut self,
        store: &mut Checkpoints,
        op: fn(u128, u128) -> Result<u128, GovernanceError>,
        delta: Balance,
    ) -> Result<(u128, u128), GovernanceError> {
        let (old_value, new_value) = store
            .push(Self::env().block_timestamp(), op(store.latest(), delta)?)
            .map_err(|err| <CheckpointsError as Into<GovernanceError>>::into(err))?;
        Ok((old_value, new_value))
    }

    fn _add(a: u128, b: u128) -> Result<u128, GovernanceError> {
        Ok(a.checked_add(b).ok_or(GovernanceError::Overflow)?)
    }

    fn _sub(a: u128, b: u128) -> Result<u128, GovernanceError> {
        Ok(a.checked_sub(b).ok_or(GovernanceError::Overflow)?)
    }

    fn _get_voting_units(&self, account: &AccountId) -> Balance;
}
