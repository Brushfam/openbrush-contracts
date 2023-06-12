#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
pub mod my_psp22_pausable {
    use openbrush::{
        contracts::{
            pausable::*,
            psp22::*,
        },
        modifiers,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        pause: pausable::Data,
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

        // override this function
        #[modifiers(when_not_paused)]
        fn _before_token_transfer(
            &mut self,
            _: Option<&AccountId>,
            _: Option<&AccountId>,
            _: &Balance,
        ) -> Result<(), PSP22Error> {
            Ok(())
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

    impl PausableImpl for Contract {}

    impl Pausable for Contract {
        #[ink(message)]
        fn paused(&self) -> bool {
            PausableImpl::paused(self)
        }
    }

    impl pausable::InternalImpl for Contract {}

    impl pausable::Internal for Contract {
        fn _emit_paused_event(&self, account: AccountId) {
            pausable::InternalImpl::_emit_paused_event(self, account)
        }

        fn _emit_unpaused_event(&self, account: AccountId) {
            pausable::InternalImpl::_emit_unpaused_event(self, account)
        }

        fn _paused(&self) -> bool {
            pausable::InternalImpl::_paused(self)
        }

        fn _pause<E: From<PausableError>>(&mut self) -> Result<(), E> {
            pausable::InternalImpl::_pause(self)
        }

        fn _unpause<E: From<PausableError>>(&mut self) -> Result<(), E> {
            pausable::InternalImpl::_unpause(self)
        }

        fn _switch_pause<E: From<PausableError>>(&mut self) -> Result<(), E> {
            pausable::InternalImpl::_switch_pause(self)
        }
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self::default();

            psp22::Internal::_mint_to(&mut instance, Self::env().caller(), total_supply).expect("Should mint");

            instance
        }

        /// Function which changes state to unpaused if paused and vice versa
        #[ink(message)]
        pub fn change_state(&mut self) -> Result<(), PSP22Error> {
            pausable::Internal::_switch_pause(self)
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use openbrush::contracts::{
            pausable::pausable_external::Pausable,
            psp22::psp22_external::PSP22,
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
        async fn can_transfer_when_not_paused(client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(1000);
            let address = client
                .instantiate("my_psp22_pausable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert!(matches!(balance_of!(client, address, alice), 1000));
            assert!(matches!(balance_of!(client, address, bob), 0));

            let transfer_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of!(bob), 100, vec![]));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("transfer failed")
            }
            .return_value();

            assert!(matches!(transfer_tx, Ok(())));

            assert!(matches!(balance_of!(client, address, alice), 900));
            assert!(matches!(balance_of!(client, address, bob), 100));

            Ok(())
        }

        #[ink_e2e::test]
        async fn cannot_transfer_when_paused(client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(1000);
            let address = client
                .instantiate("my_psp22_pausable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert!(matches!(balance_of!(client, address, alice), 1000));
            assert!(matches!(balance_of!(client, address, bob), 0));

            let pause_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.change_state());
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("change_state failed")
            }
            .return_value();

            assert!(matches!(pause_tx, Ok(())));

            let transfer_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of!(bob), 100, vec![]));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(transfer_tx, Err(_)));

            assert!(matches!(balance_of!(client, address, alice), 1000));
            assert!(matches!(balance_of!(client, address, bob), 0));

            Ok(())
        }
    }
}
