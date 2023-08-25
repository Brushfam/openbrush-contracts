// Copyright (c) 2023 Brushfam
// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

pub use crate::traits::{
    errors::GovernanceError,
    governance::{ProposalCore, ProposalId, Transaction},
};
pub use ink::prelude::collections::VecDeque;
pub use openbrush::{
    storage::Mapping,
    traits::{Storage, Timestamp},
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
