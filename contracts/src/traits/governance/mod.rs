use crate::traits::errors::GovernanceError;
use ink::prelude::vec::Vec;
use openbrush::traits::{
    AccountId,
    Balance,
    Timestamp,
};

pub mod extensions;
pub mod governor;
pub mod utils;

pub type ProposalId = [u8; 32];
pub type HashType = [u8; 32];
pub type Selector = [u8; 4];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
pub struct Transaction {
    pub callee: Option<AccountId>,
    pub selector: [u8; 4],
    pub destination: AccountId,
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
pub enum CancellationStatus {
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
    pub cancelled: CancellationStatus,
}

impl Default for ProposalCore {
    fn default() -> Self {
        Self {
            proposer: AccountId::from([0x0; 32]),
            vote_start: Default::default(),
            vote_duration: Default::default(),
            executed: Default::default(),
            cancelled: Default::default(),
        }
    }
}

impl ProposalCore {
    pub fn new(proposer: AccountId, vote_start: Timestamp, vote_duration: Timestamp) -> Self {
        Self {
            proposer,
            vote_start,
            vote_duration,
            executed: ExecutionStatus::NotExecuted,
            cancelled: CancellationStatus::NotCanceled,
        }
    }

    pub fn is_executed(&self) -> bool {
        self.executed == ExecutionStatus::Executed
    }

    pub fn is_canceled(&self) -> bool {
        self.cancelled == CancellationStatus::Canceled
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
