// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::{
    psp22,
    psp22::extensions::wrapper,
    traits::psp22::{
        extensions::wrapper::*,
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
    String,
};
pub use psp22::{
    Internal as _,
    InternalImpl as _,
    PSP22Impl,
};
pub use wrapper::Internal as _;

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    #[lazy]
    pub underlying: Option<AccountId>,
}

pub trait PSP22WrapperImpl: Storage<Data> + Internal + psp22::Internal {
    fn deposit_for(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        if Some(account) == self.data().underlying.get_or_default() {
            return Err(PSP22Error::Custom(String::from("Cannot deposit to underlying")))
        }

        self._deposit(amount)?;
        psp22::Internal::_mint_to(self, account, amount)
    }

    fn withdraw_to(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        if Some(account) == self.data().underlying.get_or_default() {
            return Err(PSP22Error::Custom(String::from("Cannot withdraw to underlying")))
        }

        psp22::Internal::_burn_from(self, Self::env().caller(), amount)?;
        self._withdraw(account, amount)
    }
}

pub trait Internal {
    /// Mint wrapped token to cover any underlyingTokens that would have been transfered by mistake. Internal
    /// function that can be exposed with access control if desired.
    fn _recover(&mut self, account: AccountId) -> Result<Balance, PSP22Error>;

    /// helper function to transfer the underlying token from caller to the contract
    fn _deposit(&mut self, amount: Balance) -> Result<(), PSP22Error>;

    /// helper function to transfer the underlying token
    fn _withdraw(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error>;

    /// helper function to get balance of underlying tokens in the contract
    fn _underlying_balance(&mut self) -> Balance;

    /// Initalize the wrapper token with defining the underlying PSP22 token
    ///
    /// `underlying` is the token to be wrapped
    fn _init(&mut self, underlying: AccountId);

    /// Getter for caller to `PSP22Wrapper` of `underlying`
    fn _underlying(&mut self) -> Option<AccountId>;
}

pub trait InternalImpl: Storage<Data> + Internal + psp22::Internal + PSP22 {
    fn _recover(&mut self, account: AccountId) -> Result<Balance, PSP22Error> {
        let value = Internal::_underlying_balance(self) - self.total_supply();
        psp22::Internal::_mint_to(self, account, value)?;
        Ok(value)
    }

    fn _deposit(&mut self, amount: Balance) -> Result<(), PSP22Error> {
        if let Some(underlying) = Internal::_underlying(self) {
            PSP22Ref::transfer_from_builder(
                &underlying,
                Self::env().caller(),
                Self::env().account_id(),
                amount,
                Vec::<u8>::new(),
            )
            .call_flags(CallFlags::default().set_allow_reentry(true))
            .try_invoke()
            .unwrap()
            .unwrap()
        } else {
            Err(PSP22Error::Custom(String::from("Underlying not initialized")))
        }
    }

    fn _withdraw(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        if let Some(underlying) = Internal::_underlying(self) {
            PSP22Ref::transfer_builder(&underlying, account, amount, Vec::<u8>::new())
                .call_flags(CallFlags::default().set_allow_reentry(true))
                .try_invoke()
                .unwrap()
                .unwrap()
        } else {
            Err(PSP22Error::Custom(String::from("Underlying not initialized")))
        }
    }

    fn _underlying_balance(&mut self) -> Balance {
        if let Some(underlying) = Internal::_underlying(self) {
            PSP22Ref::balance_of(&underlying, Self::env().account_id())
        } else {
            0
        }
    }

    fn _init(&mut self, underlying: AccountId) {
        self.data().underlying.set(&Some(underlying));
    }

    fn _underlying(&mut self) -> Option<AccountId> {
        self.data().underlying.get_or_default()
    }
}
