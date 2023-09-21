// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::{
    psp22,
    traits::psp22::{
        extensions::mintable::*,
        *,
    },
};
use openbrush::traits::{
    AccountId,
    Balance,
};
pub use psp22::{
    Internal as _,
    InternalImpl as _,
    PSP22Impl,
};

pub trait PSP22MintableImpl: psp22::Internal {
    fn mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        self._mint_to(account, amount)
    }
}
