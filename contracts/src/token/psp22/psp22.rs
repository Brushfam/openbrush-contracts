// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

pub use crate::{
    psp22,
    psp22::Internal as _,
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
        AccountIdExt,
        Balance,
        Storage,
    },
};
pub use psp22::Internal as _;

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[openbrush::upgradeable_storage(STORAGE_KEY)]
#[derive(Default, Debug)]
pub struct Data {
    pub supply: Balance,
    pub balances: Mapping<AccountId, Balance>,
    pub allowances: Mapping<(AccountId, AccountId), Balance, AllowancesKey>,
    pub _reserved: Option<()>,
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
        self.data().supply.clone()
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
        if from.is_zero() {
            return Err(PSP22Error::ZeroSenderAddress)
        }
        if to.is_zero() {
            return Err(PSP22Error::ZeroRecipientAddress)
        }

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
        if owner.is_zero() {
            return Err(PSP22Error::ZeroSenderAddress)
        }
        if spender.is_zero() {
            return Err(PSP22Error::ZeroRecipientAddress)
        }

        self.data().allowances.insert(&(&owner, &spender), &amount);
        Internal::_emit_approval_event(self, owner, spender, amount);
        Ok(())
    }

    fn _mint_to(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        if account.is_zero() {
            return Err(PSP22Error::ZeroRecipientAddress)
        }

        Internal::_before_token_transfer(self, None, Some(&account), &amount)?;
        let mut new_balance = Internal::_balance_of(self, &account);
        new_balance += amount;
        self.data().balances.insert(&account, &new_balance);
        self.data().supply += amount;
        Internal::_after_token_transfer(self, None, Some(&account), &amount)?;
        Internal::_emit_transfer_event(self, None, Some(account), amount);

        Ok(())
    }

    fn _burn_from(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        if account.is_zero() {
            return Err(PSP22Error::ZeroRecipientAddress)
        }

        let mut from_balance = Internal::_balance_of(self, &account);

        if from_balance < amount {
            return Err(PSP22Error::InsufficientBalance)
        }

        Internal::_before_token_transfer(self, Some(&account), None, &amount)?;

        from_balance -= amount;
        self.data().balances.insert(&account, &from_balance);
        self.data().supply -= amount;
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
