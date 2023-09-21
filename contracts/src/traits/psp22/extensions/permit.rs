// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

/// Extension of [`PSP22`] that allows create `amount` tokens
/// and assigns them to `account`, increasing the total supply
pub use crate::traits::errors::PSP22Error;
use openbrush::traits::{
    AccountId,
    Balance,
};
pub use openbrush::utils::crypto::Signature;

#[openbrush::wrapper]
pub type PSP22PermitRef = dyn PSP22Permit;

#[openbrush::trait_definition]
pub trait PSP22Permit {
    /// Permit allows `spender` to spend `value` tokens on behalf of `owner` with a signature
    ///
    /// See [`PSP22::_approve`].
    #[ink(message)]
    fn permit(
        &mut self,
        owner: AccountId,
        spender: AccountId,
        value: Balance,
        deadline: u64,
        signature: Signature,
    ) -> Result<(), PSP22Error>;

    #[ink(message)]
    fn domain_separator(&mut self) -> [u8; 32];
}
