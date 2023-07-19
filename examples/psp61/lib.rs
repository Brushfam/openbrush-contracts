#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP61, PSP22)]
#[openbrush::contract]
pub mod my_psp61 {
    use openbrush::contracts::supported_interfaces;

    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        pub psp22: psp22::Data,
    }

    supported_interfaces!(
        Contract =>
            psp61_external::TRAIT_ID,
            psp22_external::TRAIT_ID
    );

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn psp22_selector(&self) -> u32 {
            psp22_external::TRAIT_ID
        }
    }
    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut contract = Contract::new(false);
        }
    }
}
