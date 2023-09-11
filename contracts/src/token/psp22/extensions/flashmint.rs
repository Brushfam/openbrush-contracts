// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::{
    psp22,
    psp22::extensions::flashmint,
    traits::{
        flashloan::*,
        psp22::*,
    },
};
pub use flashmint::Internal as _;
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

pub trait FlashLenderImpl: Storage<psp22::Data> + psp22::Internal + PSP22 + Internal {
    fn max_flashloan(&mut self, token: AccountId) -> Balance {
        if token == Self::env().account_id() {
            Balance::MAX - self.total_supply()
        } else {
            0
        }
    }

    fn flash_fee(&self, token: AccountId, amount: Balance) -> Result<Balance, FlashLenderError> {
        if token != Self::env().account_id() {
            return Err(FlashLenderError::WrongTokenAddress)
        }
        Ok(self._get_fee(amount))
    }

    fn flashloan(
        &mut self,
        receiver_account: AccountId,
        token: AccountId,
        amount: Balance,
        data: Vec<u8>,
    ) -> Result<(), FlashLenderError> {
        let fee = self.flash_fee(token, amount)?;
        self._mint_to(receiver_account, amount)?;
        Internal::_on_flashloan(self, receiver_account, token, fee, amount, data)?;
        let this = Self::env().account_id();
        let current_allowance = self.allowance(receiver_account, this);
        if current_allowance < amount + fee {
            return Err(FlashLenderError::AllowanceDoesNotAllowRefund)
        }
        psp22::Internal::_approve_from_to(self, receiver_account, this, current_allowance - amount - fee)?;
        psp22::Internal::_burn_from(self, receiver_account, amount + fee)?;
        Ok(())
    }
}

pub trait Internal {
    fn _get_fee(&self, _amount: Balance) -> Balance;

    fn _on_flashloan(
        &mut self,
        receiver_account: AccountId,
        token: AccountId,
        fee: Balance,
        amount: Balance,
        data: Vec<u8>,
    ) -> Result<(), FlashLenderError>;
}

pub trait InternalImpl: Storage<psp22::Data> + Internal {
    fn _get_fee(&self, _amount: Balance) -> Balance {
        0
    }

    fn _on_flashloan(
        &mut self,
        receiver_account: AccountId,
        token: AccountId,
        fee: Balance,
        amount: Balance,
        data: Vec<u8>,
    ) -> Result<(), FlashLenderError> {
        let builder =
            FlashBorrowerRef::on_flashloan_builder(&receiver_account, Self::env().caller(), token, amount, fee, data)
                .call_flags(CallFlags::default().set_allow_reentry(true));
        let result = match builder.try_invoke() {
            Ok(Ok(Ok(_))) => Ok(()),
            Ok(Ok(Err(FlashBorrowerError::FlashloanRejected(message)))) => {
                Err(FlashLenderError::BorrowerRejected(message))
            }
            // Means unknown method
            Ok(Err(ink::LangError::CouldNotReadInput)) => Ok(()),
            // `NotCallable` means that the receiver is not a contract.
            Err(ink::env::Error::NotCallable) => Ok(()),
            _ => {
                Err(FlashLenderError::BorrowerRejected(String::from(
                    "Error while performing the `on_flashloan`",
                )))
            }
        };

        result
    }
}
