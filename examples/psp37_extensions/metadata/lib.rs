#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP37, PSP37Metadata)]
#[openbrush::contract]
pub mod my_psp37 {
    use openbrush::traits::{
        Storage,
        String,
    };

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp37: psp37::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    impl Contract {
        /// contract constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn set_attribute(&mut self, id: Id, key: String, data: String) -> Result<(), PSP37Error> {
            metadata::Internal::_set_attribute(self, &id, &key, &data)
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use openbrush::contracts::psp37::extensions::metadata::psp37metadata_external::PSP37Metadata;

        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::{build_message};

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn metadata_works<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let contract = client
                .instantiate("my_psp37_metadata", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let call = contract.call::<Contract>();

            let id = Id::U8(0);
            let attr = String::from("https://www.727.ventures/");

            let attribute = {
                let _msg = call.get_attribute(id.clone(), attr.clone());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(attribute, None);

            let set_attribute_tx = {
                let _msg = call.set_attribute(id.clone(), attr.clone(), String::from("https://www.727.ventures/"));
                client
                    .call(&ink_e2e::alice(), &_msg, 0, None)
                    .await
                    .expect("call failed")
            }
            .return_value();

            assert_eq!(set_attribute_tx, Ok(()));

            let attribute = {
                let _msg = call.get_attribute(id.clone(), attr.clone());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(attribute, Some(String::from("https://www.727.ventures/")));

            Ok(())
        }
    }
}
