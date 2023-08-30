use openbrush::traits::AccountId;
#[derive(Debug)]
#[cfg_attr(feature = "non-upgradeable-lazy", openbrush::storage_item(lazy = false))]
#[cfg_attr(not(feature = "non-upgradeable-lazy"), openbrush::storage_item)]
pub struct OwnableData {
    #[lazy]
    pub owner: AccountId,
}

#[derive(Debug)]
#[cfg_attr(feature = "non-upgradeable-lazy", openbrush::storage_item(lazy = false))]
#[cfg_attr(not(feature = "non-upgradeable-lazy"), openbrush::storage_item)]
pub struct ProxyData {
    pub forward: AccountId,
}

fn main() {}
