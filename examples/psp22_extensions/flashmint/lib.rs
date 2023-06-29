#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![feature(default_alloc_error_handler)]

#[openbrush::implementation(PSP22, Flashmint)]
#[openbrush::contract]
pub mod my_psp22_flashmint {
    #[ink(storage)]
    #[derive(Default)]
    #[openbrush::storage]
    pub struct Contract {
        #[storage_field]
        psp22: psp22::Data,
    }

    /// Override `get_fee` function to add 1% fee to the borrowed `amount`
    #[overrider(flashmint::Internal)]
    fn _get_fee(&self, amount: Balance) -> Balance {
        amount / 100
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self::default();
            psp22::Internal::_mint_to(&mut instance, Self::env().caller(), total_supply).expect("Should mint");

            instance
        }
    }
}
