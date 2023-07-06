#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP22)]
#[openbrush::contract]
pub mod my_psp22_facet_v1 {
    use openbrush::{
        contracts::ownable::*,
        modifiers,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct PSP22FacetV1 {
        #[storage_field]
        psp22: psp22::Data,
        // Ownable is used only internally without exposing it to the world
        #[storage_field]
        ownable: ownable::Data,
    }

    impl PSP22FacetV1 {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn init_psp22(&mut self) -> Result<(), PSP22Error> {
            psp22::Internal::_mint_to(self, Self::env().caller(), 1000)
        }
    }
}
