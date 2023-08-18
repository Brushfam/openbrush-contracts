#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP22, PSP22Mintable, PSP22Votes, Nonces)]
#[openbrush::contract]
pub mod my_psp22_votes {
    // use openbrush::{
    //     contracts::{
    //         governance::utils::{
    //             votes,
    //             votes::*,
    //         },
    //         governor::TimestampProvider,
    //         psp22::extensions::votes::*,
    //         traits::{
    //             errors::GovernanceError,
    //             governance::utils::*,
    //             psp22::{
    //                 extensions::votes::*,
    //                 *,
    //             },
    //             types::SignatureType,
    //         },
    //         utils::checkpoint::Checkpoint,
    //     },
    //     traits::Storage,
    // };
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        votes: votes::Data,
        #[storage_field]
        nonces: nonces::Data,
        mock_timestamp: Timestamp,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self::default();

            psp22::Internal::_mint_to(&mut instance, Self::env().caller(), total_supply).expect("Should mint");

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
    }
    impl TimestampProvider for Contract {
        fn block_timestamp(&self) -> Timestamp {
            self.mock_timestamp
        }
    }
}
