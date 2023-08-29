use openbrush::traits::AccountId;

/// Provides tracking nonces for addresses. Nonces will only increment.
#[openbrush::trait_definition]
pub trait Nonces {
    /// Returns the nonce of `account`.
    #[ink(message)]
    fn nonces(&self, account: AccountId) -> u128;
}
