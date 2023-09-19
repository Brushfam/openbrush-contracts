// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    /// The minimum number of votes required for a proposal to be accepted
    #[lazy]
    pub proposal_threshold: u128,
    /// The delay before voting on a proposal starts
    #[lazy]
    pub voting_delay: u64,
    /// The duration of the voting period
    #[lazy]
    pub voting_period: u64,
}
