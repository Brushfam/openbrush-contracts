#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP61, Ownable, AccessControl, Pausable, Upgradeable)]
#[openbrush::contract]
pub mod my_psp61 {
    use ink::prelude::{
        vec,
        vec::Vec,
    };
    use openbrush::{
        contracts::supported_interfaces,
        traits::{
            Storage,
            String,
        },
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        pub ownable: ownable::Data,
        #[storage_field]
        pub access_control: access_control::Data,
        #[storage_field]
        pub pausable: pausable::Data,
    }

    supported_interfaces!(Contract);

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn ownable_id(&self) -> u32 {
            ownable::ownable_external::TRAIT_ID
        }

        #[ink(message)]
        pub fn access_control_id(&self) -> u32 {
            access_control::accesscontrol_external::TRAIT_ID
        }

        #[ink(message)]
        pub fn pausable_id(&self) -> u32 {
            pausable::pausable_external::TRAIT_ID
        }

        #[ink(message)]
        pub fn upgradeable_id(&self) -> u32 {
            upgradeable::upgradeable_external::TRAIT_ID
        }

        #[ink(message)]
        pub fn psp61_id(&self) -> u32 {
            psp61::psp61_external::TRAIT_ID
        }

        #[ink(message)]
        pub fn id_batch(&self) -> Vec<(String, u32)> {
            vec![
                (String::from("ownable"), ownable::ownable_external::TRAIT_ID),
                (
                    String::from("access_control"),
                    access_control::accesscontrol_external::TRAIT_ID,
                ),
                (String::from("pausable"), pausable::pausable_external::TRAIT_ID),
                (String::from("upgradeable"), upgradeable::upgradeable_external::TRAIT_ID),
                (String::from("psp61"), psp61::psp61_external::TRAIT_ID),
            ]
        }
    }

    #[cfg(test)]
    mod tests {
        use super::Contract;
        use openbrush::contracts::{
            access_control,
            ownable,
            pausable,
            psp61,
            psp61::PSP61,
            upgradeable,
        };

        #[ink::test]
        fn assure_ids_are_proper() {
            let contract = Contract::new();

            assert_eq!(contract.ownable_id(), ownable::ownable_external::TRAIT_ID);
            assert_eq!(
                contract.access_control_id(),
                access_control::accesscontrol_external::TRAIT_ID
            );
            assert_eq!(contract.pausable_id(), pausable::pausable_external::TRAIT_ID);
            assert_eq!(contract.upgradeable_id(), upgradeable::upgradeable_external::TRAIT_ID);
            assert_eq!(contract.psp61_id(), psp61::psp61_external::TRAIT_ID);
        }

        #[ink::test]
        fn check_for_interfaces() {
            let contract = Contract::new();

            assert_eq!(contract.supports_interface(ownable::ownable_external::TRAIT_ID), true);
            assert_eq!(
                contract.supports_interface(access_control::accesscontrol_external::TRAIT_ID),
                true
            );
            assert_eq!(contract.supports_interface(pausable::pausable_external::TRAIT_ID), true);
            assert_eq!(
                contract.supports_interface(upgradeable::upgradeable_external::TRAIT_ID),
                true
            );
            assert_eq!(contract.supports_interface(psp61::psp61_external::TRAIT_ID), true);
        }

        #[ink::test]
        fn check_for_interfaces_batch() {
            let contract = Contract::new();

            let ids = contract.id_batch();
            let mut interfaces = contract.supported_interfaces();

            let mut ids: Vec<_> = ids
                .into_iter()
                .map(|(_, id)| {
                    assert_eq!(contract.supports_interface(id), true);
                    id
                })
                .collect();

            ids.sort_unstable();
            interfaces.sort_unstable();

            assert_eq!(ids, interfaces);
        }

        #[ink::test]
        fn check_for_non_existing_interface() {
            let contract = Contract::new();

            assert_eq!(contract.supports_interface(0), false);

            let interfaces = contract.supported_interfaces();

            interfaces.into_iter().for_each(|id| {
                assert_eq!(contract.supports_interface(id + 1), false);
            });
        }
    }
}
