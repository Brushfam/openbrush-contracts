#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP22, PSP22Metadata)]
#[openbrush::contract]
pub mod my_psp22 {
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(total_supply: Balance, name: Option<String>, symbol: Option<String>, decimal: u8) -> Self {
            let mut instance = Self::default();
            let caller = instance.env().caller();

            instance.metadata.name.set(&name);
            instance.metadata.symbol.set(&symbol);
            instance.metadata.decimals.set(&decimal);

            psp22::Internal::_mint_to(&mut instance, caller, total_supply).expect("Should mint total_supply");

            instance
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use openbrush::contracts::psp22::extensions::metadata::psp22metadata_external::PSP22Metadata;

        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::build_message;

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn metadata_works(client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let _name = String::from("TOKEN");
            let _symbol = String::from("TKN");

            let constructor = ContractRef::new(1000, Some(_name), Some(_symbol), 18);
            let address = client
                .instantiate("my_psp22_metadata", &ink_e2e::alice(), constructor, 0, None)
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
