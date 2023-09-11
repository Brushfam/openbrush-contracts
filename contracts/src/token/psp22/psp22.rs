// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::{
    psp22,
    traits::psp22::*,
};
use ink::prelude::vec::Vec;
use openbrush::{
    storage::{
        Mapping,
        TypeGuard,
    },
    traits::{
        AccountId,
        Balance,
        Storage,
    },
};
pub use psp22::{
    Internal as _,
    InternalImpl as _,
    PSP22Impl as _,
};

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    #[lazy]
    pub supply: Balance,
    pub balances: Mapping<AccountId, Balance>,
    pub allowances: Mapping<(AccountId, AccountId), Balance, AllowancesKey>,
}

pub struct AllowancesKey;

impl<'a> TypeGuard<'a> for AllowancesKey {
    type Type = &'a (&'a AccountId, &'a AccountId);
}

pub trait PSP22Impl: Storage<Data> + Internal {
    fn total_supply(&self) -> Balance {
        self._total_supply()
    }

    fn balance_of(&self, owner: AccountId) -> Balance {
        self._balance_of(&owner)
    }

    fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
        self._allowance(&owner, &spender)
    }

    fn transfer(&mut self, to: AccountId, value: Balance, data: Vec<u8>) -> Result<(), PSP22Error> {
        let from = Self::env().caller();
        self._transfer_from_to(from, to, value, data)?;
        Ok(())
    }

    fn transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        value: Balance,
        data: Vec<u8>,
    ) -> Result<(), PSP22Error> {
        let caller = Self::env().caller();
        let allowance = self._allowance(&from, &caller);

        if allowance < value {
            return Err(PSP22Error::InsufficientAllowance)
        }

        self._approve_from_to(from, caller, allowance - value)?;
        self._transfer_from_to(from, to, value, data)?;
        Ok(())
    }

    fn approve(&mut self, spender: AccountId, value: Balance) -> Result<(), PSP22Error> {
        let owner = Self::env().caller();
        self._approve_from_to(owner, spender, value)?;
        Ok(())
    }

    fn increase_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), PSP22Error> {
        let owner = Self::env().caller();
        self._approve_from_to(owner, spender, self._allowance(&owner, &spender) + delta_value)
    }

    fn decrease_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), PSP22Error> {
        let owner = Self::env().caller();
        let allowance = self._allowance(&owner, &spender);

        if allowance < delta_value {
            return Err(PSP22Error::InsufficientAllowance)
        }

        self._approve_from_to(owner, spender, allowance - delta_value)
    }
}

pub trait Internal {
    /// User must override those methods in their contract.
    fn _emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _amount: Balance);

    fn _emit_approval_event(&self, _owner: AccountId, _spender: AccountId, _amount: Balance);

    fn _total_supply(&self) -> Balance;

    fn _balance_of(&self, owner: &AccountId) -> Balance;

    fn _allowance(&self, owner: &AccountId, spender: &AccountId) -> Balance;

    fn _transfer_from_to(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: Balance,
        data: Vec<u8>,
    ) -> Result<(), PSP22Error>;

    fn _approve_from_to(&mut self, owner: AccountId, spender: AccountId, amount: Balance) -> Result<(), PSP22Error>;

    fn _mint_to(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error>;

    fn _burn_from(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error>;

    fn _before_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _amount: &Balance,
    ) -> Result<(), PSP22Error>;

    fn _after_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _amount: &Balance,
    ) -> Result<(), PSP22Error>;
}

pub trait InternalImpl: Storage<Data> + Internal {
    fn _emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _amount: Balance) {}

    fn _emit_approval_event(&self, _owner: AccountId, _spender: AccountId, _amount: Balance) {}

    fn _total_supply(&self) -> Balance {
        self.data().supply.get_or_default()
    }

    fn _balance_of(&self, owner: &AccountId) -> Balance {
        self.data().balances.get(owner).unwrap_or(0)
    }

    fn _allowance(&self, owner: &AccountId, spender: &AccountId) -> Balance {
        self.data().allowances.get(&(owner, spender)).unwrap_or(0)
    }

    fn _transfer_from_to(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: Balance,
        _data: Vec<u8>,
    ) -> Result<(), PSP22Error> {
        let from_balance = Internal::_balance_of(self, &from);

        if from_balance < amount {
            return Err(PSP22Error::InsufficientBalance)
        }

        Internal::_before_token_transfer(self, Some(&from), Some(&to), &amount)?;

        self.data().balances.insert(&from, &(from_balance - amount));

        let to_balance = Internal::_balance_of(self, &to);
        self.data().balances.insert(&to, &(to_balance + amount));

        Internal::_after_token_transfer(self, Some(&from), Some(&to), &amount)?;
        Internal::_emit_transfer_event(self, Some(from), Some(to), amount);

        Ok(())
    }

    fn _approve_from_to(&mut self, owner: AccountId, spender: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        self.data().allowances.insert(&(&owner, &spender), &amount);
        Internal::_emit_approval_event(self, owner, spender, amount);
        Ok(())
    }

    fn _mint_to(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        Internal::_before_token_transfer(self, None, Some(&account), &amount)?;
        let mut new_balance = Internal::_balance_of(self, &account);
        new_balance += amount;
        self.data().balances.insert(&account, &new_balance);

        let new_supply = self.data().supply.get_or_default() + amount;
        self.data().supply.set(&new_supply);

        Internal::_after_token_transfer(self, None, Some(&account), &amount)?;
        Internal::_emit_transfer_event(self, None, Some(account), amount);

        Ok(())
    }

    fn _burn_from(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        let mut from_balance = Internal::_balance_of(self, &account);

        if from_balance < amount {
            return Err(PSP22Error::InsufficientBalance)
        }

        Internal::_before_token_transfer(self, Some(&account), None, &amount)?;

        from_balance -= amount;
        self.data().balances.insert(&account, &from_balance);

        let new_supply = self.data().supply.get_or_default() - amount;
        self.data().supply.set(&new_supply);

        Internal::_after_token_transfer(self, Some(&account), None, &amount)?;
        Internal::_emit_transfer_event(self, Some(account), None, amount);

        Ok(())
    }

    fn _before_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _amount: &Balance,
    ) -> Result<(), PSP22Error> {
        Ok(())
    }

    fn _after_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _amount: &Balance,
    ) -> Result<(), PSP22Error> {
        Ok(())
    }
}
