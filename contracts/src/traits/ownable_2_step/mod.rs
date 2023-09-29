pub use crate::traits::errors::OwnableError;
use openbrush::traits::AccountId;

#[openbrush::wrapper]
pub type Ownable2StepRef = dyn Ownable2Step;

/// Contract module which provides a 2 step ownership mechanism, where an
/// owner can transfer ownership to a new address which must then accept
/// the transfer before becoming the new owner.
#[openbrush::trait_definition]
pub trait Ownable2Step {
    #[ink(message)]
    fn pending_owner(&self) -> Option<AccountId>;
    /// Transfers ownership of the contract to a `pending_owner`.
    /// Can only be called by the current owner.
    ///
    /// On success a `OwnershipTransferStarted` event is emitted.
    ///
    /// # Errors
    ///
    /// Panics with `CallerIsNotOwner` error if caller is not owner.
    #[ink(message)]
    fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<(), OwnableError>;

    /// Accepts ownership of the contract.
    /// Can only be called by pending owner.
    ///
    ///
    /// On success a `OwnershipTransferred` event is emitted.
    ///
    /// # Errors
    ///
    /// Panics with `CallerIsNotPendingOwner` error if caller is not pending owner.
    #[ink(message)]
    fn accept_ownership(&mut self) -> Result<(), OwnableError>;
}
