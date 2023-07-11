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
    payment_splitter,
    traits::payment_splitter::*,
};
use ink::prelude::vec::Vec;
use openbrush::{
    storage::Mapping,
    traits::{
        AccountId,
        Balance,
        DefaultEnv,
        Storage,
        StorageAccess,
    },
    with_data,
};
pub use payment_splitter::Internal as _;

#[cfg(feature = "upgradeable")]
use openbrush::storage::Lazy;

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::storage_item(STORAGE_KEY)]
pub struct Data {
    pub total_shares: Balance,
    pub total_released: Balance,
    pub shares: Mapping<AccountId, Balance>,
    pub released: Mapping<AccountId, Balance>,
    pub payees: Vec<AccountId>,
    pub _reserved: Option<()>,
}

#[cfg(feature = "upgradeable")]
pub type DataType = Lazy<Data>;
#[cfg(not(feature = "upgradeable"))]
pub type DataType = Data;

pub trait PaymentSplitterImpl: StorageAccess<Data> + Internal + Sized {
    fn total_shares(&self) -> Balance {
        self.get_or_default().total_shares.clone()
    }

    fn total_released(&self) -> Balance {
        self.get_or_default().total_released.clone()
    }

    fn shares(&self, account: AccountId) -> Balance {
        self.get_or_default().shares.get(&account).unwrap_or(0)
    }

    fn released(&self, account: AccountId) -> Balance {
        self.get_or_default().released.get(&account).unwrap_or(0)
    }

    fn payee(&self, index: u32) -> Option<AccountId> {
        self.get_or_default().payees.get(index as usize).cloned()
    }

    fn receive(&mut self) {
        self._emit_payee_added_event(Self::env().caller(), Self::env().transferred_value())
    }

    fn release(&mut self, account: AccountId) -> Result<(), PaymentSplitterError> {
        self._release(account)
    }
}

pub trait Internal {
    /// User must override those methods in their contract.
    fn _emit_payee_added_event(&self, account: AccountId, shares: Balance);

    fn _emit_payment_received_event(&self, from: AccountId, amount: Balance);

    fn _emit_payment_released_event(&self, to: AccountId, amount: Balance);

    /// Inits an instance of `PaymentSplitter` where each account in `payees` is assigned the number of shares at
    /// the matching position in the `shares` array.
    ///
    /// All addresses in `payees` must be non-zero. Both arrays must have the same non-zero length, and there must be no
    /// duplicates in `payees`.
    ///
    /// Emits `PayeeAdded` on each account.
    fn _init(&mut self, payees_and_shares: Vec<(AccountId, Balance)>) -> Result<(), PaymentSplitterError>;

    fn _add_payee(&mut self, payee: AccountId, share: Balance) -> Result<(), PaymentSplitterError>;

    /// Calls the `release` method for each `AccountId` in the `payees` vec.
    fn _release_all(&mut self) -> Result<(), PaymentSplitterError>;

    fn _release(&mut self, account: AccountId) -> Result<(), PaymentSplitterError>;
}

pub trait InternalImpl: StorageAccess<Data> + Internal + Sized {
    fn _emit_payee_added_event(&self, _account: AccountId, _shares: Balance) {}

    fn _emit_payment_received_event(&self, _from: AccountId, _amount: Balance) {}

    fn _emit_payment_released_event(&self, _to: AccountId, _amount: Balance) {}

    fn _init(&mut self, payees_and_shares: Vec<(AccountId, Balance)>) -> Result<(), PaymentSplitterError> {
        if payees_and_shares.is_empty() {
            return Err(PaymentSplitterError::NoPayees)
        }

        for (payee, share) in payees_and_shares.into_iter() {
            Internal::_add_payee(self, payee, share)?;
        }
        Ok(())
    }

    fn _add_payee(&mut self, payee: AccountId, share: Balance) -> Result<(), PaymentSplitterError> {
        if share == 0 {
            return Err(PaymentSplitterError::SharesAreZero)
        }
        if self.get_or_default().shares.get(&payee).is_some() {
            return Err(PaymentSplitterError::AlreadyHasShares)
        }

        with_data!(self, data, {
            data.payees.push(payee.clone());
            data.shares.insert(&payee, &share);
            data.total_shares += share;
        });

        Internal::_emit_payee_added_event(self, payee, share);
        Ok(())
    }

    fn _release_all(&mut self) -> Result<(), PaymentSplitterError> {
        let len = self.get_or_default().payees.len();

        for i in 0..len {
            let account = self.get_or_default().payees[i];
            Internal::_release(self, account)?;
        }

        Ok(())
    }

    fn _release(&mut self, account: AccountId) -> Result<(), PaymentSplitterError> {
        if !self.get_or_default().shares.get(&account).is_some() {
            return Err(PaymentSplitterError::AccountHasNoShares)
        }

        let balance = Self::env().balance();
        let current_balance = balance.checked_sub(Self::env().minimum_balance()).unwrap_or_default();
        let total_received = current_balance + self.get_or_default().total_released;
        let shares = self.get_or_default().shares.get(&account).unwrap().clone();
        let total_shares = self.get_or_default().total_shares;
        let released = self.get_or_default().released.get(&account).unwrap_or_default();
        let payment = total_received * shares / total_shares - released;

        if payment == 0 {
            return Err(PaymentSplitterError::AccountIsNotDuePayment)
        }

        with_data!(self, data, {
            data.released.insert(&account, &(released + payment));
            data.total_released += payment;
        });

        let transfer_result = Self::env().transfer(account.clone(), payment);
        if transfer_result.is_err() {
            return Err(PaymentSplitterError::TransferFailed)
        }
        Internal::_emit_payment_released_event(self, account, payment);
        Ok(())
    }
}
