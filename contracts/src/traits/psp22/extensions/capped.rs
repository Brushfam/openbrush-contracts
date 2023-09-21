// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

/// Extension of [`PSP22`] that allows to set a limit on the total funding
use openbrush::traits::Balance;

#[openbrush::wrapper]
pub type PSP22CappedRef = dyn PSP22Capped;

#[openbrush::trait_definition]
pub trait PSP22Capped {
    /// Returns the token's cap
    #[ink(message)]
    fn cap(&self) -> Balance;
}
