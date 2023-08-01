use crate::{
    traits::governance::ProposalId,
};
use openbrush::traits::{
    AccountId,
    Balance,
    Storage,
    String,
};
use crate::governance::extensions::governor_counting::{Data, CountingInternal};

pub trait GovernorCountingImpl: Storage<Data> + CountingInternal {
    fn counting_mode(&self) -> String {
        String::from("support=bravo&quorum=for,abstain")
    }

    fn has_voted(&self, proposal_id: ProposalId, account: AccountId) -> bool {
        self.data().has_votes.get(&(proposal_id, account)).unwrap_or_default()
    }

    fn proposal_votes(&self, proposal_id: ProposalId) -> (Balance, Balance, Balance) {
        let proposal_vote = self.data().proposal_votes.get(&proposal_id).unwrap_or_default();
        (proposal_vote.for_votes, proposal_vote.against_votes, proposal_vote.abstain_votes)
    }
}
