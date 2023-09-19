// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::traits::{
    errors::GovernanceError,
    governance::{
        ProposalCore,
        ProposalId,
        Transaction,
    },
};
pub use ink::prelude::collections::VecDeque;
pub use openbrush::{
    storage::Mapping,
    traits::{
        Storage,
        Timestamp,
    },
};

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    /// Stores the proposals
    /// The key is the proposal id and the value is the proposal core
    pub proposals: Mapping<ProposalId, ProposalCore>,
    /// Stored the cross-contract calls that are executed when a proposal is approved
    #[lazy]
    pub governance_call: VecDeque<Transaction>,
}

/// A wrapper that allows us to encode a blob of bytes.
///
/// We use this to pass the set of untyped (bytes) parameters to the `CallBuilder`.
pub struct CallInput<'a>(pub &'a [u8]);

impl<'a> scale::Encode for CallInput<'a> {
    fn encode_to<T: scale::Output + ?Sized>(&self, dest: &mut T) {
        dest.write(self.0);
    }
}

pub trait GovernorStorageGetters: Storage<Data> {
    /// Returns the timestamp when the votes is started for the proposal
    fn _proposal_snapshot(&self, proposal_id: ProposalId) -> Result<Timestamp, GovernanceError> {
        Ok(self
            .data::<Data>()
            .proposals
            .get(&proposal_id)
            .ok_or(GovernanceError::ProposalNotFound)?
            .vote_start)
    }
}
