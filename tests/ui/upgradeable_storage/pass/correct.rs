use openbrush::traits::AccountId;
pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(OwnableData);

#[derive(Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct OwnableData {
   pub owner: AccountId,
   pub _reserved: Option<()>,
}

const PROXY_KEY : u32 = openbrush::storage_unique_key!(ProxyData);

#[derive(Debug)]
#[openbrush::upgradeable_storage(PROXY_KEY)]
pub struct ProxyData {
   pub forward: AccountId,
   pub _reserved: Option<()>,
}

#[derive(Debug)]
#[openbrush::upgradeable_storage(123)]
pub struct SomeData {
   pub _reserved: Option<()>,
}

fn main() {}