#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP22)]
#[openbrush::contract]
pub mod my_psp22_facet_v2 {
    use ink::codegen::Env;
    use openbrush::{
        contracts::ownable::*,
        traits::ZERO_ADDRESS,
    };

    #[ink(storage)]
    #[derive(Default)]
    #[openbrush::storage]
    pub struct PSP22FacetV2 {
        #[storage_field]
        psp22: psp22::Data,
        // Ownable is used only internally without exposing it to the world
        #[storage_field]
        ownable: ownable::Data,
    }

    #[overrider(PSP22)]
    fn transfer(&mut self, to: AccountId, value: Balance, data: Vec<u8>) -> Result<(), PSP22Error> {
        let from = self.env().caller();
        let is_tax = to != ZERO_ADDRESS.into() && from != ZERO_ADDRESS.into();
        // we will burn 10% of transfer to and from non-zero accounts
        let burned = if is_tax { value / 10 } else { 0 };
        if is_tax {
            psp22::Internal::_burn_from(self, from, burned)?;
        }
        psp22::Internal::_transfer_from_to(self, from, to, value - burned, data)?;
        Ok(())
    }

    impl PSP22FacetV2 {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }
}
