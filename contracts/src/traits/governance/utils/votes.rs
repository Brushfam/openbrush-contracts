use crate::traits::{
    errors::GovernanceError,
    types::SignatureType,
};
use openbrush::traits::{
    AccountId,
    Balance,
    Timestamp,
};

#[openbrush::trait_definition]
pub trait Votes {
    #[ink(message)]
    fn get_votes(&self, account: AccountId) -> Result<Balance, GovernanceError>;

    #[ink(message)]
    fn get_past_votes(&self, account: AccountId, timestamp: Timestamp) -> Result<Balance, GovernanceError>;

    #[ink(message)]
    fn get_past_total_supply(&self, timestamp: Timestamp) -> Result<Balance, GovernanceError>;

    #[ink(message)]
    fn delegates(&mut self, delegator: AccountId) -> Option<AccountId>;

    #[ink(message)]
    fn delegate(&mut self, delegatee: AccountId) -> Result<(), GovernanceError>;

    #[ink(message)]
    fn delegate_by_signature(
        &mut self,
        signer: AccountId,
        delegatee: AccountId,
        nonce: u128,
        expiry: Timestamp,
        signature: SignatureType,
    ) -> Result<(), GovernanceError>;
}

#[openbrush::wrapper]
pub type VotesWrapper = dyn Votes;
