// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use openbrush::utils::checkpoints::Checkpoints;

#[derive(Debug, Default)]
#[openbrush::storage_item]
pub struct Data {
    /// Stores the quorum numerator history of the governor
    #[lazy]
    pub quorum_numerator_history: Checkpoints,
}
