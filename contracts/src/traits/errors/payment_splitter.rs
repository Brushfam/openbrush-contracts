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

/// The PaymentSplitter error type. Contract will throw one of this errors.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PaymentSplitterError {
    Custom(String),
    NoPayees,
    AccountHasNoShares,
    AccountIsNotDuePayment,
    AccountIsNotSet,
    SharesAreZero,
    AlreadyHasShares,
    TransferFailed,
}

impl From<AccessControlError> for PaymentSplitterError {
    fn from(access: AccessControlError) -> Self {
        match access {
            AccessControlError::MissingRole => PaymentSplitterError::Custom(String::from("AC::MissingRole")),
            AccessControlError::RoleRedundant => PaymentSplitterError::Custom(String::from("AC::RoleRedundant")),
            AccessControlError::InvalidCaller => PaymentSplitterError::Custom(String::from("AC::InvalidCaller")),
        }
    }
}

impl From<OwnableError> for PaymentSplitterError {
    fn from(ownable: OwnableError) -> Self {
        match ownable {
            OwnableError::CallerIsNotOwner => PaymentSplitterError::Custom(String::from("O::CallerIsNotOwner")),
            OwnableError::NewOwnerIsNotSet => PaymentSplitterError::Custom(String::from("O::NewOwnerIsNotSet")),
        }
    }
}

impl From<PausableError> for PaymentSplitterError {
    fn from(pausable: PausableError) -> Self {
        match pausable {
            PausableError::Paused => PaymentSplitterError::Custom(String::from("P::Paused")),
            PausableError::NotPaused => PaymentSplitterError::Custom(String::from("P::NotPaused")),
        }
    }
}

impl From<ReentrancyGuardError> for PaymentSplitterError {
    fn from(guard: ReentrancyGuardError) -> Self {
        match guard {
            ReentrancyGuardError::ReentrantCall => PaymentSplitterError::Custom(String::from("RG::ReentrantCall")),
        }
    }
}
