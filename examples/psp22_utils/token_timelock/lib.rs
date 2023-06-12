#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::implementation(PSP22TokenTimelock)]
#[openbrush::contract]
pub mod my_psp22_token_timelock {
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        timelock: token_timelock::Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(token_address: AccountId, beneficiary: AccountId, release_time: Timestamp) -> Self {
            let mut instance = Self::default();

            token_timelock::Internal::_init(&mut instance, token_address, beneficiary, release_time)
                .expect("Should init");

            instance
        }
    }
}
