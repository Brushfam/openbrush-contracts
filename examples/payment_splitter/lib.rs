#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
pub mod my_payment_splitter {
    use ink::prelude::vec::Vec;
    use openbrush::{
        contracts::payment_splitter::*,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        splitter: payment_splitter::Data,
    }

    impl payment_splitter::InternalImpl for Contract {}

    impl payment_splitter::Internal for Contract {
        fn _emit_payee_added_event(&self, account: AccountId, shares: Balance) {
            payment_splitter::InternalImpl::_emit_payee_added_event(self, account, shares)
        }

        fn _emit_payment_received_event(&self, from: AccountId, amount: Balance) {
            payment_splitter::InternalImpl::_emit_payment_received_event(self, from, amount)
        }

        fn _emit_payment_released_event(&self, to: AccountId, amount: Balance) {
            payment_splitter::InternalImpl::_emit_payment_released_event(self, to, amount)
        }

        fn _init(&mut self, payees_and_shares: Vec<(AccountId, Balance)>) -> Result<(), PaymentSplitterError> {
            payment_splitter::InternalImpl::_init(self, payees_and_shares)
        }

        fn _add_payee(&mut self, payee: AccountId, share: Balance) -> Result<(), PaymentSplitterError> {
            payment_splitter::InternalImpl::_add_payee(self, payee, share)
        }

        fn _release_all(&mut self) -> Result<(), PaymentSplitterError> {
            payment_splitter::InternalImpl::_release_all(self)
        }

        fn _release(&mut self, account: AccountId) -> Result<(), PaymentSplitterError> {
            payment_splitter::InternalImpl::_release(self, account)
        }
    }

    impl PaymentSplitterImpl for Contract {}

    impl PaymentSplitter for Contract {
        #[ink(message)]
        fn total_shares(&self) -> Balance {
            PaymentSplitterImpl::total_shares(self)
        }

        #[ink(message)]
        fn total_released(&self) -> Balance {
            PaymentSplitterImpl::total_released(self)
        }

        #[ink(message)]
        fn shares(&self, account: AccountId) -> Balance {
            PaymentSplitterImpl::shares(self, account)
        }

        #[ink(message)]
        fn released(&self, account: AccountId) -> Balance {
            PaymentSplitterImpl::released(self, account)
        }

        #[ink(message)]
        fn payee(&self, index: u32) -> AccountId {
            PaymentSplitterImpl::payee(self, index)
        }

        #[ink(message)]
        fn receive(&mut self) {
            PaymentSplitterImpl::receive(self)
        }

        #[ink(message)]
        fn release(&mut self, account: AccountId) -> Result<(), PaymentSplitterError> {
            PaymentSplitterImpl::release(self, account)
        }
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(payees_and_shares: Vec<(AccountId, Balance)>) -> Self {
            let mut instance = Self::default();
            payment_splitter::Internal::_init(&mut instance, payees_and_shares).expect("Should init");
            instance
        }

        /// Payout all payees at once.
        /// Delete this method if you don't want this functionality in your version of the payment splitter.
        #[ink(message)]
        pub fn release_all(&mut self) -> Result<(), PaymentSplitterError> {
            // `_release_all()` is an internal method defined by the `payment_splitter::Internal` trait
            payment_splitter::Internal::_release_all(self)
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use openbrush::contracts::payment_splitter::paymentsplitter_external::PaymentSplitter;

        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::{build_message, PolkadotConfig};
        use ink_e2e::Client;

        use test_helpers::{
            address_of,
            get_shares,
            method_call,
        };

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn init_values(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(vec![(address_of!(bob), 40), (address_of!(alice), 60)]);
            let address = client
                .instantiate("my_payment_splitter", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let bob_shares = get_shares!(client, address, bob);

            assert_eq!(bob_shares, 40);

            let alice_shares = get_shares!(client, address, alice);

            assert_eq!(alice_shares, 60);

            let total_shares = method_call!(client, address, total_shares);

            assert_eq!(total_shares, 100);

            let payee_0 = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.payee(0));
                client
                    .call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
                    .return_value()
            };

            assert_eq!(payee_0, address_of!(bob));

            let payee_1 = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.payee(1));
                client
                    .call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
                    .return_value()
            };

            assert_eq!(payee_1, address_of!(alice));

            Ok(())
        }

        #[ink_e2e::test]
        async fn release_native_token(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(vec![(address_of!(bob), 40), (address_of!(alice), 60)]);
            let address = client
                .instantiate("my_payment_splitter", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let total_released_before = method_call!(client, address, total_released);

            assert_eq!(total_released_before, 0);

            let receive_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.receive());
                client
                    .call(&ink_e2e::alice(), _msg, 1000000000000, None)
                    .await
                    .expect("call failed")
                    .return_value()
            };

            let release_bob = {
                let _msg =
                    build_message::<ContractRef>(address.clone()).call(|contract| contract.release(address_of!(bob)));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
                    .return_value()
            };

            assert!(release_bob.is_ok());

            let release_alice = {
                let _msg =
                    build_message::<ContractRef>(address.clone()).call(|contract| contract.release(address_of!(alice)));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
                    .return_value()
            };

            assert!(release_alice.is_ok());

            let total_released = method_call!(client, address, total_released);

            let bob_released = {
                let _msg =
                    build_message::<ContractRef>(address.clone()).call(|contract| contract.released(address_of!(bob)));
                client
                    .call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
                    .return_value()
            };

            let alice_released = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.released(address_of!(alice)));
                client
                    .call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
                    .return_value()
            };

            assert!(alice_released > bob_released);

            assert_eq!(bob_released, (total_released * 40) / 100);

            assert_eq!(alice_released, (total_released * 60) / 100);

            assert_eq!(alice_released + bob_released, total_released);

            Ok(())
        }

        #[ink_e2e::test]
        async fn release_native_token_using_release_all(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(vec![(address_of!(bob), 40), (address_of!(alice), 60)]);
            let address = client
                .instantiate("my_payment_splitter", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let total_released_before = method_call!(client, address, total_released);

            assert_eq!(total_released_before, 0);

            let receive_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.receive());
                client
                    .call(&ink_e2e::alice(), _msg, 1000000000000, None)
                    .await
                    .expect("call failed")
                    .return_value()
            };

            let release_all = method_call!(client, address, release_all);

            assert!(release_all.is_ok());

            let total_released = method_call!(client, address, total_released);

            let bob_released = {
                let _msg =
                    build_message::<ContractRef>(address.clone()).call(|contract| contract.released(address_of!(bob)));
                client
                    .call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
                    .return_value()
            };

            let alice_released = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.released(address_of!(alice)));
                client
                    .call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
                    .return_value()
            };

            assert!(alice_released > bob_released);

            assert_eq!(bob_released, (total_released * 40) / 100);

            assert_eq!(alice_released, (total_released * 60) / 100);

            assert_eq!(alice_released + bob_released, total_released);

            Ok(())
        }
    }
}
