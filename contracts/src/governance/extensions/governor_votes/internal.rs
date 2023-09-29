// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::{
    governance::extensions::governor_votes,
    traits::governance::utils::votes::*,
};

use crate::governance::extensions::governor_votes::Data;

use ink::{
    prelude::vec::Vec,
    primitives::AccountId,
};
use openbrush::traits::{
    Balance,
    Storage,
    Timestamp,
};

/// Extension of `Governor` for voting weight extraction from an `PSP22Votes` token
pub trait GovernorVotesInternal: Storage<Data> {
    /// Initializes the governor votes extension
    fn _init_governor_votes(&mut self, token: AccountId) -> Result<(), GovernanceError> {
        self.data().token.set(&token);
        Ok(())
    }

    /// Returns the total number of votes for an account at a given timestamp.
    fn _get_votes(
        &mut self,
        account: AccountId,
        timestamp: Timestamp,
        _params: Vec<u8>,
    ) -> Result<Balance, GovernanceError> {
        let token = self.data().token.get().ok_or(GovernanceError::TokenNotSet)?;

        VotesRef::get_past_votes(&token, account, timestamp)
    }
}
