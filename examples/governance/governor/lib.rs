#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(Governor, GovernorSettings, GovernorQuorum, GovernorVotes, GovernorCounting)]
#[openbrush::contract]
pub mod my_governor {
    use ink::prelude::vec::Vec;
    use openbrush::traits::{
        Storage,
        String,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        governor: governor::Data,
        #[storage_field]
        governor_counting: governor_counting::Data,
        #[storage_field]
        governor_votes: governor_votes::Data,
        #[storage_field]
        settings: governor_settings::Data,
        #[storage_field]
        quorum: governor_quorum::Data,
        mock_timestamp: Timestamp,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(
            token: AccountId,
            voting_delay: u64,
            voting_period: u64,
            proposal_threshold: u128,
            numerator: u128,
        ) -> Self {
            let mut instance = Self::default();

            let caller = Self::env().caller();

            instance._init_governor_votes(token).unwrap();
            instance
                ._init_governor_settings(voting_delay, voting_period, proposal_threshold)
                .unwrap();
            instance._init_quorum_numerator(numerator).unwrap();
            instance.mock_timestamp = Self::env().block_timestamp();

            instance
        }

        #[ink(message)]
        pub fn block_timestamp(&self) -> Timestamp {
            self.mock_timestamp
        }

        #[ink(message)]
        pub fn set_block_timestamp(&mut self, timestamp: Timestamp) {
            self.mock_timestamp = timestamp;
        }

        #[ink(message)]
        pub fn increase_block_timestamp(&mut self, timestamp: Timestamp) {
            self.mock_timestamp += timestamp;
        }

        #[ink(message)]
        pub fn _count_vote(
            &mut self,
            proposal_id: ProposalId,
            account: AccountId,
            support: VoteType,
            weight: Balance,
        ) -> Result<(), GovernanceError> {
            CountingInternal::_count_vote(self, proposal_id, account, support, weight)
        }

        #[ink(message)]
        pub fn _get_votes(
            &mut self,
            account: AccountId,
            timepoint: Timestamp,
            _params: Vec<u8>,
        ) -> Result<Balance, GovernanceError> {
            GovernorVotesInternal::_get_votes(self, account, timepoint, _params)
        }
    }

    impl TimestampProvider for Contract {
        fn block_timestamp(&self) -> Timestamp {
            self.mock_timestamp
        }
    }
}
