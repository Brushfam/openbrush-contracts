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
    psp37,
    traits::psp37::*,
};
use core::result::Result;
use ink::prelude::{
    vec,
    vec::Vec,
};
use openbrush::{
    storage::{
        Mapping,
        TypeGuard,
    },
    traits::{
        AccountId,
        Balance,
        DefaultEnv,
        StorageAccess,
    },
    with_data,
};
pub use psp37::{
    BalancesManager as _,
    BalancesManagerImpl as _,
    Internal as _,
    InternalImpl as _,
};

#[cfg(feature = "upgradeable")]
use openbrush::storage::Lazy;

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::storage_item(STORAGE_KEY)]
pub struct Data {
    pub balances: Mapping<(AccountId, Option<Id>), Balance, BalancesKey>,
    pub supply: Mapping<Option<Id>, Balance, SupplyKey>,
    pub operator_approvals: Mapping<(AccountId, AccountId, Option<Id>), Balance, ApprovalsKey>,
    pub _reserved: Option<()>,
}

pub struct BalancesKey;

#[cfg(feature = "upgradeable")]
pub type DataType = Lazy<Data>;
#[cfg(not(feature = "upgradeable"))]
pub type DataType = Data;

impl<'a> TypeGuard<'a> for BalancesKey {
    type Type = &'a (&'a AccountId, &'a Option<&'a Id>);
}

pub struct SupplyKey;

impl<'a> TypeGuard<'a> for SupplyKey {
    type Type = &'a Option<&'a Id>;
}

pub struct ApprovalsKey;

impl<'a> TypeGuard<'a> for ApprovalsKey {
    type Type = &'a (&'a AccountId, &'a AccountId, &'a Option<&'a Id>);
}

pub trait PSP37Impl: Internal + StorageAccess<Data> + BalancesManager + Sized {
    fn balance_of(&self, owner: AccountId, id: Option<Id>) -> Balance {
        self._balance_of(&owner, &id.as_ref())
    }

    fn total_supply(&self, id: Option<Id>) -> Balance {
        self._total_supply(&id.as_ref())
    }

    fn allowance(&self, owner: AccountId, operator: AccountId, id: Option<Id>) -> Balance {
        match id {
            None => self._get_allowance(&owner, &operator, &None),
            Some(id) => self._get_allowance(&owner, &operator, &Some(&id)),
        }
    }

    fn approve(&mut self, operator: AccountId, id: Option<Id>, value: Balance) -> Result<(), PSP37Error> {
        self._approve_for(operator, id, value)
    }

    fn transfer(&mut self, to: AccountId, id: Id, value: Balance, data: Vec<u8>) -> Result<(), PSP37Error> {
        self._transfer_from(Self::env().caller(), to, id, value, data)
    }

    fn transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        id: Id,
        value: Balance,
        data: Vec<u8>,
    ) -> Result<(), PSP37Error> {
        self._transfer_from(from, to, id, value, data)
    }
}

pub trait Internal {
    /// Those methods must be implemented in derived implementation
    fn _emit_transfer_event(&self, from: Option<AccountId>, to: Option<AccountId>, id: Id, amount: Balance);

    fn _emit_transfer_batch_event(
        &self,
        from: Option<AccountId>,
        to: Option<AccountId>,
        ids_amounts: Vec<(Id, Balance)>,
    );

    fn _emit_approval_event(&self, _owner: AccountId, _operator: AccountId, _id: Option<Id>, value: Balance);

    /// Creates `amount` tokens of token type `id` to `to`.
    ///
    /// On success a `TransferSingle` event is emitted if length of `ids_amounts` is 1, otherwise `TransferBatch` event.
    ///
    /// # Errors
    ///
    /// Returns with `TransferToZeroAddress` error if `to` is zero account.
    fn _mint_to(&mut self, to: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP37Error>;

    /// Destroys `amount` tokens of token type `id` from `from`.
    ///
    /// On success a `TransferSingle` event is emitted if length of `ids_amounts` is 1, otherwise `TransferBatch` event.
    ///
    /// # Errors
    ///
    /// Returns with `NotAllowed` error if transfer is not approved.
    ///
    /// Returns with `InsufficientBalance` error if `from` doesn't contain enough balance.
    fn _burn_from(&mut self, from: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP37Error>;

    fn _transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        id: Id,
        amount: Balance,
        data: Vec<u8>,
    ) -> Result<(), PSP37Error>;

    fn _get_allowance(&self, account: &AccountId, operator: &AccountId, id: &Option<&Id>) -> Balance;

    fn _approve_for(&mut self, operator: AccountId, id: Option<Id>, value: Balance) -> Result<(), PSP37Error>;

    fn _decrease_allowance(
        &mut self,
        owner: &AccountId,
        operator: &AccountId,
        id: &Id,
        value: Balance,
    ) -> Result<(), PSP37Error>;

    fn _transfer_token(
        &mut self,
        from: &AccountId,
        to: &AccountId,
        id: Id,
        amount: Balance,
        data: &Vec<u8>,
    ) -> Result<(), PSP37Error>;

    fn _before_token_transfer(
        &mut self,
        from: Option<&AccountId>,
        to: Option<&AccountId>,
        ids: &Vec<(Id, Balance)>,
    ) -> Result<(), PSP37Error>;

    fn _after_token_transfer(
        &mut self,
        from: Option<&AccountId>,
        to: Option<&AccountId>,
        ids: &Vec<(Id, Balance)>,
    ) -> Result<(), PSP37Error>;
}

pub trait InternalImpl: Internal + StorageAccess<Data> + BalancesManager + Sized {
    fn _emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _id: Id, _amount: Balance) {}

    fn _emit_transfer_batch_event(
        &self,
        _from: Option<AccountId>,
        _to: Option<AccountId>,
        _ids_amounts: Vec<(Id, Balance)>,
    ) {
    }

    fn _emit_approval_event(&self, _owner: AccountId, _operator: AccountId, _id: Option<Id>, _value: Balance) {}

    fn _mint_to(&mut self, to: AccountId, mut ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP37Error> {
        if ids_amounts.is_empty() {
            return Ok(())
        }

        Internal::_before_token_transfer(self, None, Some(&to), &ids_amounts)?;

        for (id, amount) in &ids_amounts {
            self._increase_balance(&to, id, amount, true)?;
        }

        Internal::_after_token_transfer(self, None, Some(&to), &ids_amounts)?;

        if ids_amounts.len() == 1 {
            let (id, amount) = unsafe { ids_amounts.pop().unwrap_unchecked() };
            Internal::_emit_transfer_event(self, None, Some(to), id, amount);
        } else {
            Internal::_emit_transfer_batch_event(self, None, Some(to), ids_amounts);
        }

        Ok(())
    }

    fn _burn_from(&mut self, from: AccountId, mut ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP37Error> {
        Internal::_before_token_transfer(self, Some(&from), None, &ids_amounts)?;

        if ids_amounts.is_empty() {
            return Ok(())
        }

        for (id, amount) in ids_amounts.iter() {
            self._decrease_balance(&from, id, amount, true)?;
        }

        Internal::_after_token_transfer(self, Some(&from), None, &ids_amounts)?;

        if ids_amounts.len() == 1 {
            let (id, amount) = unsafe { ids_amounts.pop().unwrap_unchecked() };
            Internal::_emit_transfer_event(self, Some(from), None, id, amount);
        } else {
            Internal::_emit_transfer_batch_event(self, Some(from), None, ids_amounts);
        }

        Ok(())
    }

    fn _transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        id: Id,
        value: Balance,
        data: Vec<u8>,
    ) -> Result<(), PSP37Error> {
        let operator = Self::env().caller();
        let ids_amounts = vec![(id.clone(), value)];

        if from != operator && Internal::_get_allowance(self, &from, &operator, &Some(&id)) < value {
            return Err(PSP37Error::NotAllowed)
        }

        Internal::_before_token_transfer(self, Some(&from), Some(&to), &ids_amounts)?;
        Internal::_decrease_allowance(self, &from, &operator, &id, value)?;
        Internal::_transfer_token(self, &from, &to, id.clone(), value, &data)?;
        Internal::_after_token_transfer(self, Some(&from), Some(&to), &ids_amounts)?;
        Internal::_emit_transfer_event(self, Some(from), Some(to), id, value);
        Ok(())
    }

    fn _get_allowance(&self, owner: &AccountId, operator: &AccountId, id: &Option<&Id>) -> Balance {
        return match self.get_or_default().operator_approvals.get(&(owner, operator, &None)) {
            None => {
                self.get_or_default()
                    .operator_approvals
                    .get(&(owner, operator, id))
                    .unwrap_or(0)
            }
            _ => Balance::MAX,
        }
    }

    fn _approve_for(&mut self, operator: AccountId, id: Option<Id>, value: Balance) -> Result<(), PSP37Error> {
        let caller = Self::env().caller();

        if caller == operator {
            return Err(PSP37Error::SelfApprove)
        }

        if let Some(id) = &id {
            if value == 0 {
                with_data!(self, data, {
                    data.operator_approvals.remove(&(&caller, &operator, &Some(id)));
                });
            } else {
                with_data!(self, data, {
                    data.operator_approvals.insert(&(&caller, &operator, &Some(id)), &value);
                });
            }
        } else {
            if value == 0 {
                with_data!(self, data, {
                    data.operator_approvals.remove(&(&caller, &operator, &None));
                });
            } else {
                with_data!(self, data, {
                    data.operator_approvals
                        .insert(&(&caller, &operator, &None), &Balance::MAX);
                });
            }
        }

        Internal::_emit_approval_event(self, caller, operator, id, value);

        Ok(())
    }

    fn _decrease_allowance(
        &mut self,
        owner: &AccountId,
        operator: &AccountId,
        id: &Id,
        value: Balance,
    ) -> Result<(), PSP37Error> {
        if owner == operator {
            return Ok(())
        }

        let initial_allowance = Internal::_get_allowance(self, owner, operator, &Some(id));

        if initial_allowance == Balance::MAX {
            return Ok(())
        }

        if initial_allowance < value {
            return Err(PSP37Error::InsufficientBalance)
        }

        with_data!(self, data, {
            data.operator_approvals
                .insert(&(owner, operator, &Some(id)), &(initial_allowance - value));
        });

        Ok(())
    }

    fn _transfer_token(
        &mut self,
        from: &AccountId,
        to: &AccountId,
        id: Id,
        value: Balance,
        _data: &Vec<u8>,
    ) -> Result<(), PSP37Error> {
        self._decrease_balance(from, &id, &value, false)?;
        self._increase_balance(to, &id, &value, false)?;
        Ok(())
    }

    fn _before_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _ids: &Vec<(Id, Balance)>,
    ) -> Result<(), PSP37Error> {
        Ok(())
    }

    fn _after_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _ids: &Vec<(Id, Balance)>,
    ) -> Result<(), PSP37Error> {
        Ok(())
    }
}

pub trait BalancesManager {
    fn _balance_of(&self, owner: &AccountId, id: &Option<&Id>) -> Balance;

    fn _total_supply(&self, id: &Option<&Id>) -> Balance;

    fn _increase_balance(&mut self, owner: &AccountId, id: &Id, amount: &Balance, mint: bool)
        -> Result<(), PSP37Error>;

    fn _decrease_balance(&mut self, owner: &AccountId, id: &Id, amount: &Balance, burn: bool)
        -> Result<(), PSP37Error>;
}

pub trait BalancesManagerImpl: BalancesManager + StorageAccess<Data> + Sized {
    fn _balance_of(&self, owner: &AccountId, id: &Option<&Id>) -> Balance {
        self.get_or_default().balances.get(&(owner, id)).unwrap_or(0)
    }

    fn _total_supply(&self, id: &Option<&Id>) -> Balance {
        self.get_or_default().supply.get(id).unwrap_or(0)
    }

    fn _increase_balance(
        &mut self,
        owner: &AccountId,
        id: &Id,
        amount: &Balance,
        mint: bool,
    ) -> Result<(), PSP37Error> {
        let amount = *amount;

        if amount == 0 {
            return Ok(())
        }

        let id = &Some(id);
        let balance_before = BalancesManager::_balance_of(self, owner, id);

        if balance_before == 0 {
            let amount = &BalancesManager::_balance_of(self, owner, &None).checked_add(1).unwrap();
            with_data!(self, data, {
                data.balances.insert(&(owner, &None), amount);
            });
        }

        with_data!(self, data, {
            data.balances
                .insert(&(owner, id), &balance_before.checked_add(amount).unwrap());
        });

        if mint {
            let supply_before = BalancesManager::_total_supply(self, id);

            with_data!(self, data, {
                data.supply.insert(id, &supply_before.checked_add(amount).unwrap());
            });

            if supply_before == 0 {
                let amount = &BalancesManager::_total_supply(self, &None).checked_add(1).unwrap();
                with_data!(self, data, {
                    data.supply.insert(&None, amount);
                });
            }
        }

        Ok(())
    }

    fn _decrease_balance(
        &mut self,
        owner: &AccountId,
        id: &Id,
        amount: &Balance,
        burn: bool,
    ) -> Result<(), PSP37Error> {
        let amount = *amount;

        if amount == 0 {
            return Ok(())
        }

        let id = &Some(id);
        let balance_after = BalancesManager::_balance_of(self, owner, id)
            .checked_sub(amount)
            .ok_or(PSP37Error::InsufficientBalance)?;

        with_data!(self, data, {
            data.balances.insert(&(owner, id), &balance_after);
        });

        if balance_after == 0 {
            let amount = &BalancesManager::_balance_of(self, owner, &None)
                .checked_sub(1)
                .ok_or(PSP37Error::InsufficientBalance)?;
            with_data!(self, data, {
                data.balances.insert(&(owner, &None), amount);
            });
        }

        if burn {
            let supply_after = BalancesManager::_total_supply(self, id)
                .checked_sub(amount)
                .ok_or(PSP37Error::InsufficientBalance)?;
            with_data!(self, data, {
                data.supply.insert(id, &supply_after);
            });

            if supply_after == 0 {
                let amount = &BalancesManager::_total_supply(self, &None)
                    .checked_sub(1)
                    .ok_or(PSP37Error::InsufficientBalance)?;
                with_data!(self, data, {
                    data.supply.insert(&None, amount);
                });
            }
        }
        Ok(())
    }
}
