// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

/// Extension of [`PSP37`] that adds enumerability of all the token ids in the contract as well
/// as all token ids owned by each account.
pub use crate::traits::psp37::*;
use openbrush::traits::AccountId;

#[openbrush::wrapper]
pub type PSP37EnumerableRef = dyn PSP37Enumerable + PSP37;

#[openbrush::trait_definition]
pub trait PSP37Enumerable: PSP37 {
    /// Returns a token `Id` owned by `owner` at a given `index` of its token list.
    /// Use along with `balance_of` to enumerate all of ``owner``'s tokens.
    ///
    /// The start index is zero.
    #[ink(message)]
    fn owners_token_by_index(&self, owner: AccountId, index: u128) -> Option<Id>;

    /// Returns a token `Id` at a given `index` of all the tokens stored by the contract.
    /// Use along with `total_supply` to enumerate all tokens.
    ///
    /// The start index is zero.
    #[ink(message)]
    fn token_by_index(&self, index: u128) -> Option<Id>;
}
