// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

use crate::traits::errors::NoncesError;
use openbrush::utils::{
    checkpoints::CheckpointsError,
    crypto::CryptoError,
};

/// The Governor error type. Contract will throw one of this errors.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum GovernanceError {
    TokenNotSet,
    InvalidQuorumFraction,
    AlreadyCastVote,
    DisabledDeposit,
    OnlyProposer,
    OnlyExecutor,
    NonexistentProposal,
    UnexpectedProposalState,
    InvalidVotingPeriod,
    InsufficientProposerVotes,
    InvalidVoteType,
    InvalidSignature,
    ProposerRestricted,
    InvalidDestination,
    ZeroSnapshot,
    DeadlineOverflow,
    ZeroProposalLength,
    ProposalNotFound,
    InvalidInput,
    UnderlyingTransactionReverted,
    ProposalAlreadyExists,
    ErrorParsingDescription,
    FutureLookup,
    ExpiredSignature,
    CryptoError(CryptoError),
    NoncesError(NoncesError),
    ExecutionFailed,
    CheckpointsError(CheckpointsError),
    IndexOutOfRange,
    Overflow,
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
