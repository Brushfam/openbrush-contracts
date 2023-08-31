#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP22)]
#[openbrush::contract]
pub mod my_psp22_facet_v2 {
    use ink::codegen::Env;
    use openbrush::{
        contracts::ownable::*,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct PSP22FacetV2 {
        #[storage_field]
        psp22: psp22::Data,
        // Ownable is used only internally without exposing it to the world
        #[storage_field]
        ownable: ownable::Data,
    }

    #[overrider(PSP22)]
    fn transfer(&mut self, to: AccountId, value: Balance, data: Vec<u8>) -> Result<(), PSP22Error> {
        // we will burn 10% of transfer to and from non-zero accounts
        let burned = value / 10;
        psp22::Internal::_burn_from(self, self.env().caller(), burned)?;
        psp22::Internal::_transfer_from_to(self, self.env().caller(), to, value - burned, data)?;
        Ok(())
    }

    impl PSP22FacetV2 {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }
}
