use super::data::Data;
use crate::traits::lending::*;
use openbrush::traits::{
    AccountId,
    Balance,
    Storage,
    StorageAccess,
};
pub use Internal as _;

pub trait Instantiator {
    /// Internal function which instantiates a shares contract and returns its AccountId
    fn _instantiate_shares_contract(&self, contract_name: &str, contract_symbol: &str) -> AccountId;
}

pub trait Internal: StorageAccess<Data> + Sized {
    fn _accept_lending(&mut self, asset_address: AccountId, share_address: AccountId, reserve_address: AccountId) {
        self.get_or_default()
            .asset_shares
            .insert(&asset_address, &share_address);
        self.get_or_default()
            .shares_asset
            .insert(&share_address, &asset_address);
        self.get_or_default()
            .assets_lended
            .insert(&asset_address, &reserve_address);
    }

    fn _disallow_lending(&mut self, asset_address: AccountId) {
        if let Some(share_address) = self.data().asset_shares.get(&asset_address) {
            self.get_or_default().asset_shares.remove(&asset_address);
        self.get_or_default().shares_asset.remove(&share_address);
        self.get_or_default().assets_lended.remove(&asset_address);
        }
    }

    /// this function will accept `asset_address` for using as collateral
    fn _set_collateral_accepted(&mut self, asset_address: AccountId, accepted: bool) {
        self.get_or_default()
            .collateral_accepted
            .insert(&asset_address, &accepted);
    }

    /// this internal function will be used to set price of `asset_in` when we deposit `asset_out`
    /// we are using this function in our example to simulate an oracle
    fn _set_asset_price(&mut self, asset_in: &AccountId, asset_out: &AccountId, price: &Balance) {
        self.get_or_default().asset_price.insert(&(asset_in, asset_out), price);
    }

    /// this internal function will be used to set price of `asset_in` when we deposit `asset_out`
    /// we are using this function in our example to simulate an oracle
    fn _get_asset_price(&self, amount_in: &Balance, asset_in: &AccountId, asset_out: &AccountId) -> Balance {
        let price = self
            .get_or_default()
            .asset_price
            .get(&(asset_in, asset_out))
            .unwrap_or(0);
        price * amount_in
    }

    /// Internal function which will return the address of the shares token
    /// which are minted when `asset_address` is borrowed
    fn _get_reserve_asset(&self, asset_address: &AccountId) -> Result<AccountId, LendingError> {
        self.get_or_default()
            .asset_shares
            .get(&asset_address)
            .ok_or(LendingError::AssetNotSupported)
    }

    /// internal function which will return the address of asset
    /// which is bound to `shares_address` shares token
    fn _get_asset_from_shares(&self, shares_address: &AccountId) -> Result<AccountId, LendingError> {
        self.get_or_default()
            .shares_asset
            .get(shares_address)
            .ok_or(LendingError::AssetNotSupported)
    }
}
