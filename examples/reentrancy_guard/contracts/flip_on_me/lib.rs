#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use openbrush::examples::contracts::reentrancy_guard::flip_on_me::*;

#[openbrush::contract]
pub mod flip_on_me {
    use flipper::traits::{
        flip_on_me::*,
        flipper::*,
    };
    use ink::env::CallFlags;
    use openbrush::traits::DefaultEnv;

    #[ink(storage)]
    #[derive(Default)]
    pub struct FlipOnMeContract {}

    impl FlipOnMeContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }

    impl FlipOnMe for FlipOnMeContract {
        #[ink(message)]
        fn flip_on_target(&mut self, callee: AccountId) -> Result<(), ReentrancyGuardError> {
            // This method does a cross-contract call to caller contract and calls the `flip` method.
            FlipperRef::flip_builder(&callee)
                .call_flags(CallFlags::default().set_allow_reentry(true))
                .invoke()
                .unwrap();
            Ok(())
        }
    }
}
