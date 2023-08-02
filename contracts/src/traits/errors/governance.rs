use crate::traits::{
    errors::{
        CheckpointsError,
        CryptoError,
        NoncesError,
    },
    governance::{
        ProposalId,
        ProposalState,
        Transaction,
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
    ErrorParsingDescription,
    FutureLookup(Timestamp, Timestamp),
    ExpiredSignature(Timestamp),
    CryptoError(CryptoError),
    NoncesError(NoncesError),
    ExecutionFailed(Transaction),
    CheckpointsError(CheckpointsError),
    IndexOutOfRange,
    AccountNotFound,
    Overflow,
    InvalidQuorumFraction(u128, u128),
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

impl From<CheckpointsError> for GovernanceError {
    fn from(err: CheckpointsError) -> Self {
        GovernanceError::CheckpointsError(err)
    }
}
