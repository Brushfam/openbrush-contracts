use openbrush::traits::{AccountId, Balance};

pub mod extensions;
pub mod governor;

pub type ProposalId = [u8; 32];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
pub struct Transaction {
    pub callee: Option<AccountId>,
    pub selector: [u8; 4],
    pub input: Vec<u8>,
    pub transferred_value: Balance,
    pub gas_limit: u64,
}
