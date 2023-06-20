#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP22Pallet, PSP22PalletMetadata)]
#[openbrush::contract]
pub mod my_psp22_pallet_metadata {
    use psp22_pallet::{
        DefaultEnvironment,
        Error,
    };

    #[ink(storage)]
    #[derive(Default)]
    #[openbrush::storage]
    pub struct Contract {
        #[storage_field]
        pallet: psp22_pallet::Data,
    }

    impl Contract {
        /// During instantiation of the contract, you need to pass native tokens as a deposit
        /// for asset creation.
        #[ink(constructor)]
        #[ink(payable)]
        pub fn new(
            asset_id: u32,
            min_balance: Balance,
            total_supply: Balance,
            name: String,
            symbol: String,
            decimal: u8,
        ) -> Self {
            let mut instance = Self::default();

            psp22_pallet::Internal::_create(&mut instance, asset_id, Self::env().account_id(), min_balance)
                .expect("Should create an asset");
            instance.pallet.asset_id = asset_id;
            instance.pallet.origin = Origin::Caller;
            instance
                .pallet
                .pallet_assets
                .set_metadata(asset_id, name.into(), symbol.into(), decimal)
                .expect("Should set metadata");
            psp22_pallet::Internal::_mint_to(&mut instance, Self::env().caller(), total_supply).expect("Should mint");

            instance
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use openbrush::contracts::psp22_pallet::{
            extensions::{
                metadata::psp22metadata_external::PSP22Metadata,
            },
        };

        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::{build_message, PolkadotConfig};


        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        fn random_num() -> u32 {
            use rand::Rng;
            rand::thread_rng().gen_range(1..1000)
        }

        #[ink_e2e::test]
        async fn metadata_works(client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let _name = String::from("TOKEN");
            let _symbol = String::from("TKN");

            let constructor = ContractRef::new(random_num(), 1, 1000, _name, _symbol, 18);
            let address = client
                .instantiate(
                    "my_psp22_pallet_metadata",
                    &ink_e2e::alice(),
                    constructor,
                    1000000000000000000,
                    None,
                )
                .await
                .expect("instantiate failed")
                .account_id;

            let token_name = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.token_name());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            let token_symbol = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.token_symbol());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            let token_decimals = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.token_decimals());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(token_name, Some(_name)));
            assert!(matches!(token_symbol, Some(_symbol)));
            assert!(matches!(token_decimals, 18));

            Ok(())
        }
    }
}
