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

use crate::traits::errors::GovernanceError;
use ink::prelude::vec::Vec;
use openbrush::traits::{
    AccountId,
    Balance,
    Timestamp,
};

pub mod governor;

pub use governor::*;

pub mod extensions {
    pub mod governor_counting;
    pub mod governor_quorum;
    pub mod governor_settings;
    pub mod timelock_controller;
}

pub mod utils {
    pub mod votes;
}
pub type ProposalId = [u8; 32];
pub type HashType = [u8; 32];
pub type Selector = [u8; 4];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
pub struct Transaction {
    pub callee: AccountId,
    pub selector: [u8; 4],
    pub input: Vec<u8>,
    pub transferred_value: Balance,
    pub gas_limit: u64,
}

#[derive(scale::Decode, scale::Encode, Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
pub enum ExecutionStatus {
    #[default]
    NotExecuted,
    Executed,
}

#[derive(scale::Decode, scale::Encode, Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
pub enum CancelationStatus {
    #[default]
    NotCanceled,
    Canceled,
}

#[derive(scale::Decode, scale::Encode, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
pub struct ProposalCore {
    pub proposer: AccountId,
    pub vote_start: Timestamp,
    pub vote_duration: Timestamp,
    pub executed: ExecutionStatus,
    pub canceled: CancelationStatus,
}

impl ProposalCore {
    pub fn new(proposer: AccountId, vote_start: Timestamp, vote_duration: Timestamp) -> Self {
        Self {
            proposer,
            vote_start,
            vote_duration,
            executed: ExecutionStatus::NotExecuted,
            canceled: CancelationStatus::NotCanceled,
        }
    }

    pub fn is_executed(&self) -> bool {
        self.executed == ExecutionStatus::Executed
    }

    pub fn is_canceled(&self) -> bool {
        self.canceled == CancelationStatus::Canceled
    }

    pub fn deadline(&self) -> Result<u64, GovernanceError> {
        let start = self.vote_start.clone();
        let duration = self.vote_duration.clone();

        start.checked_add(duration).ok_or(GovernanceError::DeadlineOverflow)
    }

    pub fn hash(&self) -> [u8; 32] {
        use ink::env::hash;

        let bytes: Vec<u8> = scale::Encode::encode(&self);

        let mut output = <hash::Blake2x256 as hash::HashOutput>::Type::default();
        ink::env::hash_bytes::<hash::Blake2x256>(&bytes[..], &mut output);

        output
    }
}

#[derive(scale::Decode, scale::Encode, Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
pub enum ProposalState {
    #[default]
    Pending = 1 << 0,
    Active = 1 << 1,
    Canceled = 1 << 2,
    Defeated = 1 << 3,
    Succeeded = 1 << 4,
    Queued = 1 << 5,
    Expired = 1 << 6,
    Executed = 1 << 7,
}

impl ProposalState {
    pub fn u128(self) -> u128 {
        self as u128
    }
}

pub const ALL_PROPOSAL_STATES: u128 = 0b11111111;

#[derive(scale::Decode, scale::Encode, Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
pub struct ProposalVote {
    pub against_votes: Balance,
    pub for_votes: Balance,
    pub abstain_votes: Balance,
}

#[derive(scale::Decode, scale::Encode, Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
pub enum VoteType {
    #[default]
    Against,
    For,
    Abstain,
}
