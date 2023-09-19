#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(Ownable, PSP37, PSP37Burnable, PSP37Mintable)]
#[openbrush::contract]
pub mod ownable {
    use openbrush::{
        modifiers,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp37: psp37::Data,
        #[storage_field]
        ownable: ownable::Data,
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

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use openbrush::contracts::{
            ownable::ownable_external::Ownable,
            psp37::{
                extensions::mintable::psp37mintable_external::PSP37Mintable,
                psp37_external::PSP37,
            },
        };

        #[rustfmt::skip]
        use super::*;

        use test_helpers::{
            address_of,
            method_call_dry_run,
            method_call,
        };
        use ink_e2e::ContractsBackend;


        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn owner_is_by_default_contract_deployer<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let contract = client
                .instantiate("my_ownable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let mut call = contract.call::<Contract>();

            assert_eq!(method_call_dry_run!(client, call, owner()), Some(address_of!(Alice)));

            Ok(())
        }

        #[ink_e2e::test]
        async fn only_owner_is_allowed_to_mint<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let contract = client
                .instantiate("my_ownable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let mut call = contract.call::<Contract>();

            assert_eq!(method_call_dry_run!(client, call, owner()), Some(address_of!(Alice)));


            let mint_tx =             method_call_dry_run!(client, call, mint(address_of!(Bob), vec![(Id::U8(0), 1)]));
            assert_eq!(mint_tx, Ok(()));
            method_call!(client, call, mint(address_of!(Bob), vec![(Id::U8(0), 1)]));

            Ok(())
        }

        #[ink_e2e::test]
        async fn transfer_ownership_works<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let contract = client
                .instantiate("my_ownable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let mut call = contract.call::<Contract>();

            let token = Id::U8(1);
            let ids_amounts = vec![(token.clone(), 123)];

            assert_eq!(method_call_dry_run!(client, call, owner()), Some(address_of!(Alice)));

            let mint_tx = method_call_dry_run!(client, call, bob, mint(address_of!(Bob), vec![(Id::U8(0), 1)]));
            assert!(matches!(mint_tx, Err(_)));

            assert_eq!(method_call_dry_run!(client, call, balance_of(address_of!(Bob), Some(token.clone()))), 0);

            let transfer_ownership_tx = method_call_dry_run!(client, call, transfer_ownership(address_of!(Bob)));
            assert_eq!(transfer_ownership_tx    , Ok(()));
            method_call!(client, call, transfer_ownership(address_of!(Bob)));

            assert_eq!(method_call_dry_run!(client, call, owner()), Some(address_of!(Bob)));

            let mint_tx = method_call_dry_run!(client, call, bob, mint(address_of!(Bob), ids_amounts.clone()));
            assert_eq!(mint_tx, Ok(()));
            method_call!(client, call, bob, mint(address_of!(Bob), ids_amounts.clone()));

            assert_eq!(method_call_dry_run!(client, call, balance_of(address_of!(Bob), Some(token.clone()))), 123);

            Ok(())
        }

        #[ink_e2e::test]
        async fn renounce_ownership_works<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let contract = client
                .instantiate("my_ownable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let mut call = contract.call::<Contract>();

            assert_eq!(method_call_dry_run!(client, call, owner()), Some(address_of!(Alice)));

            let renounce_ownership_tx = method_call_dry_run!(client, call, renounce_ownership());
            assert_eq!(renounce_ownership_tx, Ok(()));
            method_call!(client, call, renounce_ownership());

            assert_eq!(method_call_dry_run!(client, call, owner()), None);

            Ok(())
        }

        #[ink_e2e::test]
        async fn cannot_renounce_ownership_if_not_owner<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let contract = client
                .instantiate("my_ownable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let mut call = contract.call::<Contract>();

            assert_eq!(method_call_dry_run!(client, call, owner()), Some(address_of!(Alice)));

            let renounce_ownership_tx = method_call_dry_run!(client, call, bob, renounce_ownership());
            assert!(matches!(renounce_ownership_tx, Err(_)));

            assert_eq!(method_call_dry_run!(client, call, owner()), Some(address_of!(Alice)));

            Ok(())
        }

        #[ink_e2e::test]
        async fn cannot_transfer_ownership_if_not_owner<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let contract = client
                .instantiate("my_ownable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let mut call = contract.call::<Contract>();

            assert_eq!(method_call_dry_run!(client, call, owner()), Some(address_of!(Alice)));

            let transfer_ownership_tx = method_call_dry_run!(client, call, bob, transfer_ownership(address_of!(Charlie)));
            assert!(matches!(transfer_ownership_tx, Err(_)));

            assert_eq!(method_call_dry_run!(client, call, owner()), Some(address_of!(Alice)));

            Ok(())
        }
    }
}
