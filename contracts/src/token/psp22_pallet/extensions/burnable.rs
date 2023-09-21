// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::{
    psp22_pallet,
    traits::psp22::{
        extensions::burnable::*,
        *,
    },
};
pub use ink::env::DefaultEnvironment;
use openbrush::traits::{
    AccountId,
    Balance,
    Storage,
};
pub use pallet_assets_chain_extension::traits::{
    Error,
    Origin,
};
pub use psp22_pallet::{
    Internal as _,
    InternalImpl as _,
    PSP22PalletImpl,
};

pub trait PSP22PalletBurnableImpl: Storage<psp22_pallet::Data> + psp22_pallet::Internal {
    fn burn(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        self._burn_from(account, amount)
    }
}
