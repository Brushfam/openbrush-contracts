// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

use crate::psp34::ApprovalsKey;
pub use crate::{
    psp34,
    psp34::extensions::enumerable,
    traits::psp34::{
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
pub use psp34::{
    BalancesManager as _,
    Internal as _,
    InternalImpl as _,
    Operator,
    Owner,
    PSP34Impl,
};

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    pub token_owner: Mapping<Id, Owner>,
    pub operator_approvals: Mapping<(Owner, Operator, Option<Id>), (), ApprovalsKey>,
    pub balances: MultiMapping<Option<AccountId>, Id, EnumerableKey>,
}

pub struct EnumerableKey;

impl<'a> TypeGuard<'a> for EnumerableKey {
    type Type = &'a Option<&'a AccountId>;
}

pub trait BalancesManagerImpl: Storage<Data> {
    fn _balance_of(&self, owner: &Owner) -> u32 {
        self.data().balances.count(&Some(owner)) as u32
    }

    fn _increase_balance(&mut self, owner: &Owner, id: &Id, increase_supply: bool) {
        self.data().balances.insert(&Some(owner), id);
        if increase_supply {
            self.data().balances.insert(&None, id);
        }
    }

    fn _decrease_balance(&mut self, owner: &Owner, id: &Id, decrease_supply: bool) {
        self.data().balances.remove_value(&Some(owner), id);
        if decrease_supply {
            self.data().balances.remove_value(&None, id);
        }
    }

    fn _total_supply(&self) -> Balance {
        self.data().balances.count(&None)
    }

    fn _owner_of(&self, id: &Id) -> Option<AccountId> {
        self.data().token_owner.get(id)
    }

    fn _operator_approvals(&self, owner: &Owner, operator: &Operator, id: &Option<&Id>) -> Option<()> {
        self.data().operator_approvals.get(&(owner, operator, id))
    }

    fn _insert_operator_approvals(&mut self, owner: &Owner, operator: &Operator, id: &Option<&Id>) {
        self.data().operator_approvals.insert(&(owner, operator, id), &());
    }

    fn _remove_operator_approvals(&mut self, owner: &Owner, operator: &Operator, id: &Option<&Id>) {
        self.data().operator_approvals.remove(&(owner, operator, id));
    }

    fn _insert_token_owner(&mut self, id: &Id, to: &AccountId) {
        self.data().token_owner.insert(id, to);
    }

    fn _remove_token_owner(&mut self, id: &Id) {
        self.data().token_owner.remove(id);
    }
}

pub trait PSP34EnumerableImpl: Storage<Data> {
    fn owners_token_by_index(&self, owner: AccountId, index: u128) -> Result<Id, PSP34Error> {
        self.data()
            .balances
            .get_value(&Some(&owner), &index)
            .ok_or(PSP34Error::TokenNotExists)
    }

    fn token_by_index(&self, index: u128) -> Result<Id, PSP34Error> {
        self.data()
            .balances
            .get_value(&None, &index)
            .ok_or(PSP34Error::TokenNotExists)
    }
}
