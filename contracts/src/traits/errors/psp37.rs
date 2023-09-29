// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

use super::{
    AccessControlError,
    OwnableError,
    PausableError,
    ReentrancyGuardError,
};
use openbrush::traits::String;

/// The PSP37 error type. Contract will throw one of this errors.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PSP37Error {
    /// Custom error type for cases if writer of traits added own restrictions
    Custom(String),
    /// Returned if the account doesn't contain enough funds.
    InsufficientBalance,
    /// Returned if recipient address is not set.
    TransferToNonSetAddress,
    /// Returned if token doesn't exist
    TokenNotExists,
    /// Returned if the caller is not allowed.
    NotAllowed,
    /// Returned if caller trying to approve himself
    SelfApprove,
    /// Returned if safe transfer check fails
    SafeTransferCheckFailed(String),
}

impl From<OwnableError> for PSP37Error {
    fn from(ownable: OwnableError) -> Self {
        match ownable {
            OwnableError::CallerIsNotOwner => PSP37Error::Custom(String::from("O::CallerIsNotOwner")),
            OwnableError::NewOwnerIsNotSet => PSP37Error::Custom(String::from("O::NewOwnerIsNotSet")),
            OwnableError::OwnableUnauthorizedAccount => {
                PSP37Error::Custom(String::from("O::OwnableUnauthorizedAccount"))
            }
        }
    }
}

impl From<AccessControlError> for PSP37Error {
    fn from(access: AccessControlError) -> Self {
        match access {
            AccessControlError::MissingRole => PSP37Error::Custom(String::from("AC::MissingRole")),
            AccessControlError::RoleRedundant => PSP37Error::Custom(String::from("AC::RoleRedundant")),
            AccessControlError::InvalidCaller => PSP37Error::Custom(String::from("AC::InvalidCaller")),
        }
    }
}

impl From<PausableError> for PSP37Error {
    fn from(pausable: PausableError) -> Self {
        match pausable {
            PausableError::Paused => PSP37Error::Custom(String::from("P::Paused")),
            PausableError::NotPaused => PSP37Error::Custom(String::from("P::NotPaused")),
        }
    }
}

impl From<ReentrancyGuardError> for PSP37Error {
    fn from(guard: ReentrancyGuardError) -> Self {
        match guard {
            ReentrancyGuardError::ReentrantCall => PSP37Error::Custom(String::from("RG::ReentrantCall")),
        }
    }
}

/// The PSP37Receiver error types.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PSP37ReceiverError {
    /// Returned if transfer is rejected.
    TransferRejected(String),
}

impl From<PSP37ReceiverError> for PSP37Error {
    fn from(error: PSP37ReceiverError) -> Self {
        match error {
            PSP37ReceiverError::TransferRejected(message) => PSP37Error::SafeTransferCheckFailed(message),
        }
    }
}
