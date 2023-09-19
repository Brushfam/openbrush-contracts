// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::traits::governance::{
    ProposalId,
    ProposalVote,
};
pub use openbrush::{
    storage::Mapping,
    traits::AccountId,
};

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    /// Stores the ammounts of the votes of the proposals
    /// The key is the proposal id and the value is the vote, which contains the ammount of votes
    pub proposal_votes: Mapping<ProposalId, ProposalVote>,

    /// Stores if the account has voted for the proposal
    pub has_votes: Mapping<(ProposalId, AccountId), ()>,
}
