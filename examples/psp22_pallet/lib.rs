#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP22Pallet)]
#[openbrush::contract]
pub mod my_psp22_pallet {
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

        /// Asset id of the asset in the `pallet-assets`
        #[ink(message)]
        pub fn asset_id(&self) -> u32 {
            self.pallet.asset_id.get_or_default()
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use openbrush::contracts::psp22_pallet::psp22_external::PSP22;
        #[rustfmt::skip]
        use super::*;

        use test_helpers::{
            address_of,
            balance_of,
        };
        use ink_e2e::ContractsBackend;

        fn random_num() -> u32 {
            use rand::Rng;
            rand::thread_rng().gen_range(1..1000)
        }

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn assigns_initial_balance<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let constructor = ContractRef::new(random_num(), 1, 100);
            let contract = client
                .instantiate(
                    "my_psp22_pallet",
                    &ink_e2e::alice(),
                    constructor,
                    10000000000000000,
                    None,
                )
                .await
                .expect("instantiate failed");
            let mut call = contract.call::<Contract>();

            let result = {
                let _msg = call.balance_of(address_of!(Alice));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            };

            assert!(matches!(result.return_value(), 100));

            Ok(())
        }

        #[ink_e2e::test]
        async fn transfer_adds_amount_to_destination_account<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let constructor = ContractRef::new(random_num(), 1, 100);
            let contract = client
                .instantiate(
                    "my_psp22_pallet",
                    &ink_e2e::alice(),
                    constructor,
                    10000000000000000,
                    None,
                )
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
            let constructor = ContractRef::new(random_num(), 1, 100);
            let contract = client
                .instantiate(
                    "my_psp22_pallet",
                    &ink_e2e::alice(),
                    constructor,
                    10000000000000000,
                    None,
                )
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
    }
}
