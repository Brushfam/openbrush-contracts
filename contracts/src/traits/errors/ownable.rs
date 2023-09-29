// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

/// The Ownable error type. Contract will throw one of this errors.
/// Modifier returns `CallerIsNotOwner`.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum OwnableError {
    CallerIsNotOwner,
    NewOwnerIsNotSet,
    OwnableUnauthorizedAccount,
}
