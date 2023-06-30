#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::implementation(PSP22Metadata)]
#[openbrush::contract]
pub mod my_psp22_metadata_facet {
    use openbrush::{
        contracts::ownable::*,
        modifiers,
        traits::{
            Storage,
            String,
        },
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct PSP22Facet {
        #[storage_field]
        metadata: metadata::Data,
        // Ownable is used only internally without exposing it to the world
        #[storage_field]
        ownable: ownable::Data,
    }

    impl PSP22Facet {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn init_metadata(&mut self) -> Result<(), PSP22Error> {
            self.metadata.name.set(&Some(String::from("PSP22 Diamond")));
            self.metadata.symbol.set(&Some(String::from("PSP22D")));
            self.metadata.decimals.set(&18);
            Ok(())
        }
    }
}
