#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(Ownable2Step, Ownable, PSP37, PSP37Burnable, PSP37Mintable)]
#[openbrush::contract]
pub mod ownable_2_step {
    use openbrush::{
        modifiers,
        traits::{Storage},
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp37: psp37::Data,
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        ownable_2_step: ownable_2_step::Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            ownable::Internal::_init_with_owner(&mut instance, Self::env().caller());
            instance
        }
    }

    #[default_impl(PSP37Mintable)]
    #[modifiers(only_owner)]
    fn mint(&mut self) {}

    #[default_impl(PSP37Burnable)]
    #[modifiers(only_owner)]
    fn burn(&mut self) {}
}
