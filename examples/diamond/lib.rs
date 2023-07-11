#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(Ownable, Diamond, DiamondLoupe)]
#[openbrush::contract]
pub mod diamond {
    use openbrush::{
        modifiers,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default)]
    #[openbrush::storage]
    pub struct Contract {
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        diamond: diamond::Data,
        #[storage_field]
        loupe: diamond_loupe::Data,
    }

    #[default_impl(Diamond)]
    #[modifiers(only_owner)]
    fn diamond_cut() {}

    impl Contract {
        #[ink(constructor)]
        pub fn new(owner: AccountId) -> Self {
            let mut instance = Self::default();
            ownable::Internal::_init_with_owner(&mut instance, owner);

            instance
        }

        #[ink(message, payable, selector = _)]
        pub fn forward(&self) {
            diamond::Internal::_fallback(self)
        }
    }
}
