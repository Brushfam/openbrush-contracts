#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::contract]
pub mod my_timelock_controller {
    use ink::prelude::vec::Vec;
    use openbrush::{
        contracts::timelock_controller::*,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        access_control: access_control::Data,
        #[storage_field]
        timelock: timelock_controller::Data,
    }

    // `TimelockController` is an extension for `AccessControl`, so you have to inherit logic related to both modules.
    impl AccessControlImpl for Contract {}

    impl AccessControl for Contract {
        #[ink(message)]
        fn has_role(&self, role: RoleType, address: AccountId) -> bool {
            AccessControlImpl::has_role(self, role, address)
        }

        #[ink(message)]
        fn get_role_admin(&self, role: RoleType) -> RoleType {
            AccessControlImpl::get_role_admin(self, role)
        }

        #[ink(message)]
        fn grant_role(&mut self, role: RoleType, account: AccountId) -> Result<(), AccessControlError> {
            AccessControlImpl::grant_role(self, role, account)
        }

        #[ink(message)]
        fn revoke_role(&mut self, role: RoleType, account: AccountId) -> Result<(), AccessControlError> {
            AccessControlImpl::revoke_role(self, role, account)
        }

        #[ink(message)]
        fn renounce_role(&mut self, role: RoleType, account: AccountId) -> Result<(), AccessControlError> {
            AccessControlImpl::renounce_role(self, role, account)
        }
    }

    impl access_control::InternalImpl for Contract {}

    impl access_control::Internal for Contract {
        fn _emit_role_admin_changed(&mut self, _role: RoleType, _previous: RoleType, _new: RoleType) {
            access_control::InternalImpl::_emit_role_admin_changed(self, _role, _previous, _new);
        }

        fn _emit_role_granted(&mut self, _role: RoleType, _grantee: AccountId, _grantor: Option<AccountId>) {
            access_control::InternalImpl::_emit_role_granted(self, _role, _grantee, _grantor);
        }

        fn _emit_role_revoked(&mut self, _role: RoleType, _account: AccountId, _sender: AccountId) {
            access_control::InternalImpl::_emit_role_revoked(self, _role, _account, _sender);
        }

        fn _default_admin() -> RoleType {
            <Self as access_control::InternalImpl>::_default_admin()
        }

        fn _init_with_caller(&mut self) {
            access_control::InternalImpl::_init_with_caller(self);
        }

        fn _init_with_admin(&mut self, admin: AccountId) {
            access_control::InternalImpl::_init_with_admin(self, admin);
        }

        fn _setup_role(&mut self, role: RoleType, member: AccountId) {
            access_control::InternalImpl::_setup_role(self, role, member);
        }

        fn _do_revoke_role(&mut self, role: RoleType, account: AccountId) {
            access_control::InternalImpl::_do_revoke_role(self, role, account);
        }

        fn _set_role_admin(&mut self, role: RoleType, new_admin: RoleType) {
            access_control::InternalImpl::_set_role_admin(self, role, new_admin);
        }

        fn _check_role(&self, role: RoleType, account: AccountId) -> Result<(), AccessControlError> {
            access_control::InternalImpl::_check_role(self, role, account)
        }

        fn _get_role_admin(&self, role: RoleType) -> RoleType {
            access_control::InternalImpl::_get_role_admin(self, role)
        }
    }

    impl access_control::MembersManagerImpl for Contract {}

    impl access_control::MembersManager for Contract {
        fn _has_role(&self, role: RoleType, address: &AccountId) -> bool {
            access_control::MembersManagerImpl::_has_role(self, role, address)
        }

        fn _add(&mut self, role: RoleType, member: &AccountId) {
            access_control::MembersManagerImpl::_add(self, role, member)
        }

        fn _remove(&mut self, role: RoleType, member: &AccountId) {
            access_control::MembersManagerImpl::_remove(self, role, member)
        }
    }

    impl timelock_controller::InternalImpl for Contract {}

    impl timelock_controller::Internal for Contract {
        fn _emit_min_delay_change_event(&self, old_delay: Timestamp, new_delay: Timestamp) {
            InternalImpl::_emit_min_delay_change_event(self, old_delay, new_delay)
        }

        fn _emit_call_scheduled_event(
            &self,
            id: OperationId,
            index: u8,
            transaction: Transaction,
            predecessor: Option<OperationId>,
            delay: Timestamp,
        ) {
            InternalImpl::_emit_call_scheduled_event(self, id, index, transaction, predecessor, delay)
        }

        fn _emit_cancelled_event(&self, id: OperationId) {
            InternalImpl::_emit_cancelled_event(self, id)
        }

        fn _emit_call_executed_event(&self, id: OperationId, index: u8, transaction: Transaction) {
            InternalImpl::_emit_call_executed_event(self, id, index, transaction)
        }

        fn _init_with_caller(&mut self, min_delay: Timestamp, proposers: Vec<AccountId>, executors: Vec<AccountId>) {
            InternalImpl::_init_with_caller(self, min_delay, proposers, executors)
        }

        fn _init_with_admin(
            &mut self,
            admin: AccountId,
            min_delay: Timestamp,
            proposers: Vec<AccountId>,
            executors: Vec<AccountId>,
        ) {
            InternalImpl::_init_with_admin(self, admin, min_delay, proposers, executors)
        }

        fn _hash_operation(
            &self,
            transaction: &Transaction,
            predecessor: &Option<OperationId>,
            salt: &[u8; 32],
        ) -> OperationId {
            InternalImpl::_hash_operation(self, transaction, predecessor, salt)
        }

        fn _hash_operation_batch(
            &self,
            transactions: &Vec<Transaction>,
            predecessor: &Option<OperationId>,
            salt: &[u8; 32],
        ) -> OperationId {
            InternalImpl::_hash_operation_batch(self, transactions, predecessor, salt)
        }

        fn _schedule(&mut self, id: OperationId, delay: &Timestamp) -> Result<(), TimelockControllerError> {
            InternalImpl::_schedule(self, id, delay)
        }

        fn _before_call(&self, predecessor: Option<OperationId>) -> Result<(), TimelockControllerError> {
            InternalImpl::_before_call(self, predecessor)
        }

        fn _after_call(&mut self, id: OperationId) -> Result<(), TimelockControllerError> {
            InternalImpl::_after_call(self, id)
        }

        fn _call(&mut self, id: OperationId, i: u8, transaction: Transaction) -> Result<(), TimelockControllerError> {
            InternalImpl::_call(self, id, i, transaction)
        }

        fn _timelock_admin_role() -> RoleType {
            <Self as InternalImpl>::_timelock_admin_role()
        }

        fn _proposal_role() -> RoleType {
            <Self as InternalImpl>::_proposal_role()
        }

        fn _executor_role() -> RoleType {
            <Self as InternalImpl>::_executor_role()
        }

        fn _done_timestamp() -> Timestamp {
            <Self as InternalImpl>::_done_timestamp()
        }

        fn _is_operation(&self, id: OperationId) -> bool {
            InternalImpl::_is_operation(self, id)
        }

        fn _is_operation_ready(&self, id: OperationId) -> bool {
            InternalImpl::_is_operation_ready(self, id)
        }

        fn _is_operation_done(&self, id: OperationId) -> bool {
            InternalImpl::_is_operation_done(self, id)
        }

        fn _get_timestamp(&self, id: OperationId) -> Timestamp {
            InternalImpl::_get_timestamp(self, id)
        }
    }

    impl TimelockControllerImpl for Contract {}

    impl TimelockController for Contract {
        #[ink(message)]
        fn is_operation(&self, id: OperationId) -> bool {
            TimelockControllerImpl::is_operation(self, id)
        }

        #[ink(message)]
        fn is_operation_pending(&self, id: OperationId) -> bool {
            TimelockControllerImpl::is_operation_pending(self, id)
        }

        #[ink(message)]
        fn is_operation_ready(&self, id: OperationId) -> bool {
            TimelockControllerImpl::is_operation_ready(self, id)
        }

        #[ink(message)]
        fn is_operation_done(&self, id: OperationId) -> bool {
            TimelockControllerImpl::is_operation_done(self, id)
        }

        #[ink(message)]
        fn get_timestamp(&self, id: OperationId) -> Timestamp {
            TimelockControllerImpl::get_timestamp(self, id)
        }

        #[ink(message)]
        fn get_min_delay(&self) -> Timestamp {
            TimelockControllerImpl::get_min_delay(self)
        }

        #[ink(message)]
        fn hash_operation(&self, transaction: Transaction, predecessor: Option<OperationId>, salt: [u8; 32]) -> Hash {
            TimelockControllerImpl::hash_operation(self, transaction, predecessor, salt)
        }

        #[ink(message)]
        fn hash_operation_batch(
            &self,
            transactions: Vec<Transaction>,
            predecessor: Option<OperationId>,
            salt: [u8; 32],
        ) -> Hash {
            TimelockControllerImpl::hash_operation_batch(self, transactions, predecessor, salt)
        }

        #[ink(message)]
        fn schedule(
            &mut self,
            transaction: Transaction,
            predecessor: Option<OperationId>,
            salt: [u8; 32],
            delay: Timestamp,
        ) -> Result<(), TimelockControllerError> {
            TimelockControllerImpl::schedule(self, transaction, predecessor, salt, delay)
        }

        #[ink(message)]
        fn schedule_batch(
            &mut self,
            transactions: Vec<Transaction>,
            predecessor: Option<OperationId>,
            salt: [u8; 32],
            delay: Timestamp,
        ) -> Result<(), TimelockControllerError> {
            TimelockControllerImpl::schedule_batch(self, transactions, predecessor, salt, delay)
        }

        #[ink(message)]
        fn cancel(&mut self, id: OperationId) -> Result<(), TimelockControllerError> {
            TimelockControllerImpl::cancel(self, id)
        }

        #[ink(message)]
        fn execute(
            &mut self,
            transaction: Transaction,
            predecessor: Option<OperationId>,
            salt: [u8; 32],
        ) -> Result<(), TimelockControllerError> {
            TimelockControllerImpl::execute(self, transaction, predecessor, salt)
        }

        #[ink(message)]
        fn execute_batch(
            &mut self,
            transactions: Vec<Transaction>,
            predecessor: Option<OperationId>,
            salt: [u8; 32],
        ) -> Result<(), TimelockControllerError> {
            TimelockControllerImpl::execute_batch(self, transactions, predecessor, salt)
        }

        #[ink(message)]
        fn update_delay(&mut self, new_delay: Timestamp) -> Result<(), TimelockControllerError> {
            TimelockControllerImpl::update_delay(self, new_delay)
        }
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(min_delay: Timestamp, proposers: Vec<AccountId>, executors: Vec<AccountId>) -> Self {
            let mut instance = Self::default();

            let caller = Self::env().caller();
            // `TimelockController` and `AccessControl` have `_init_with_admin` methods.
            // You need to call it for each trait separately, to initialize everything for these traits.
            access_control::Internal::_init_with_admin(&mut instance, caller);
            timelock_controller::Internal::_init_with_admin(&mut instance, caller, min_delay, proposers, executors);

            instance
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use openbrush::contracts::timelock_controller::timelockcontroller_external::TimelockController;

        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::{build_message, PolkadotConfig};
        use ink_e2e::Client;

        use test_helpers::address_of;

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn can_schedule(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(0, vec![address_of!(bob)], vec![address_of!(bob)]);
            let address = client
                .instantiate("my_timelock_controller", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let transaction = Transaction {
                callee: address.clone(),
                selector: [0, 0, 0, 0],
                input: vec![],
                transferred_value: 0,
                gas_limit: 0,
            };

            let salt = [0; 32];

            let id = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.hash_operation(transaction.clone(), None, salt));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            let is_operation_pending = {
                let _msg =
                    build_message::<ContractRef>(address.clone()).call(|contract| contract.is_operation_pending(id));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(is_operation_pending, false);

            let schedule_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.schedule(transaction.clone(), None, salt, 0));
                client
                    .call(&ink_e2e::bob(), _msg, 0, None)
                    .await
                    .expect("schedule failed")
            }
            .return_value();

            assert_eq!(schedule_tx, Ok(()));

            let is_operation_pending = {
                let _msg =
                    build_message::<ContractRef>(address.clone()).call(|contract| contract.is_operation_pending(id));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(is_operation_pending, true);

            let is_operation_ready = {
                let _msg =
                    build_message::<ContractRef>(address.clone()).call(|contract| contract.is_operation_ready(id));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(is_operation_ready, true);

            let is_operation_done = {
                let _msg =
                    build_message::<ContractRef>(address.clone()).call(|contract| contract.is_operation_done(id));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(is_operation_done, false);

            Ok(())
        }

        #[ink_e2e::test]
        async fn schedule_and_execute_without_input_data(client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(0, vec![address_of!(bob)], vec![address_of!(bob)]);
            let address = client
                .instantiate("my_timelock_controller", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let transaction = Transaction {
                callee: address.clone(),
                selector: ink::selector_bytes!("TimelockController::get_min_delay"),
                input: vec![],
                transferred_value: 0,
                gas_limit: 0,
            };

            let salt = [0; 32];

            let id = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.hash_operation(transaction.clone(), None, salt));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            let schedule_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.schedule(transaction.clone(), None, salt, 0));
                client
                    .call(&ink_e2e::bob(), _msg, 0, None)
                    .await
                    .expect("schedule failed")
            }
            .return_value();

            assert_eq!(schedule_tx, Ok(()));

            let is_operation_done = {
                let _msg =
                    build_message::<ContractRef>(address.clone()).call(|contract| contract.is_operation_done(id));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(is_operation_done, false);

            let execute_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.execute(transaction.clone(), None, salt));
                client
                    .call(&ink_e2e::bob(), _msg, 0, None)
                    .await
                    .expect("execute failed")
            }
            .return_value();

            assert_eq!(execute_tx, Ok(()));

            let is_operation_done = {
                let _msg =
                    build_message::<ContractRef>(address.clone()).call(|contract| contract.is_operation_done(id));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(is_operation_done, true);

            Ok(())
        }

        #[ink_e2e::test]
        async fn schedule_and_execute_by_passing_value_into_update_delay_and_update(
            client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let constructor = ContractRef::new(0, vec![address_of!(bob)], vec![address_of!(bob)]);
            let address = client
                .instantiate("my_timelock_controller", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let new_min_delay: u64 = 15;

            let transaction = Transaction {
                callee: address.clone(),
                selector: ink::selector_bytes!("TimelockController::update_delay"),
                input: new_min_delay.to_le_bytes().to_vec(),
                transferred_value: 0,
                gas_limit: 0,
            };

            let salt = [0; 32];

            let schedule_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.schedule(transaction.clone(), None, salt, 0));
                client
                    .call(&ink_e2e::bob(), _msg, 0, None)
                    .await
                    .expect("schedule failed")
            }
            .return_value();

            assert_eq!(schedule_tx, Ok(()));

            let get_min_delay = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.get_min_delay());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(get_min_delay, 0);

            let execute_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.execute(transaction.clone(), None, salt));
                client
                    .call(&ink_e2e::bob(), _msg, 0, None)
                    .await
                    .expect("execute failed")
            }
            .return_value();

            assert_eq!(execute_tx, Ok(()));

            let get_min_delay = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.get_min_delay());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(get_min_delay, new_min_delay);

            Ok(())
        }

        #[ink_e2e::test]
        async fn fails_schedule_because_signer_is_not_proposal(clientclient: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(0, vec![address_of!(bob)], vec![address_of!(bob)]);
            let address = client
                .instantiate("my_timelock_controller", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let transaction = Transaction {
                callee: address.clone(),
                selector: [0, 0, 0, 0],
                input: vec![],
                transferred_value: 0,
                gas_limit: 0,
            };

            let salt = [0; 32];

            let schedule_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.schedule(transaction.clone(), None, salt, 0));
                client.call_dry_run(&ink_e2e::charlie(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(schedule_tx, Err(_)));

            Ok(())
        }

        #[ink_e2e::test]
        async fn fails_execute_because_signer_is_not_executor(client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(0, vec![address_of!(bob)], vec![address_of!(bob)]);
            let address = client
                .instantiate("my_timelock_controller", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let transaction = Transaction {
                callee: address.clone(),
                selector: [0, 0, 0, 0],
                input: vec![],
                transferred_value: 0,
                gas_limit: 0,
            };

            let salt = [0; 32];

            let schedule_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.schedule(transaction.clone(), None, salt, 0));
                client
                    .call(&ink_e2e::bob(), _msg, 0, None)
                    .await
                    .expect("schedule failed")
            }
            .return_value();

            assert_eq!(schedule_tx, Ok(()));

            let execute_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.execute(transaction.clone(), None, salt));
                client.call_dry_run(&ink_e2e::charlie(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(execute_tx, Err(_)));

            Ok(())
        }

        #[ink_e2e::test]
        async fn fails_update_delay(client: Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(0, vec![address_of!(bob)], vec![address_of!(bob)]);
            let address = client
                .instantiate("my_timelock_controller", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let update_delay_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.update_delay(15));
                client.call_dry_run(&ink_e2e::bob(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(update_delay_tx, Err(_)));

            Ok(())
        }
    }
}
