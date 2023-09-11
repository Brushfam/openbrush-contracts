// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

use ink::prelude::vec::Vec;

#[cfg(feature = "std")]
use ink::storage::traits::StorageLayout;

/// `Id` represents the identifier of the NFT. `Id::U8(1)` and `Id::U16(1)` are two different identifiers.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
pub enum Id {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Bytes(Vec<u8>),
}

impl Default for Id {
    fn default() -> Self {
        Self::U8(0)
    }
}
