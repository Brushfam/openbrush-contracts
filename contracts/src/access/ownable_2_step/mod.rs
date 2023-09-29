pub use crate::{
    ownable_2_step,
    traits::ownable_2_step::*,
    ownable,
    ownable::*,
    traits::ownable::*,
};
use openbrush::{
    modifiers,
    traits::{
        AccountId,
        Storage,
    },
};

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    #[lazy]
    pub pending_owner: Option<AccountId>,
}

pub trait Ownable2StepImpl: Storage<Data> + Storage<ownable::Data> + Internal + OwnableImpl {
    fn pending_owner(&self) -> Option<AccountId> {
        self.data::<Data>().pending_owner.get_or_default()
    }

    #[modifiers(only_owner)]
    fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<(), OwnableError> {
        self.data::<Data>().pending_owner.set(&Some(new_owner));
        let owner = self.data::<ownable::Data>().owner.get_or_default();
        self._emit_ownership_transferred_started_event(owner, Some(new_owner));
        Ok(())
    }

    fn accept_ownership(&mut self) -> Result<(), OwnableError> {
        let caller = Self::env().caller();
        if self.data::<Data>().pending_owner.get_or_default() != Some(caller) {
            return Err(OwnableError::OwnableUnauthorizedAccount)
        }
        self::Internal::_transfer_ownership(self, Some(caller))?;
        Ok(())
    }
}

pub trait Internal {
    /// User must override this method in their contract.
    fn _emit_ownership_transferred_started_event(&self, _previous: Option<AccountId>, _new: Option<AccountId>);

    fn _transfer_ownership(&mut self, new_owner: Option<AccountId>) -> Result<(), OwnableError>;
}

pub trait InternalImpl: Storage<Data> + Internal + ownable::Internal {
    fn _emit_ownership_transferred_started_event(&self, _previous: Option<AccountId>, _new: Option<AccountId>) {}

    fn _transfer_ownership(&mut self, new_owner: Option<AccountId>) -> Result<(), OwnableError> {
        self.data::<Data>().pending_owner.set(&None);
        ownable::Internal::_transfer_ownership(self, new_owner)?;
        Ok(())
    }
}
