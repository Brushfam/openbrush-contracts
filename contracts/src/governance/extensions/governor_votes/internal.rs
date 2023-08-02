use crate::{
    extensions::governor_votes::Data,
    traits::{
        errors::GovernanceError,
        governance::utils::VotesWrapper,
    },
};
use ink::{
    prelude::vec::Vec,
    primitives::AccountId,
};
use openbrush::traits::{
    Balance,
    Storage,
    StorageAsRef,
    Timestamp,
};

pub trait GovernorVotesInternal: Storage<Data> {
    fn _init_governor_votes(&mut self, token: AccountId) -> Result<(), GovernanceError> {
        self.data().token.set(token);
        Ok(())
    }

    fn _get_votes(
        &mut self,
        account: AccountId,
        timepoint: Timestamp,
        _params: Vec<u8>,
    ) -> Result<Balance, GovernanceError> {
        let token = self.data().token.get().ok_or(GovernanceError::TokenNotSet)?;

        VotesWrapper::get_past_votes(&mut token, account, timepoint)
    }
}
