#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::contract]
pub mod my_lazy_contract {
    use ink::prelude::vec::Vec;
    use openbrush::traits::{Storage, String};

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        data: Data,
    }

    #[derive(Default, Debug)]
    #[openbrush::storage_item]
    pub struct Data {
        #[lazy]
        pub value1: u128,
        pub value2: u128,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            instance
        }

        #[ink(message)]
        pub fn get_value1(&self) -> u128 {
            self.data.value1.get_or_default()
        }

        #[ink(message)]
        pub fn get_value2(&self) -> u128 {
            self.data.value2
        }

        #[ink(message)]
        pub fn set_value1(&mut self, value: u128) {
            self.data.value1.set(&value);
        }

        #[ink(message)]
        pub fn set_value2(&mut self, value: u128) {
            self.data.value2 = value;
        }
    }
}
