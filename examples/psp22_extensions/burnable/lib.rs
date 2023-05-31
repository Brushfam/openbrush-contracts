#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
pub mod my_psp22_burnable {
    use openbrush::{
        contracts::psp22::extensions::burnable::*,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp22: psp22::Data,
    }

    impl psp22::InternalImpl for Contract {}

    impl psp22::Internal for Contract {
        fn _emit_transfer_event(&self, from: Option<AccountId>, to: Option<AccountId>, amount: Balance) {
            psp22::InternalImpl::_emit_transfer_event(self, from, to, amount)
        }

        fn _emit_approval_event(&self, owner: AccountId, spender: AccountId, amount: Balance) {
            psp22::InternalImpl::_emit_approval_event(self, owner, spender, amount)
        }

        fn _total_supply(&self) -> Balance {
            psp22::InternalImpl::_total_supply(self)
        }

        fn _balance_of(&self, owner: &AccountId) -> Balance {
            psp22::InternalImpl::_balance_of(self, owner)
        }

        fn _allowance(&self, owner: &AccountId, spender: &AccountId) -> Balance {
            psp22::InternalImpl::_allowance(self, owner, spender)
        }

        fn _transfer_from_to(
            &mut self,
            from: AccountId,
            to: AccountId,
            amount: Balance,
            data: Vec<u8>,
        ) -> Result<(), PSP22Error> {
            psp22::InternalImpl::_transfer_from_to(self, from, to, amount, data)
        }

        fn _approve_from_to(
            &mut self,
            owner: AccountId,
            spender: AccountId,
            amount: Balance,
        ) -> Result<(), PSP22Error> {
            psp22::InternalImpl::_approve_from_to(self, owner, spender, amount)
        }

        fn _mint_to(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            psp22::InternalImpl::_mint_to(self, account, amount)
        }

        fn _burn_from(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            psp22::InternalImpl::_burn_from(self, account, amount)
        }

        fn _before_token_transfer(
            &mut self,
            from: Option<&AccountId>,
            to: Option<&AccountId>,
            amount: &Balance,
        ) -> Result<(), PSP22Error> {
            psp22::InternalImpl::_before_token_transfer(self, from, to, amount)
        }

        fn _after_token_transfer(
            &mut self,
            from: Option<&AccountId>,
            to: Option<&AccountId>,
            amount: &Balance,
        ) -> Result<(), PSP22Error> {
            psp22::InternalImpl::_after_token_transfer(self, from, to, amount)
        }
    }

    impl PSP22Impl for Contract {}

    impl PSP22 for Contract {
        #[ink(message)]
        fn total_supply(&self) -> Balance {
            PSP22Impl::total_supply(self)
        }

        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> Balance {
            PSP22Impl::balance_of(self, owner)
        }

        #[ink(message)]
        fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            PSP22Impl::allowance(self, owner, spender)
        }

        #[ink(message)]
        fn transfer(&mut self, to: AccountId, value: Balance, data: Vec<u8>) -> Result<(), PSP22Error> {
            PSP22Impl::transfer(self, to, value, data)
        }

        #[ink(message)]
        fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
            data: Vec<u8>,
        ) -> Result<(), PSP22Error> {
            PSP22Impl::transfer_from(self, from, to, value, data)
        }

        #[ink(message)]
        fn approve(&mut self, spender: AccountId, value: Balance) -> Result<(), PSP22Error> {
            PSP22Impl::approve(self, spender, value)
        }

        #[ink(message)]
        fn increase_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), PSP22Error> {
            PSP22Impl::increase_allowance(self, spender, delta_value)
        }

        #[ink(message)]
        fn decrease_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), PSP22Error> {
            PSP22Impl::decrease_allowance(self, spender, delta_value)
        }
    }

    impl PSP22BurnableImpl for Contract {}

    impl PSP22Burnable for Contract {
        #[ink(message)]
        fn burn(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            PSP22BurnableImpl::burn(self, account, amount)
        }
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self::default();

            psp22::Internal::_mint_to(&mut instance, Self::env().caller(), total_supply).expect("Should mint");

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
        use openbrush::contracts::psp22::{
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

        #[ink_e2e::test]
        async fn assigns_initial_balance(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(100);
            let address = client
                .instantiate("my_psp22_burnable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let balance_of_alice = balance_of!(client, address, alice);

            assert!(matches!(balance_of_alice, 100));

            Ok(())
        }

        #[ink_e2e::test]
        async fn can_burn(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(100);
            let address = client
                .instantiate("my_psp22_burnable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.burn(address_of!(alice), 10));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let balance_of_alice = balance_of!(client, address, alice);

            assert!(matches!(balance_of_alice, 90));

            Ok(())
        }

        #[ink_e2e::test]
        async fn can_burn_without_allowance(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(100);
            let address = client
                .instantiate("my_psp22_burnable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert!(matches!(balance_of!(client, address, bob), 0));
            assert!(matches!(balance_of!(client, address, alice), 100));

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.burn(address_of!(alice), 10));
                client.call(&ink_e2e::bob(), _msg, 0, None).await.expect("call failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let balance_of_alice = balance_of!(client, address, alice);

            assert!(matches!(balance_of_alice, 90));

            Ok(())
        }

        #[ink_e2e::test]
        async fn decreases_total_supply_after_burning(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(100);
            let address = client
                .instantiate("my_psp22_burnable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let total_supply = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.total_supply());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            };

            assert!(matches!(total_supply.return_value(), 100));

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.burn(address_of!(alice), 10));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let total_supply = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.total_supply());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            };

            assert!(matches!(total_supply.return_value(), 90));

            Ok(())
        }

        #[ink_e2e::test]
        async fn can_burn_from(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(100);
            let address = client
                .instantiate("my_psp22_burnable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of!(bob), 10, vec![]));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let balance_of_bob = balance_of!(client, address, bob);

            assert!(matches!(balance_of_bob, 10));

            let result = {
                let _msg =
                    build_message::<ContractRef>(address.clone()).call(|contract| contract.burn(address_of!(bob), 10));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let balance_of_bob = balance_of!(client, address, bob);

            assert!(matches!(balance_of_bob, 0));

            Ok(())
        }

        #[ink_e2e::test]
        async fn can_burn_from_many(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(100);
            let address = client
                .instantiate("my_psp22_burnable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of!(bob), 10, vec![]));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of!(charlie), 10, vec![]));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let balance_of_bob = balance_of!(client, address, bob);
            let balance_of_charlie = balance_of!(client, address, charlie);

            assert!(matches!(balance_of_bob, 10));
            assert!(matches!(balance_of_charlie, 10));

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.burn_from_many(vec![(address_of!(bob), 10), (address_of!(charlie), 10)]));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let balance_of_bob = balance_of!(client, address, bob);
            let balance_of_charlie = balance_of!(client, address, charlie);

            assert!(matches!(balance_of_bob, 0));
            assert!(matches!(balance_of_charlie, 0));

            Ok(())
        }

        #[ink_e2e::test]
        async fn fails_if_one_of_the_accounts_balance_exceeds_amount_to_burn(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let constructor = ContractRef::new(100);
            let address = client
                .instantiate("my_psp22_burnable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of!(bob), 10, vec![]));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of!(charlie), 5, vec![]));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let balance_of_bob = balance_of!(client, address, bob);
            let balance_of_charlie = balance_of!(client, address, charlie);

            assert!(matches!(balance_of_bob, 10));
            assert!(matches!(balance_of_charlie, 5));

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.burn_from_many(vec![(address_of!(bob), 10), (address_of!(charlie), 10)]));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            };

            assert!(matches!(result.return_value(), Err(PSP22Error::InsufficientBalance)));

            let balance_of_bob = balance_of!(client, address, bob);
            let balance_of_charlie = balance_of!(client, address, charlie);

            assert!(matches!(balance_of_bob, 10));
            assert!(matches!(balance_of_charlie, 5));

            Ok(())
        }
    }
}
