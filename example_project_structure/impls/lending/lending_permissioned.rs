use super::{
    lending_internal,
    lending_internal::*,
};
use crate::traits::lending::*;
use openbrush::{
    contracts::{
        access_control::*,
        traits::psp22::PSP22Ref,
    },
    modifiers,
    traits::{
        AccountId,
        Balance,
        Storage,
    },
};

pub const MANAGER: RoleType = ink::selector_id!("MANAGER");

pub trait LendingPermissionedImpl:
    access_control::Internal + Storage<access_control::Data> + lending_internal::Internal + Lending + Instantiator
{
    #[modifiers(only_role(MANAGER))]
    fn allow_asset(&mut self, asset_address: AccountId) -> Result<(), LendingError> {
        // we will ensure the asset is not accepted already
        if self.is_accepted_lending(asset_address) {
            return Err(LendingError::AssetSupported)
        }

        // instantiate the shares of the lended assets
        let shares_address = self._instantiate_shares_contract("LendingShares", "LS");
        // instantiate the reserves of the borrowed assets
        let reserves_address = self._instantiate_shares_contract("LendingReserves", "LR");
        // accept the asset and map shares and reserves to it

        self._accept_lending(asset_address, shares_address, reserves_address);
        Ok(())
    }

    #[modifiers(only_role(MANAGER))]
    fn disallow_lending(&mut self, asset_address: AccountId) -> Result<(), LendingError> {
        let reserve_asset = self._get_reserve_asset(&asset_address)?;
        if PSP22Ref::balance_of(&asset_address, Self::env().account_id()) > 0
            || PSP22Ref::balance_of(&reserve_asset, Self::env().account_id()) > 0
        {
            return Err(LendingError::AssetsInTheContract)
        }
        self._disallow_lending(asset_address);
        Ok(())
    }

    #[modifiers(only_role(MANAGER))]
    fn allow_collateral(&mut self, asset_address: AccountId) -> Result<(), LendingError> {
        // we will ensure the asset is not accepted already
        if self.is_accepted_collateral(asset_address) {
            return Err(LendingError::AssetSupported)
        }
        self._set_collateral_accepted(asset_address, true);
        Ok(())
    }

    #[modifiers(only_role(MANAGER))]
    fn disallow_collateral(&mut self, asset_address: AccountId) -> Result<(), LendingError> {
        // we will ensure the asset is not accepted already
        if self.is_accepted_collateral(asset_address) {
            self._set_collateral_accepted(asset_address, false);
        }
        Ok(())
    }

    #[modifiers(only_role(MANAGER))]
    fn set_asset_price(
        &mut self,
        asset_in: AccountId,
        asset_out: AccountId,
        price: Balance,
    ) -> Result<(), LendingError> {
        self._set_asset_price(&asset_in, &asset_out, &price);
        Ok(())
    }
}
