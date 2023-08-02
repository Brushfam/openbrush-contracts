use openbrush::traits::AccountId;

#[openbrush::trait_definition]
pub trait Nonces {
    #[ink(message)]
    fn nonces(&self, account: AccountId) -> u128;
}
