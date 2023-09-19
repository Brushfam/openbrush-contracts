// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::{
    psp34,
    traits::psp34::{
        extensions::mintable::*,
        *,
    },
};
use openbrush::traits::AccountId;
pub use psp34::{
    BalancesManager as _,
    Internal as _,
    InternalImpl as _,
    Operator,
    Owner,
    PSP34Impl,
};

pub trait PSP34MintableImpl: psp34::Internal {
    fn mint(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error> {
        self._mint_to(account, id)
    }
}
