// Copyright (c) 2023 Brushfam
// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use crate::{
    governance::extensions::governor_counting::{
        CountingInternal,
        Data,
    },
    traits::{
        errors::GovernanceError,
        governance::ProposalId,
    },
};
use openbrush::traits::{
    AccountId,
    Balance,
    Storage,
    String,
};

pub trait GovernorCountingImpl: Storage<Data> + CountingInternal {
    fn counting_mode(&self) -> String {
        String::from("support=bravo&quorum=for,abstain")
    }

    fn has_voted(&self, proposal_id: ProposalId, account: AccountId) -> bool {
        self.data::<Data>()
            .has_votes
            .get(&(proposal_id, account))
            .unwrap_or_default()
    }

    fn proposal_votes(&self, proposal_id: ProposalId) -> Result<(Balance, Balance, Balance), GovernanceError> {
        let proposal_vote = self.data::<Data>().proposal_votes.get(&proposal_id).unwrap_or_default();
        Ok((
            proposal_vote.for_votes,
            proposal_vote.against_votes,
            proposal_vote.abstain_votes,
        ))
    }
}
