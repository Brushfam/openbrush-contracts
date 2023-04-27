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

/// Extension of [`PSP22`] which allows the beneficiary to extract tokens after given time
pub use crate::{
    psp22,
    psp22::utils::token_timelock,
    traits::psp22::{
        utils::token_timelock::*,
        *,
    },
};
use ink::{
    env::CallFlags,
    prelude::vec::Vec,
};
use openbrush::traits::{
    AccountId,
    Balance,
    Storage,
    Timestamp,
};
pub use psp22::Internal as _;
pub use token_timelock::Internal as _;

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Debug, Default)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    token: Option<AccountId>,
    beneficiary: Option<AccountId>,
    release_time: Timestamp,
}

impl<T: Storage<Data>> PSP22TokenTimelock for T {
    /// Returns the token address
    default fn token(&self) -> Option<AccountId> {
        self.data().token
    }

    /// Returns the beneficiary of the tokens
    default fn beneficiary(&self) -> Option<AccountId> {
        self.data().beneficiary
    }

    /// Returns the timestamp when the tokens are released
    default fn release_time(&self) -> Timestamp {
        self.data().release_time
    }

    /// Transfers the tokens held by timelock to the beneficairy
    default fn release(&mut self) -> Result<(), PSP22TokenTimelockError> {
        if Self::env().block_timestamp() < self.data().release_time {
            return Err(PSP22TokenTimelockError::CurrentTimeIsBeforeReleaseTime)
        }
        let amount = self._contract_balance();
        if amount == 0 {
            return Err(PSP22TokenTimelockError::NoTokensToRelease)
        }
        self._withdraw(amount)
    }
}

pub trait Internal {
    /// Helper function to withdraw tokens
    fn _withdraw(&mut self, amount: Balance) -> Result<(), PSP22TokenTimelockError>;

    /// Helper function to return balance of the contract
    fn _contract_balance(&mut self) -> Balance;

    /// Initializes the contract
    fn _init(
        &mut self,
        token: AccountId,
        beneficiary: AccountId,
        release_time: Timestamp,
    ) -> Result<(), PSP22TokenTimelockError>;
}

impl<T: Storage<Data>> Internal for T {
    default fn _withdraw(&mut self, amount: Balance) -> Result<(), PSP22TokenTimelockError> {
        if let Some(beneficiary) = self.beneficiary() {
            if let Some(token) = self.data().token {
                PSP22Ref::transfer_builder(&token, beneficiary, amount, Vec::<u8>::new())
                    .call_flags(CallFlags::default().set_allow_reentry(true))
                    .try_invoke()
                    .unwrap()
                    .unwrap()?;
                return Ok(())
            }
        }
        Err(PSP22TokenTimelockError::NonExistingAccount)
    }

    default fn _contract_balance(&mut self) -> Balance {
        match self.data().token {
            Some(token) => PSP22Ref::balance_of(&token, Self::env().account_id()),
            None => 0,
        }
    }

    default fn _init(
        &mut self,
        token: AccountId,
        beneficiary: AccountId,
        release_time: Timestamp,
    ) -> Result<(), PSP22TokenTimelockError> {
        if release_time <= Self::env().block_timestamp() {
            return Err(PSP22TokenTimelockError::ReleaseTimeIsBeforeCurrentTime)
        }
        self.data().token = Some(token);
        self.data().beneficiary = Some(beneficiary);
        self.data().release_time = release_time;
        Ok(())
    }
}
