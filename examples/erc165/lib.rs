#![cfg_attr(not(feature = "std"), no_std, no_main)]

// #[openbrush::implementation()]
#[openbrush::contract]
pub mod my_erc165 {
    use openbrush::contracts::erc165;
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        erc165: erc165::Data,
    }

    impl erc165::ERC165Internal for Contract {}

    impl erc165::ERC165InternalOB for Contract {}

    impl erc165::ERC165Impl for Contract {}

    impl erc165::ERC165 for Contract {
        #[ink(message)]
        fn supports_interface(&self, interface_id: u32) -> bool {
            erc165::ERC165Impl::supports_interface(self, interface_id)
        }
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();

            erc165::ERC165Impl::init(&mut instance);

            instance
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
            let mut asda = Asda::new(false);
        }
    }
}
