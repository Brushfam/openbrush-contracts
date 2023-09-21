// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

use openbrush::traits::AccountId;

pub mod psp34;
pub use psp34::*;

pub mod extensions {
    pub mod burnable;
    pub mod enumerable;
    pub mod metadata;
    pub mod mintable;
}

pub type Owner = AccountId;
pub type Operator = AccountId;
