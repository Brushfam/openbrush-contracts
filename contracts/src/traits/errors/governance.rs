use crate::traits::governance::{ProposalId, ProposalState};
use crate::utils::crypto::CryptoError;
use openbrush::traits::{AccountId, Timestamp};

/// The Governor error type. Contract will throw one of this errors.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum GovernanceError {
    /// TODO : make errors as structs
    AlreadyCastVote(AccountId),
    DisabledDeposit,
    OnlyProposer(AccountId),
    OnlyExecutor(AccountId),
    NonexistentProposal(ProposalId),
    UnexpectedProposalState(ProposalId, ProposalState, u128),
    InvalidVotingPeriod(Timestamp),
    InsufficientProposerVotes(AccountId, u128, u128),
    InvalidVoteType,
    InvalidSignature(AccountId),
    ProposerRestricted(AccountId),
    InvalidDestination,
    ZeroSnapshot,
    DeadlineOverflow,
    ZeroProposalLength,
    ProposalNotFound,
    InvalidInput,
    UnderlyingTransactionReverted,
    CryptoError(CryptoError),
}

impl From<CryptoError> for GovernanceError {
    fn from(err: CryptoError) -> Self {
        GovernanceError::CryptoError(err)
    }
}
