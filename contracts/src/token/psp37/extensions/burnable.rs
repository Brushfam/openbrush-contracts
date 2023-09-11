// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::{
    psp37,
    traits::psp37::{
        extensions::burnable::*,
        *,
    },
};
use ink::prelude::vec::Vec;
use openbrush::traits::{
    AccountId,
    Balance,
};
pub use psp37::{
    BalancesManager as _,
    BalancesManagerImpl as _,
    Internal as _,
    InternalImpl as _,
    PSP37Impl,
};

pub trait PSP37BurnableImpl: psp37::Internal {
    fn burn(&mut self, from: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP37Error> {
        self._burn_from(from, ids_amounts)
    }
}
