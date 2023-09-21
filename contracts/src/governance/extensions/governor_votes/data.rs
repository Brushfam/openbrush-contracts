// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use openbrush::traits::AccountId;

#[derive(Debug, Default)]
#[openbrush::storage_item]
pub struct Data {
    /// Stores the token of the `PSP22Votes` contract
    #[lazy]
    pub token: AccountId,
}
