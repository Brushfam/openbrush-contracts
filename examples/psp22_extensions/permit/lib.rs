#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP22, PSP22Permit)]
#[openbrush::contract]
pub mod my_psp22_burnable {
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        psp22_permit: psp22::extensions::permit::Data,
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
