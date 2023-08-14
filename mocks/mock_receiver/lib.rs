#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod mock_receiver {
    #[ink(storage)]
    pub struct MockReceiver {}

    impl MockReceiver {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
        }

        #[ink(message)]
        pub fn mock_function(&mut self) -> u32 {
            1234321
        }
    }
}
