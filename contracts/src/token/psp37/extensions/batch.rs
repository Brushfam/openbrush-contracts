// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::{
    psp37,
    psp37::extensions::batch,
    traits::psp37::{
        extensions::batch::*,
        *,
    },
};
pub use batch::Internal as _;
use ink::prelude::vec::Vec;
use openbrush::traits::{
    AccountId,
    Balance,
    Storage,
};
pub use psp37::{
    BalancesManager as _,
    BalancesManagerImpl as _,
    Internal as _,
    InternalImpl as _,
    PSP37Impl,
};

pub trait PSP37BatchImpl: Internal + Storage<psp37::Data> {
    fn batch_transfer(
        &mut self,
        to: AccountId,
        ids_amounts: Vec<(Id, Balance)>,
        data: Vec<u8>,
    ) -> Result<(), PSP37Error> {
        self._batch_transfer_from(Self::env().caller(), to, ids_amounts, data)
    }

    fn batch_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        ids_amounts: Vec<(Id, Balance)>,
        data: Vec<u8>,
    ) -> Result<(), PSP37Error> {
        self._batch_transfer_from(from, to, ids_amounts, data)
    }
}

pub trait Internal {
    fn _batch_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        ids_amounts: Vec<(Id, Balance)>,
        data: Vec<u8>,
    ) -> Result<(), PSP37Error>;
}

pub trait InternalImpl: Internal + psp37::Internal + Storage<psp37::Data> + psp37::BalancesManager {
    fn _batch_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        ids_amounts: Vec<(Id, Balance)>,
        _: Vec<u8>,
    ) -> Result<(), PSP37Error> {
        let operator = Self::env().caller();

        for (id, value) in &ids_amounts {
            if from != operator && &self._get_allowance(&from, &operator, &Some(id)) < value {
                return Err(PSP37Error::NotAllowed)
            }
        }

        self._before_token_transfer(Some(&from), Some(&to), &ids_amounts)?;

        for (id, value) in &ids_amounts {
            self._decrease_allowance(&from, &operator, id, *value)?;

            self._decrease_balance(&from, id, value, false)?;
        }

        for (id, value) in &ids_amounts {
            self._increase_balance(&to, id, value, false)?;
        }

        self._after_token_transfer(Some(&from), Some(&to), &ids_amounts)?;

        self._emit_transfer_batch_event(Some(from), Some(to), ids_amounts);

        Ok(())
    }
}
