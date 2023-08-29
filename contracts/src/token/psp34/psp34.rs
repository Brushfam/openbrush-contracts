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
    psp34,
    psp34::{Operator, Owner},
    traits::psp34::*,
};
use ink::prelude::vec::Vec;
use openbrush::{
    storage::{Mapping, TypeGuard},
    traits::{AccountId, Balance, DefaultEnv, Storage},
};
pub use psp34::{BalancesManager as _, Internal as _, InternalImpl as _, PSP34Impl as _};

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    pub token_owner: Mapping<Id, Owner>,
    pub operator_approvals: Mapping<(Owner, Operator, Option<Id>), (), ApprovalsKey>,
    pub owned_tokens_count: Mapping<Owner, u32>,
    pub total_supply: Balance,
}

pub struct ApprovalsKey;

impl<'a> TypeGuard<'a> for ApprovalsKey {
    type Type = &'a (&'a Owner, &'a Operator, &'a Option<&'a Id>);
}

pub trait PSP34Impl: Internal + PSP34 + BalancesManager + Sized {
    fn collection_id(&self) -> Id {
        let account_id = Self::env().account_id();
        Id::Bytes(<_ as AsRef<[u8; 32]>>::as_ref(&account_id).to_vec())
    }

    fn balance_of(&self, owner: AccountId) -> u32 {
        self._balance_of(&owner)
    }

    fn owner_of(&self, id: Id) -> Option<AccountId> {
        Internal::_owner_of(self, &id)
    }

    fn allowance(&self, owner: AccountId, operator: AccountId, id: Option<Id>) -> bool {
        self._allowance(&owner, &operator, &id.as_ref())
    }

    fn approve(&mut self, operator: AccountId, id: Option<Id>, approved: bool) -> Result<(), PSP34Error> {
        self._approve_for(operator, id, approved)
    }

    fn transfer(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), PSP34Error> {
        self._transfer_token(to, id, data)
    }

    fn total_supply(&self) -> Balance {
        self._total_supply()
    }
}

pub trait Internal {
    /// Those methods must be implemented in derived implementation
    fn _emit_transfer_event(&self, from: Option<AccountId>, to: Option<AccountId>, id: Id);

    fn _emit_approval_event(&self, from: AccountId, to: AccountId, id: Option<Id>, approved: bool);

    /// Approve the passed AccountId to transfer the specified token on behalf of the message's sender.
    fn _approve_for(&mut self, to: AccountId, id: Option<Id>, approved: bool) -> Result<(), PSP34Error>;

    /// Returns the owner of the token.
    fn _owner_of(&self, id: &Id) -> Option<AccountId>;

    /// Gets an operator on other Account's behalf.
    fn _transfer_token(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), PSP34Error>;

    fn _mint_to(&mut self, to: AccountId, id: Id) -> Result<(), PSP34Error>;

    fn _burn_from(&mut self, from: AccountId, id: Id) -> Result<(), PSP34Error>;

    fn _allowance(&self, owner: &Owner, operator: &Operator, id: &Option<&Id>) -> bool;

    fn _check_token_exists(&self, id: &Id) -> Result<AccountId, PSP34Error>;

    fn _before_token_transfer(
        &mut self,
        from: Option<&AccountId>,
        to: Option<&AccountId>,
        id: &Id,
    ) -> Result<(), PSP34Error>;

    fn _after_token_transfer(
        &mut self,
        from: Option<&AccountId>,
        to: Option<&AccountId>,
        id: &Id,
    ) -> Result<(), PSP34Error>;
}

pub trait InternalImpl: Internal + BalancesManager + Sized {
    fn _emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _id: Id) {}

    fn _emit_approval_event(&self, _from: AccountId, _to: AccountId, _id: Option<Id>, _approved: bool) {}

    fn _approve_for(&mut self, to: AccountId, id: Option<Id>, approved: bool) -> Result<(), PSP34Error> {
        let mut caller = Self::env().caller();

        if let Some(id) = &id {
            let owner = Internal::_owner_of(self, id).ok_or(PSP34Error::TokenNotExists)?;

            if approved && owner == to {
                return Err(PSP34Error::SelfApprove);
            }

            if owner != caller && !Internal::_allowance(self, &owner, &caller, &None) {
                return Err(PSP34Error::NotApproved);
            };
            caller = owner;
        }

        if approved {
            self._insert_operator_approvals(&caller, &to, &id.as_ref());
        } else {
            self._remove_operator_approvals(&caller, &to, &id.as_ref());
        }
        Internal::_emit_approval_event(self, caller, to, id, approved);

        Ok(())
    }

    fn _owner_of(&self, id: &Id) -> Option<AccountId> {
        BalancesManager::_owner_of(self, id)
    }

    fn _transfer_token(&mut self, to: AccountId, id: Id, _data: Vec<u8>) -> Result<(), PSP34Error> {
        let owner = Internal::_check_token_exists(self, &id)?;
        let caller = Self::env().caller();

        if owner != caller && !Internal::_allowance(self, &owner, &caller, &Some(&id)) {
            return Err(PSP34Error::NotApproved);
        }

        Internal::_before_token_transfer(self, Some(&owner), Some(&to), &id)?;

        self._remove_operator_approvals(&owner, &caller, &Some(&id));
        BalancesManager::_decrease_balance(self, &owner, &id, false);
        self._remove_token_owner(&id);

        BalancesManager::_increase_balance(self, &to, &id, false);
        self._insert_token_owner(&id, &to);
        Internal::_after_token_transfer(self, Some(&owner), Some(&to), &id)?;
        Internal::_emit_transfer_event(self, Some(owner), Some(to), id);

        Ok(())
    }

    fn _mint_to(&mut self, to: AccountId, id: Id) -> Result<(), PSP34Error> {
        if Internal::_owner_of(self, &id).is_some() {
            return Err(PSP34Error::TokenExists);
        }
        Internal::_before_token_transfer(self, None, Some(&to), &id)?;

        BalancesManager::_increase_balance(self, &to, &id, true);
        self._insert_token_owner(&id, &to);
        Internal::_after_token_transfer(self, None, Some(&to), &id)?;
        Internal::_emit_transfer_event(self, None, Some(to), id);

        Ok(())
    }

    fn _burn_from(&mut self, from: AccountId, id: Id) -> Result<(), PSP34Error> {
        Internal::_check_token_exists(self, &id)?;

        Internal::_before_token_transfer(self, Some(&from), None, &id)?;

        self._remove_token_owner(&id);
        BalancesManager::_decrease_balance(self, &from, &id, true);
        Internal::_after_token_transfer(self, Some(&from), None, &id)?;
        Internal::_emit_transfer_event(self, Some(from), None, id);
        Ok(())
    }

    fn _allowance(&self, owner: &Owner, operator: &Operator, id: &Option<&Id>) -> bool {
        self._operator_approvals(owner, operator, &None).is_some()
            || id.is_some() && self._operator_approvals(owner, operator, id).is_some()
    }

    fn _check_token_exists(&self, id: &Id) -> Result<AccountId, PSP34Error> {
        Internal::_owner_of(self, id).ok_or(PSP34Error::TokenNotExists)
    }

    fn _before_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _id: &Id,
    ) -> Result<(), PSP34Error> {
        Ok(())
    }

    fn _after_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _id: &Id,
    ) -> Result<(), PSP34Error> {
        Ok(())
    }
}

pub trait BalancesManager {
    fn _balance_of(&self, owner: &Owner) -> u32;

    fn _increase_balance(&mut self, owner: &Owner, id: &Id, increase_supply: bool);

    fn _decrease_balance(&mut self, owner: &Owner, id: &Id, decrease_supply: bool);

    fn _total_supply(&self) -> u128;

    fn _owner_of(&self, id: &Id) -> Option<AccountId>;

    fn _operator_approvals(&self, owner: &Owner, operator: &Operator, id: &Option<&Id>) -> Option<()>;

    fn _insert_operator_approvals(&mut self, owner: &Owner, operator: &Operator, id: &Option<&Id>);

    fn _remove_operator_approvals(&mut self, owner: &Owner, operator: &Operator, id: &Option<&Id>);

    fn _insert_token_owner(&mut self, id: &Id, to: &AccountId);

    fn _remove_token_owner(&mut self, id: &Id);
}

pub trait BalancesManagerImpl: BalancesManager + Storage<Data> {
    fn _balance_of(&self, owner: &Owner) -> u32 {
        self.data().owned_tokens_count.get(owner).unwrap_or(0)
    }

    fn _increase_balance(&mut self, owner: &Owner, _id: &Id, increase_supply: bool) {
        let to_balance = self.data().owned_tokens_count.get(owner).unwrap_or(0);
        self.data().owned_tokens_count.insert(owner, &(to_balance + 1));
        if increase_supply {
            self.data().total_supply = self.data().total_supply + 1;
        }
    }

    fn _decrease_balance(&mut self, owner: &Owner, _id: &Id, decrease_supply: bool) {
        let from_balance = self.data().owned_tokens_count.get(owner).unwrap_or(0);
        self.data()
            .owned_tokens_count
            .insert(owner, &(from_balance.checked_sub(1).unwrap()));

        if decrease_supply {
            self.data().total_supply = self.data().total_supply - 1;
        }
    }

    fn _total_supply(&self) -> u128 {
        self.data().total_supply
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
