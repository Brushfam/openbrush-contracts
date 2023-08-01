use crate::{
    traits::governance::{
        ProposalId,
        ProposalState,
    },
    utils::{
        crypto::CryptoError,
        nonces::NoncesError,
    },
};
use openbrush::traits::{
    AccountId,
    Timestamp,
};

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
    ProposalAlreadyExists,
    FutureLookup(Timestamp, Timestamp),
    ExpiredSignature(Timestamp),
    CryptoError(CryptoError),
    NoncesError(NoncesError),
}

impl From<CryptoError> for GovernanceError {
    fn from(err: CryptoError) -> Self {
        GovernanceError::CryptoError(err)
    }
}

impl From<NoncesError> for GovernanceError {
    fn from(err: NoncesError) -> Self {
        GovernanceError::NoncesError(err)
    }
}
