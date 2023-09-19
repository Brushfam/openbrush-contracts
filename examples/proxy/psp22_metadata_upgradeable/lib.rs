#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(Ownable, PSP22, PSP22Metadata)]
#[openbrush::contract]
pub mod my_psp22 {
    use openbrush::{modifiers, traits::Storage};

    #[ink(storage)]
    #[derive(Default, Storage)]
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
        ) -> Result<(), PSP22Error> {
            self.metadata.name.set(&name);
            self.metadata.symbol.set(&symbol);
            self.metadata.decimals.set(&decimal);
            if let Some(owner) = Ownable::owner(self) {
                psp22::Internal::_mint_to(self, owner, total_supply)
            } else {
                Err(PSP22Error::Custom(String::from("Owner not set!")))
            }
        }
    }
}
