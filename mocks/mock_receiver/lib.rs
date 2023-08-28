#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod mock_receiver {
    #[ink(storage)]
    pub struct MockReceiver {
        pub some_value: u32,
    }

    impl MockReceiver {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { some_value: 0 }
        }
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
        }

        #[ink(message)]
        pub fn mock_function(&mut self) -> u32 {
            self.some_value = self.some_value + 1;
            self.some_value
        }
    }
}
