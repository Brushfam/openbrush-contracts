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

use crate::traits::{
    errors::NoncesError,
    governance::{
        ProposalId,
        ProposalState,
        Transaction,
    },
};
use openbrush::{
    traits::{
        AccountId,
        Timestamp,
    },
    utils::{
        checkpoints::CheckpointsError,
        crypto::CryptoError,
    },
};

/// The Governor error type. Contract will throw one of this errors.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum GovernanceError {
    TokenNotSet,
    InvalidQuorumFraction(u128, u128),
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
