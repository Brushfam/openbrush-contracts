#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::contract]
pub mod my_flipper_guard {
    use flipper::traits::flipper::*;
    use flipper::traits::flip_on_me::*;
    use openbrush::{
        modifiers,
        traits::Storage,
    };
    use ink::env::CallFlags;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct MyFlipper {
        #[storage_field]
        guard: reentrancy_guard::Data,
        value: bool,
    }

    impl MyFlipper {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }

    impl Flipper for MyFlipper {
        #[ink(message)]
        fn get_value(&self) -> bool {
            self.value
        }

        #[ink(message)]
        #[openbrush::modifiers(non_reentrant)]
        fn flip(&mut self) -> Result<(), ReentrancyGuardError> {
            self.value = !self.value;
            Ok(())
        }

        #[ink(message)]
        #[modifiers(non_reentrant)]
        fn call_flip_on_me(&mut self, callee: AccountId) -> Result<(), ReentrancyGuardError> {
            // This method will do a cross-contract call to callee account. It calls method `flip_on_me`.
            // Callee contract during execution of `flip_on_me` will call `flip` of this contract.
            // `call_flip_on_me` and `flip` are marked with `non_reentrant` modifier. It means,
            // that call of `flip` after `call_flip_on_me` must fail.
            FlipOnMeRef::flip_on_me_builder(&callee)
                .call_flags(CallFlags::default().set_allow_reentry(true))
                .invoke()
                .unwrap();
            Ok(())
        }
    }
}
