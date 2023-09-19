// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

use super::{
    AccessControlError,
    OwnableError,
    PSP22Error,
    PSP22ReceiverError,
    PausableError,
    ReentrancyGuardError,
};
use openbrush::traits::String;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum FlashBorrowerError {
    FlashloanRejected(String),
}

impl From<OwnableError> for FlashBorrowerError {
    fn from(ownable: OwnableError) -> Self {
        match ownable {
            OwnableError::CallerIsNotOwner => {
                FlashBorrowerError::FlashloanRejected(String::from("O::CallerIsNotOwner"))
            }
            OwnableError::NewOwnerIsNotSet => FlashBorrowerError::FlashloanRejected(String::from("O::NewOwnerIsNotSet")),
        }
    }
}

impl From<AccessControlError> for FlashBorrowerError {
    fn from(access: AccessControlError) -> Self {
        match access {
            AccessControlError::MissingRole => FlashBorrowerError::FlashloanRejected(String::from("AC::MissingRole")),
            AccessControlError::RoleRedundant => {
                FlashBorrowerError::FlashloanRejected(String::from("AC::RoleRedundant"))
            }
            AccessControlError::InvalidCaller => {
                FlashBorrowerError::FlashloanRejected(String::from("AC::InvalidCaller"))
            }
        }
    }
}

impl From<PausableError> for FlashBorrowerError {
    fn from(pausable: PausableError) -> Self {
        match pausable {
            PausableError::Paused => FlashBorrowerError::FlashloanRejected(String::from("P::Paused")),
            PausableError::NotPaused => FlashBorrowerError::FlashloanRejected(String::from("P::NotPaused")),
        }
    }
}

impl From<ReentrancyGuardError> for FlashBorrowerError {
    fn from(guard: ReentrancyGuardError) -> Self {
        match guard {
            ReentrancyGuardError::ReentrantCall => {
                FlashBorrowerError::FlashloanRejected(String::from("RG::ReentrantCall"))
            }
        }
    }
}

impl From<PSP22ReceiverError> for FlashBorrowerError {
    fn from(error: PSP22ReceiverError) -> Self {
        match error {
            PSP22ReceiverError::TransferRejected(message) => FlashBorrowerError::FlashloanRejected(message),
        }
    }
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum FlashLenderError {
    Custom(String),
    /// Returned if we our flashlendner does not support lending of this token
    WrongTokenAddress,
    /// Returned if the contract does not have enough allowance to transfer borrowed amount and fees
    AllowanceDoesNotAllowRefund,
    /// Callee contract rejected the flashloan
    BorrowerRejected(String),
}

impl From<PSP22Error> for FlashLenderError {
    fn from(error: PSP22Error) -> Self {
        match error {
            PSP22Error::Custom(message) => FlashLenderError::Custom(message),
            PSP22Error::InsufficientBalance => FlashLenderError::Custom(String::from("PSP22: Insufficient Balance")),
            PSP22Error::InsufficientAllowance => {
                FlashLenderError::Custom(String::from("PSP22: Insufficient Allowance"))
            }
            PSP22Error::RecipientIsNotSet => FlashLenderError::Custom(String::from("PSP22: Recipient Address is not set")),
            PSP22Error::SenderIsNotSet => FlashLenderError::Custom(String::from("PSP22: Sender Address in not set")),
            PSP22Error::SafeTransferCheckFailed(message) => FlashLenderError::Custom(message),
        }
    }
}

impl From<FlashBorrowerError> for FlashLenderError {
    fn from(error: FlashBorrowerError) -> Self {
        match error {
            FlashBorrowerError::FlashloanRejected(message) => FlashLenderError::BorrowerRejected(message),
        }
    }
}

impl From<OwnableError> for FlashLenderError {
    fn from(ownable: OwnableError) -> Self {
        match ownable {
            OwnableError::CallerIsNotOwner => FlashLenderError::Custom(String::from("O::CallerIsNotOwner")),
            OwnableError::NewOwnerIsNotSet => FlashLenderError::Custom(String::from("O::NewOwnerIsNotSet")),
        }
    }
}

impl From<AccessControlError> for FlashLenderError {
    fn from(access: AccessControlError) -> Self {
        match access {
            AccessControlError::MissingRole => FlashLenderError::Custom(String::from("AC::MissingRole")),
            AccessControlError::RoleRedundant => FlashLenderError::Custom(String::from("AC::RoleRedundant")),
            AccessControlError::InvalidCaller => FlashLenderError::Custom(String::from("AC::InvalidCaller")),
        }
    }
}

impl From<PausableError> for FlashLenderError {
    fn from(pausable: PausableError) -> Self {
        match pausable {
            PausableError::Paused => FlashLenderError::Custom(String::from("P::Paused")),
            PausableError::NotPaused => FlashLenderError::Custom(String::from("P::NotPaused")),
        }
    }
}

impl From<ReentrancyGuardError> for FlashLenderError {
    fn from(guard: ReentrancyGuardError) -> Self {
        match guard {
            ReentrancyGuardError::ReentrantCall => FlashLenderError::Custom(String::from("RG::ReentrantCall")),
        }
    }
}
