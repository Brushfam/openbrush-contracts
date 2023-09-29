// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::{
    psp22,
    psp22::extensions::capped,
    traits::psp22::{
        extensions::capped::*,
        *,
    },
};
pub use capped::Internal as _;
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

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    #[lazy]
    pub cap: Balance,
}

pub trait PSP22CappedImpl: Internal {
    fn cap(&self) -> Balance {
        self._cap()
    }
}

pub trait Internal {
    /// Initializes the token's cap
    fn _init_cap(&mut self, cap: Balance) -> Result<(), PSP22Error>;

    fn _is_cap_exceeded(&self, amount: &Balance) -> bool;

    fn _cap(&self) -> Balance;
}

pub trait InternalImpl: Storage<Data> + Internal + PSP22 {
    fn _init_cap(&mut self, cap: Balance) -> Result<(), PSP22Error> {
        if cap == 0 {
            return Err(PSP22Error::Custom(String::from("Cap must be above 0")))
        }
        self.data().cap.set(&cap);
        Ok(())
    }

    fn _is_cap_exceeded(&self, amount: &Balance) -> bool {
        if self.total_supply() + amount > Internal::_cap(self) {
            return true
        }
        false
    }

    fn _cap(&self) -> Balance {
        self.data().cap.get_or_default()
    }
}

pub trait PSP22TransferImpl: Internal {
    fn _before_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _amount: &Balance,
    ) -> Result<(), PSP22Error> {
        if _from.is_none() && _to.is_some() && Internal::_is_cap_exceeded(self, _amount) {
            return Err(PSP22Error::Custom(String::from("Cap exceeded")))
        }

        Ok(())
    }

    fn _after_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _amount: &Balance,
    ) -> Result<(), PSP22Error> {
        Ok(())
    }
}
