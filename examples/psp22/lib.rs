#![cfg_attr(not(feature = "std"), no_std, no_main)]

// pub use my_psp22::*;
pub use openbrush::traits::{
    AccountId,
    Storage,
};

// we need to expand this struct before the contract macro is expanded
// that is why we declare it here for this example
#[ink::storage_item]
#[openbrush::accessors(HatedStorageAccessors)]
#[derive(Debug)]
pub struct HatedStorage {
    #[get]
    #[set]
    pub hated_account: AccountId,
}

#[openbrush::implementation(PSP22)]
#[openbrush::contract]
pub mod my_psp22 {
    use crate::*;
    use openbrush::traits::String;

    #[ink(storage)]
    #[derive(Storage)]
    pub struct Contract {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        hated_storage: HatedStorage,
    }

    #[overrider(psp22::Internal)]
    fn _before_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        to: Option<&AccountId>,
        _amount: &Balance,
    ) -> Result<(), PSP22Error> {
        if to == Some(&self.hated_storage.hated_account) {
            return Err(PSP22Error::Custom(String::from("I hate this account!")))
        }
        Ok(())
    }

    impl HatedStorageAccessors for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self {
                psp22: Default::default(),
                hated_storage: HatedStorage {
                    hated_account: [255; 32].into(),
                },
            };

            Internal::_mint_to(&mut instance, Self::env().caller(), total_supply).expect("Should mint");

            instance
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use super::*;
        use crate::hatedstorageaccessors_external::HatedStorageAccessors;
        use ink_e2e::ContractsBackend;
        use openbrush::contracts::psp22::psp22_external::PSP22;
        use test_helpers::{
            address_of,
            balance_of,
        };

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn assigns_initial_balance<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let constructor = ContractRef::new(100);
            let contract = client
                .instantiate("my_psp22", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let call = contract.call::<Contract>();

            let result = {
                let balance_of = call.balance_of(address_of!(Alice));
                client.call_dry_run(&ink_e2e::alice(), &balance_of, 0, None).await
            };

            assert!(matches!(result.return_value(), 100));

            Ok(())
        }

        #[ink_e2e::test]
        async fn transfer_adds_amount_to_destination_account<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let constructor = ContractRef::new(100);
            let contract = client
                .instantiate("my_psp22", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let mut call = contract.call::<Contract>();

            let result = {
                let _msg = call.transfer(address_of!(Bob), 50, vec![]);
                client
                    .call(&ink_e2e::alice(), &_msg, 0, None)
                    .await
                    .expect("transfer failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let balance_of_alice = balance_of!(client, call, Alice);

            let balance_of_bob = balance_of!(client, call, Bob);

            assert_eq!(balance_of_bob, 50, "Bob should have 50 tokens");
            assert_eq!(balance_of_alice, 50, "Alice should have 50 tokens");

            Ok(())
        }

        #[ink_e2e::test]
        async fn cannot_transfer_above_the_amount<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let constructor = ContractRef::new(100);
            let contract = client
                .instantiate("my_psp22", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let mut call = contract.call::<Contract>();

            let result = {
                let _msg = call.transfer(address_of!(Bob), 101, vec![]);
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            };

            assert!(matches!(result.return_value(), Err(PSP22Error::InsufficientBalance)));

            Ok(())
        }

        #[ink_e2e::test]
        async fn cannot_transfer_to_hated_account<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let constructor = ContractRef::new(100);
            let contract = client
                .instantiate("my_psp22", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let mut call = contract.call::<Contract>();

            let result = {
                let _msg = call.transfer(address_of!(Bob), 10, vec![]);
                client
                    .call(&ink_e2e::alice(), &_msg, 0, None)
                    .await
                    .expect("transfer failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let balance_of_bob = balance_of!(client, call, Bob);

            assert!(matches!(balance_of_bob, 10));

            let result = {
                let _msg = call.set_hated_account(address_of!(Bob));
                client
                    .call(&ink_e2e::alice(), &_msg, 0, None)
                    .await
                    .expect("set_hated_account failed")
            };

            assert!(matches!(result.return_value(), ()));

            let result = {
                let _msg = call.transfer(address_of!(Bob), 10, vec![]);
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            };

            assert!(matches!(result.return_value(), Err(PSP22Error::Custom(_))));

            let balance_of_bob = balance_of!(client, call, Bob);

            assert!(matches!(balance_of_bob, 10));

            Ok(())
        }
    }
}
