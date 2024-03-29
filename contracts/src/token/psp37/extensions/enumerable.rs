// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

use crate::psp37::{
    ApprovalsKey,
    BalancesManager,
};
pub use crate::{
    psp37,
    psp37::extensions::enumerable,
    traits::psp37::{
        extensions::enumerable::*,
        *,
    },
};
use openbrush::{
    storage::{
        Mapping,
        MultiMapping,
        TypeGuard,
    },
    traits::{
        AccountId,
        Balance,
        Storage,
    },
};
pub use psp37::{
    BalancesManager as _,
    Internal as _,
    InternalImpl as _,
    PSP37Impl,
};

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    pub enumerable: MultiMapping<Option<AccountId>, Id, EnumerableKey>,
    pub balances: Mapping<(AccountId, Id), Balance, BalancesKey>,
    pub supply: Mapping<Id, Balance>,
    pub operator_approvals: Mapping<(AccountId, AccountId, Option<Id>), Balance, ApprovalsKey>,
}

pub struct EnumerableKey;

impl<'a> TypeGuard<'a> for EnumerableKey {
    type Type = &'a Option<&'a AccountId>;
}

pub struct BalancesKey;

impl<'a> TypeGuard<'a> for BalancesKey {
    type Type = &'a (&'a AccountId, &'a Id);
}

pub trait BalancesManagerImpl: Storage<Data> + psp37::BalancesManager {
    fn _balance_of(&self, owner: &AccountId, id: &Option<&Id>) -> Balance {
        match id {
            None => self.data().enumerable.count(&Some(owner)),
            Some(id) => self.data().balances.get(&(owner, id)).unwrap_or(0),
        }
    }

    #[inline(always)]
    fn _total_supply(&self, id: &Option<&Id>) -> Balance {
        match id {
            None => self.data().enumerable.count(&None),
            Some(id) => self.data().supply.get(id).unwrap_or(0),
        }
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

        let balance_before = BalancesManager::_balance_of(self, owner, &Some(id));
        self.data()
            .balances
            .insert(&(owner, id), &(balance_before.checked_add(amount).unwrap()));

        if balance_before == 0 {
            self.data().enumerable.insert(&Some(owner), id);
        }

        if mint {
            let supply_before = BalancesManager::_total_supply(self, &Some(id));

            self.data()
                .supply
                .insert(id, &(supply_before.checked_add(amount).unwrap()));

            if supply_before == 0 {
                self.data().enumerable.insert(&None, id);
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

        let balance_after = BalancesManager::_balance_of(self, owner, &Some(id))
            .checked_sub(amount)
            .ok_or(PSP37Error::InsufficientBalance)?;
        self.data().balances.insert(&(owner, id), &balance_after);

        if balance_after == 0 {
            self.data().enumerable.remove_value(&Some(owner), id);
        }

        if burn {
            let supply_after = BalancesManager::_total_supply(self, &Some(id))
                .checked_sub(amount)
                .ok_or(PSP37Error::InsufficientBalance)?;
            self.data().supply.insert(id, &supply_after);

            if supply_after == 0 {
                self.data().enumerable.remove_value(&None, id);
            }
        }
        Ok(())
    }

    fn _insert_operator_approvals(
        &mut self,
        owner: &AccountId,
        operator: &AccountId,
        id: &Option<&Id>,
        amount: &Balance,
    ) {
        self.data().operator_approvals.insert(&(owner, operator, id), amount);
    }

    fn _get_operator_approvals(&self, owner: &AccountId, operator: &AccountId, id: &Option<&Id>) -> Option<Balance> {
        self.data().operator_approvals.get(&(owner, operator, id))
    }

    fn _remove_operator_approvals(&self, owner: &AccountId, operator: &AccountId, id: &Option<&Id>) {
        self.data().operator_approvals.remove(&(owner, operator, id));
    }
}

pub trait PSP37EnumerableImpl: Storage<Data> {
    fn owners_token_by_index(&self, owner: AccountId, index: u128) -> Option<Id> {
        self.data().enumerable.get_value(&Some(&owner), &index)
    }

    fn token_by_index(&self, index: u128) -> Option<Id> {
        self.data().enumerable.get_value(&None, &index)
    }
}
