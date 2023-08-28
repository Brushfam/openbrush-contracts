#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(Nonces)]
#[openbrush::contract]
pub mod my_nonces {
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        nonces: nonces::Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn use_nonce(&mut self, account: AccountId) -> Result<u128, NoncesError> {
            NoncesImpl::_use_nonce(self, &account)
        }

        #[ink(message)]
        pub fn use_checked_nonce(&mut self, account: AccountId, nonce: u128) -> Result<u128, NoncesError> {
            NoncesImpl::_use_checked_nonce(self, &account, nonce)
        }
    }
}
