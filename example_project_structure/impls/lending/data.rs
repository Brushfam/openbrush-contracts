// Importing everything publicly from traits allows you to import every stuff related to lending
// by one import
use openbrush::{
    storage::{
        Mapping,
        TypeGuard,
    },
    traits::{
        AccountId,
        Balance,
        Hash,
    },
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Debug)]
#[openbrush::storage_item]
/// define the struct with the data that our smart contract will be using
/// this will isolate the logic of our smart contract from its storage
pub struct Data {
    /// mapping from asset address to lended asset address
    /// when X amount of asset is lended, X amount of asset it is mapped to is minted
    /// so the contract knows how much of asset it has and how much of the asset was lended
    pub assets_lended: Mapping<AccountId, AccountId>,
    /// mapping from asset address to shares asset address
    /// the lended asset is mapped to a shares asset which represents
    /// the total share of the mapping asset
    /// example: if a user has X% of the total supply of the asset A', they
    /// are eligible to withdraw X% of the asset A tracked by this contract
    pub asset_shares: Mapping<AccountId, AccountId>,
    /// mapping from share token to asset token
    pub shares_asset: Mapping<AccountId, AccountId>,
    /// mapping from asset address to bool
    /// maps to `true` if an asset is accepted for using as collateral
    pub collateral_accepted: Mapping<AccountId, bool>,
    /// mapping from tuple of two assets to balance
    /// mapped balance represents the amount of assets of tuple.1 we get
    /// when we deposit 1 unit of tuple.0
    /// we are using this just to simulate an oracle in our example
    /// in the example the returned balance will be amount of stable coin for an asset
    pub asset_price: Mapping<(AccountId, AccountId), Balance, AssetPriceKey>,
    /// code hash of the `SharesContract`
    pub shares_contract_code_hash: Hash,
    /// the `AccountId` of the `Loan`
    pub loan_account: AccountId,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            assets_lended: Default::default(),
            asset_shares: Default::default(),
            shares_asset: Default::default(),
            collateral_accepted: Default::default(),
            asset_price: Default::default(),
            shares_contract_code_hash: Hash::default(),
            loan_account: [0u8; 32].into(),
        }
    }
}

pub struct AssetPriceKey;

impl<'a> TypeGuard<'a> for AssetPriceKey {
    type Type = &'a (&'a AccountId, &'a AccountId);
}
