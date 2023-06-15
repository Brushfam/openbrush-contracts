#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::implementation(Ownable, PSP22, PSP22Metadata)]
#[openbrush::contract]
pub mod my_psp22 {
    use openbrush::{
        modifiers,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default)]
    #[openbrush::storage]
    pub struct MyPSP22 {
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    impl MyPSP22 {
        #[ink(constructor)]
        pub fn new(total_supply: Balance, name: Option<String>, symbol: Option<String>, decimal: u8) -> Self {
            let mut instance = Self::default();

            ownable::Internal::_init_with_owner(&mut instance, Self::env().caller());
            instance.initialize(total_supply, name, symbol, decimal).ok().unwrap();

            instance
        }

        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn initialize(
            &mut self,
            total_supply: Balance,
            name: Option<String>,
            symbol: Option<String>,
            decimal: u8,
        ) -> Result<(), OwnableError> {
            self.metadata.name = name;
            self.metadata.symbol = symbol;
            self.metadata.decimals = decimal;
            psp22::Internal::_mint_to(self, Ownable::owner(self), total_supply).expect("Should mint");
            Ok(())
        }
    }
}
