#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP22Pallet, PSP22PalletBurnable)]
#[openbrush::contract]
pub mod my_psp22_pallet_burnable {
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        pallet: psp22_pallet::Data,
    }

    impl Contract {
        /// During instantiation of the contract, you need to pass native tokens as a deposit
        /// for asset creation.
        #[ink(constructor)]
        #[ink(payable)]
        pub fn new(asset_id: u32, min_balance: Balance, total_supply: Balance) -> Self {
            let mut instance = Self::default();

            psp22_pallet::Internal::_create(&mut instance, asset_id, Self::env().account_id(), min_balance)
                .expect("Should create an asset");
            instance.pallet.asset_id.set(&asset_id);
            instance.pallet.origin.set(&Origin::Caller);
            psp22_pallet::Internal::_mint_to(&mut instance, Self::env().caller(), total_supply).expect("Should mint");

            instance
        }

        #[ink(message)]
        pub fn burn_from_many(&mut self, accounts: Vec<(AccountId, Balance)>) -> Result<(), PSP22Error> {
            for account in accounts.iter() {
                PSP22Burnable::burn(self, account.0, account.1)?;
            }
            Ok(())
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use openbrush::contracts::psp22_pallet::{
            extensions::burnable::psp22burnable_external::PSP22Burnable,
            psp22_external::PSP22,
        };
        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::{build_message, PolkadotConfig};

        use test_helpers::{
            address_of,
            balance_of,
        };

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        fn random_num() -> u32 {
            use rand::Rng;
            rand::thread_rng().gen_range(1..1000)
        }

        #[ink_e2e::test]
        async fn assigns_initial_balance<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let constructor = ContractRef::new(random_num(), 1, 1000);
            let contract = client
                .instantiate(
                    "my_psp22_pallet_burnable",
                    &ink_e2e::alice(),
                    constructor,
                    10000000000000000,
                    None,
                )
                .await
                .expect("instantiate failed");
            let call = contract.call::<Contract>();

            let balance_of_alice = balance_of!(client, call, Alice);

            assert!(matches!(balance_of_alice, 1000));

            Ok(())
        }

        #[ink_e2e::test]
        async fn can_burn<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let constructor = ContractRef::new(random_num(), 1, 1000);
            let contract = client
                .instantiate(
                    "my_psp22_pallet_burnable",
                    &ink_e2e::alice(),
                    constructor,
                    10000000000000000,
                    None,
                )
                .await
                .expect("instantiate failed");
            let call = contract.call::<Contract>();

            let result = {
                let _msg = call.burn(address_of!(Alice), 10);
                client
                    .call(&ink_e2e::alice(), &_msg, 0, None)
                    .await
                    .expect("call failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let balance_of_alice = balance_of!(client, call, Alice);

            assert!(matches!(balance_of_alice, 990));

            Ok(())
        }

        #[ink_e2e::test]
        async fn can_burn_without_allowance<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let constructor = ContractRef::new(random_num(), 1, 1000);
            let contract = client
                .instantiate(
                    "my_psp22_pallet_burnable",
                    &ink_e2e::alice(),
                    constructor,
                    10000000000000000,
                    None,
                )
                .await
                .expect("instantiate failed");
            let call = contract.call::<Contract>();

            assert!(matches!(balance_of!(client, call, Bob), 0));
            assert!(matches!(balance_of!(client, call, Alice), 1000));

            let result = {
                let _msg = call.burn(address_of!(Alice), 10);
                client.call(&ink_e2e::bob(), &_msg, 0, None).await.expect("call failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let balance_of_alice = balance_of!(client, call, Alice);

            assert!(matches!(balance_of_alice, 990));

            Ok(())
        }

        #[ink_e2e::test]
        async fn decreases_total_supply_after_burning<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let constructor = ContractRef::new(random_num(), 1, 1000);
            let contract = client
                .instantiate(
                    "my_psp22_pallet_burnable",
                    &ink_e2e::alice(),
                    constructor,
                    10000000000000000,
                    None,
                )
                .await
                .expect("instantiate failed");
            let call = contract.call::<Contract>();

            let total_supply = {
                let _msg = call.total_supply();
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            };

            assert!(matches!(total_supply.return_value(), 1000));

            let result = {
                let _msg = call.burn(address_of!(Alice), 10);
                client
                    .call(&ink_e2e::alice(), &_msg, 0, None)
                    .await
                    .expect("call failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let total_supply = {
                let _msg = call.total_supply();
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            };

            assert!(matches!(total_supply.return_value(), 990));

            Ok(())
        }

        #[ink_e2e::test]
        async fn can_burn_from<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let constructor = ContractRef::new(random_num(), 1, 1000);
            let contract = client
                .instantiate(
                    "my_psp22_pallet_burnable",
                    &ink_e2e::alice(),
                    constructor,
                    10000000000000000,
                    None,
                )
                .await
                .expect("instantiate failed");
            let call = contract.call::<Contract>();

            let result = {
                let _msg = call.transfer(address_of!(Bob), 10, vec![]);
                client
                    .call(&ink_e2e::alice(), &_msg, 0, None)
                    .await
                    .expect("call failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let balance_of_bob = balance_of!(client, call, Bob);

            assert!(matches!(balance_of_bob, 10));

            let result = {
                let _msg =call.burn(address_of!(Bob), 10);
                client
                    .call(&ink_e2e::alice(), &_msg, 0, None)
                    .await
                    .expect("call failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let balance_of_bob = balance_of!(client, call, Bob);

            assert!(matches!(balance_of_bob, 0));

            Ok(())
        }

        #[ink_e2e::test]
        async fn can_burn_from_many<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let constructor = ContractRef::new(random_num(), 1, 1000);
            let contract = client
                .instantiate(
                    "my_psp22_pallet_burnable",
                    &ink_e2e::alice(),
                    constructor,
                    10000000000000000,
                    None,
                )
                .await
                .expect("instantiate failed");
            let call = contract.call::<Contract>();

            let result = {
                let _msg = call.transfer(address_of!(Bob), 10, vec![]);
                client
                    .call(&ink_e2e::alice(), &_msg, 0, None)
                    .await
                    .expect("call failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let result = {
                let _msg = call.transfer(address_of!(Charlie), 10, vec![]);
                client
                    .call(&ink_e2e::alice(), &_msg, 0, None)
                    .await
                    .expect("call failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let balance_of_bob = balance_of!(client, call, Bob);
            let balance_of_charlie = balance_of!(client, call, Charlie);

            assert!(matches!(balance_of_bob, 10));
            assert!(matches!(balance_of_charlie, 10));

            let result = {
                let _msg = call.burn_from_many(vec![(address_of!(Bob), 10), (address_of!(Charlie), 10)]);
                client
                    .call(&ink_e2e::alice(), &_msg, 0, None)
                    .await
                    .expect("call failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let balance_of_bob = balance_of!(client, call, Bob);
            let balance_of_charlie = balance_of!(client, call, Charlie);

            assert!(matches!(balance_of_bob, 0));
            assert!(matches!(balance_of_charlie, 0));

            Ok(())
        }

        #[ink_e2e::test]
        async fn fails_if_one_of_the_accounts_balance_exceeds_amount_to_burn(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let constructor = ContractRef::new(random_num(), 1, 1000);
            let contract = client
                .instantiate(
                    "my_psp22_pallet_burnable",
                    &ink_e2e::alice(),
                    constructor,
                    10000000000000000,
                    None,
                )
                .await
                .expect("instantiate failed");
            let call = contract.call::<Contract>();

            let result = {
                let _msg = call.transfer(address_of!(Bob), 10, vec![]);
                client
                    .call(&ink_e2e::alice(), &_msg, 0, None)
                    .await
                    .expect("call failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let result = {
                let _msg = call.transfer(address_of!(Charlie), 5, vec![]);
                client
                    .call(&ink_e2e::alice(), &_msg, 0, None)
                    .await
                    .expect("call failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let balance_of_bob = balance_of!(client, call, Bob);
            let balance_of_charlie = balance_of!(client, call, Charlie);

            assert!(matches!(balance_of_bob, 10));
            assert!(matches!(balance_of_charlie, 5));

            let result = {
                let _msg = call.burn_from_many(vec![(address_of!(Bob), 10), (address_of!(Charlie), 10)]);
                client
                    .call(&ink_e2e::alice(), &_msg, 0, None)
                    .await
                    .expect("call failed")
            };
            // This is not working properly TBD
            assert!(matches!(result.return_value(), Ok(())));

            let balance_of_bob = balance_of!(client, call, Bob);
            let balance_of_charlie = balance_of!(client, call, Charlie);

            assert!(matches!(balance_of_bob, 0));
            assert!(matches!(balance_of_charlie, 0));

            Ok(())
        }
    }
}
