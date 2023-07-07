use openbrush::traits::AccountId;
#[derive(Debug)]
#[openbrush::storage_item]
pub struct OwnableData {
   #[lazy]
   pub owner: AccountId,
}

#[derive(Debug)]
#[openbrush::storage_item]
pub struct ProxyData {
   pub forward: AccountId,
}

fn main() {}