#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// This contract will be used to represent the shares of a user
/// and other instance of this contract will be used to represent
/// the amount of borrowed tokens
#[openbrush::implementation(PSP22, PSP22Metadata, PSP22Mintable, PSP22Burnable, Ownable)]
#[openbrush::contract]
pub mod shares {
    use lending_project::traits::shares::*;
    use openbrush::{
        contracts::ownable::*,
        modifiers,
        traits::{Storage, String},
    };

    /// Define the storage for PSP22 data, Metadata data and Ownable data
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct SharesContract {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    /// override the `mint` function to add the `only_owner` modifier
    #[default_impl(PSP22Mintable)]
    #[modifiers(only_owner)]
    fn mint() {}

    /// override the `burn` function to add the `only_owner` modifier
    #[default_impl(PSP22Burnable)]
    #[modifiers(only_owner)]
    fn burn() {}

    // It forces the compiler to check that you implemented all super traits
    impl Shares for SharesContract {}

    impl SharesContract {
        /// constructor with name and symbol
        #[ink(constructor)]
        pub fn new(name: Option<String>, symbol: Option<String>) -> Self {
            let mut instance = Self::default();
            let caller = Self::env().caller();
            instance.metadata.name.set(&name);
            instance.metadata.symbol.set(&symbol);
            instance.metadata.decimals.set(&18);
            ownable::Internal::_init_with_owner(&mut instance, caller);

            instance
        }
    }
}
