#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::implementation(Ownable, PSP22)]
#[openbrush::contract]
pub mod my_psp22_upgradeable {
    use openbrush::modifiers;

    #[ink(storage)]
    #[derive(Default)]
    #[openbrush::storage]
    pub struct MyPSP22 {
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        psp22: psp22::Data,
    }

    impl MyPSP22 {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self::default();

            ownable::Internal::_init_with_owner(&mut instance, Self::env().caller());
            instance.initialize(total_supply).ok().unwrap();

            instance
        }

        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn initialize(&mut self, total_supply: Balance) -> Result<(), OwnableError> {
            if let Some(owner) = Ownable::owner(self) {
                psp22::Internal::_mint_to(self, owner, total_supply).expect("Should mint");
            }
            Ok(())
        }
    }
}
