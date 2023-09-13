#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(Pausable)]
#[openbrush::contract]
pub mod my_pausable {
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        pause: pausable::Data,
        flipped: bool,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        #[openbrush::modifiers(when_not_paused)]
        pub fn flip(&mut self) -> Result<(), PausableError> {
            self.flipped = !self.flipped;
            Ok(())
        }

        #[ink(message)]
        pub fn pause(&mut self) -> Result<(), PausableError> {
            Internal::_pause(self)
        }

        #[ink(message)]
        pub fn unpause(&mut self) -> Result<(), PausableError> {
            Internal::_unpause(self)
        }

        #[ink(message)]
        pub fn change_state(&mut self) -> Result<(), PausableError> {
            Internal::_switch_pause(self)
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        #[rustfmt::skip]
        use super::*;

        use test_helpers::{
            method_call,
            method_call_dry_run,
        };
        use ink_e2e::ContractsBackend;


        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn success_flip_when_not_paused<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let contract = client
                .instantiate("my_pausable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let mut call = contract.call::<Contract>();

            assert_eq!(method_call!(client, call, flip()), Ok(()));

            Ok(())
        }

        #[ink_e2e::test]
        async fn success_pause_when_not_paused<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let contract = client
                .instantiate("my_pausable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let mut call = contract.call::<Contract>();

            assert_eq!(method_call!(client, call, pause()), Ok(()));

            Ok(())
        }

        #[ink_e2e::test]
        async fn success_change_state<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let contract = client
                .instantiate("my_pausable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let mut call = contract.call::<Contract>();

            assert_eq!(method_call!(client, call, change_state()), Ok(()));

            Ok(())
        }

        #[ink_e2e::test]
        async fn failed_double_pause<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let contract = client
                .instantiate("my_pausable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let mut call = contract.call::<Contract>();

            assert_eq!(method_call!(client, call, pause()), Ok(()));
            assert!(matches!(method_call_dry_run!(client, call, pause()), Err(_)));

            Ok(())
        }

        #[ink_e2e::test]
        async fn success_pause_and_unpause<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let contract = client
                .instantiate("my_pausable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let mut call = contract.call::<Contract>();

            assert_eq!(method_call!(client, call, pause()), Ok(()));
            assert_eq!(method_call!(client, call, unpause()), Ok(()));

            Ok(())
        }

        #[ink_e2e::test]
        async fn failed_unpause<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let contract = client
                .instantiate("my_pausable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let mut call = contract.call::<Contract>();

            assert!(matches!(method_call_dry_run!(client, call, unpause()), Err(_)));

            Ok(())
        }

        #[ink_e2e::test]
        async fn failed_flip_when_paused<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let contract = client
                .instantiate("my_pausable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let mut call = contract.call::<Contract>();

            assert_eq!(method_call!(client, call, pause()), Ok(()));
            assert!(matches!(method_call_dry_run!(client, call, flip()), Err(_)));

            Ok(())
        }
    }
}
