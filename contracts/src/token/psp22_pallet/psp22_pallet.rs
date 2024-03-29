// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::{
    psp22_pallet,
    traits::psp22::*,
};
pub use ink::{
    env::DefaultEnvironment,
    prelude::vec::Vec,
};
use openbrush::traits::{
    AccountId,
    Balance,
    Storage,
    String,
};
pub use pallet_assets_chain_extension::{
    ink::*,
    traits::*,
};
pub use psp22_pallet::{
    Internal as _,
    InternalImpl as _,
    PSP22PalletImpl as _,
};

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    /// Asset id of the token on the pallet.
    #[lazy]
    pub asset_id: u32,
    /// Default origin of the contract.
    #[lazy]
    pub origin: Origin,
    /// Extension to interact with `pallet-assets`
    #[lazy]
    pub pallet_assets: AssetsExtension,
}

pub trait PSP22PalletImpl: Storage<Data> + Internal {
    fn total_supply(&self) -> Balance {
        let self_ = self.data();
        self_
            .pallet_assets
            .get_or_default()
            .total_supply(self_.asset_id.get_or_default())
    }

    fn balance_of(&self, owner: AccountId) -> Balance {
        let self_ = self.data();
        self_
            .pallet_assets
            .get_or_default()
            .balance_of(self_.asset_id.get_or_default(), owner)
    }

    fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
        let self_ = self.data();
        self_
            .pallet_assets
            .get_or_default()
            .allowance(self_.asset_id.get_or_default(), owner, spender)
    }

    fn transfer(&mut self, to: AccountId, value: Balance, _data: Vec<u8>) -> Result<(), PSP22Error> {
        if value == 0 {
            return Ok(())
        }

        let self_ = self.data();
        self_.pallet_assets.get_or_default().transfer(
            self_.origin.get_or_default(),
            self_.asset_id.get_or_default(),
            to.clone(),
            value,
        )?;
        self._emit_transfer_event(Some(self._sender()), Some(to), value);
        Ok(())
    }

    fn transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        value: Balance,
        _data: Vec<u8>,
    ) -> Result<(), PSP22Error> {
        if value == 0 {
            return Ok(())
        }

        let self_ = self.data();
        self_.pallet_assets.get_or_default().transfer_approved(
            self_.origin.get_or_default(),
            self_.asset_id.get_or_default(),
            from.clone(),
            to.clone(),
            value,
        )?;
        self._emit_transfer_event(Some(from), Some(to), value);
        Ok(())
    }

    fn approve(&mut self, spender: AccountId, value: Balance) -> Result<(), PSP22Error> {
        if value == 0 {
            return Ok(())
        }

        let caller = self._sender();
        let allowance = self.allowance(caller.clone(), spender.clone());
        let self_ = self.data();

        if allowance > 0 {
            // First we reset the previous approve and after set a new one.
            self_.pallet_assets.get_or_default().cancel_approval(
                self_.origin.get_or_default(),
                self_.asset_id.get_or_default(),
                spender.clone(),
            )?;
        }

        self_.pallet_assets.get_or_default().approve_transfer(
            self_.origin.get_or_default(),
            self_.asset_id.get_or_default(),
            spender,
            value,
        )?;
        self._emit_approval_event(caller, spender, value);
        Ok(())
    }

    fn increase_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), PSP22Error> {
        if delta_value == 0 {
            return Ok(())
        }

        let caller = self._sender();
        let allowance = self.allowance(caller.clone(), spender.clone());
        let self_ = self.data();
        // `approve_transfer` increases by default
        self_.pallet_assets.get_or_default().approve_transfer(
            self_.origin.get_or_default(),
            self_.asset_id.get_or_default(),
            spender,
            delta_value,
        )?;
        self._emit_approval_event(caller, spender, allowance + delta_value);

        Ok(())
    }

    fn decrease_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), PSP22Error> {
        if delta_value == 0 {
            return Ok(())
        }

        let caller = self._sender();

        let mut allowance = self.allowance(caller.clone(), spender.clone());

        if allowance < delta_value {
            return Err(PSP22Error::InsufficientAllowance)
        }
        allowance -= delta_value;

        self.approve(spender, allowance)?;
        self._emit_approval_event(caller, spender, allowance);

        Ok(())
    }
}

pub trait Internal {
    /// User must override those methods in their contract.
    fn _emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _amount: Balance);

    fn _emit_approval_event(&self, _owner: AccountId, _spender: AccountId, _amount: Balance);

    fn _mint_to(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error>;

    fn _burn_from(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error>;

    fn _create(
        &mut self,
        asset_id: u32,
        admin: AccountId,
        min_balance: Balance,
    ) -> Result<(), Error<DefaultEnvironment>>;

    fn _sender(&self) -> AccountId;
}

pub trait InternalImpl: Storage<Data> + Internal {
    fn _emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _amount: Balance) {}

    fn _emit_approval_event(&self, _owner: AccountId, _spender: AccountId, _amount: Balance) {}

    fn _mint_to(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        let self_ = self.data();
        self_
            .pallet_assets
            .get_or_default()
            .mint(self_.asset_id.get_or_default(), account.clone(), amount)?;
        Internal::_emit_transfer_event(self, None, Some(account), amount);
        Ok(())
    }

    fn _burn_from(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        let self_ = self.data();
        self_
            .pallet_assets
            .get_or_default()
            .burn(self_.asset_id.get_or_default(), account.clone(), amount)?;
        Internal::_emit_transfer_event(self, Some(account), None, amount);
        Ok(())
    }

    fn _create(
        &mut self,
        asset_id: u32,
        admin: AccountId,
        min_balance: Balance,
    ) -> Result<(), Error<DefaultEnvironment>> {
        self.data()
            .pallet_assets
            .get_or_default()
            .create(asset_id, admin, min_balance)
    }

    fn _sender(&self) -> AccountId {
        match self.data().origin.get_or_default() {
            Origin::Caller => Self::env().caller(),
            Origin::Address => Self::env().account_id(),
        }
    }
}

impl From<Error<DefaultEnvironment>> for PSP22Error {
    fn from(error: Error<DefaultEnvironment>) -> Self {
        match error {
            Error::ContractIsNotAdmin => PSP22Error::Custom(String::from("ContractIsNotAdmin")),
            Error::BalanceLow => PSP22Error::InsufficientBalance,
            Error::NoAccount => PSP22Error::Custom(String::from("NoAccount")),
            Error::NoPermission => PSP22Error::Custom(String::from("NoPermission")),
            Error::Unknown => PSP22Error::Custom(String::from("Unknown")),
            Error::Frozen => PSP22Error::Custom(String::from("Frozen")),
            Error::InUse => PSP22Error::Custom(String::from("InUse")),
            Error::BadWitness => PSP22Error::Custom(String::from("BadWitness")),
            Error::MinBalanceZero => PSP22Error::Custom(String::from("MinBalanceZero")),
            Error::NoProvider => PSP22Error::Custom(String::from("NoProvider")),
            Error::BadMetadata => PSP22Error::Custom(String::from("BadMetadata")),
            Error::Unapproved => PSP22Error::InsufficientAllowance,
            Error::WouldDie => PSP22Error::Custom(String::from("WouldDie")),
            Error::AlreadyExists => PSP22Error::Custom(String::from("AlreadyExists")),
            Error::NoDeposit => PSP22Error::Custom(String::from("NoDeposit")),
            Error::WouldBurn => PSP22Error::Custom(String::from("WouldBurn")),
            Error::AssetPalletInternal => PSP22Error::Custom(String::from("AssetPalletInternal")),
            // All future errors should be `AssetPalletInternal`
            _ => panic!("other error are not supported"),
        }
    }
}
