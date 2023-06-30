#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// This is a simple `PSP22` which will be used as a stable coin and a collateral token in our lending contract
#[openbrush::implementation(PSP22, PSP22Metadata, PSP22Mintable)]
#[openbrush::contract]
pub mod token {
    use lending_project::traits::stable_coin::*;
    use openbrush::traits::{
        Storage,
        String,
    };

    /// Define the storage for PSP22 data and Metadata data
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct StableCoinContract {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    // It forces the compiler to check that you implemented all super traits
    impl StableCoin for StableCoinContract {}

    impl StableCoinContract {
        /// Constructor with name and symbol
        #[ink(constructor)]
        pub fn new(name: Option<String>, symbol: Option<String>) -> Self {
            let mut instance = Self::default();

            instance.metadata.name = name;
            instance.metadata.symbol = symbol;
            instance.metadata.decimals = 18;
            let total_supply = 1_000_000 * 10_u128.pow(18);
            assert!(psp22::Internal::_mint_to(&mut instance, Self::env().caller(), total_supply).is_ok());

            instance
        }
    }
}
