// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

pub use crate::{
    access_control,
    timelock_controller,
    traits::{
        access_control::*,
        governance::extensions::timelock_controller::*,
    },
};
pub use access_control::{
    AccessControlImpl,
    Internal as _,
    InternalImpl as _,
};
use core::convert::TryFrom;
use ink::{
    env::{
        call::{
            build_call,
            Call,
            ExecutionInput,
        },
        hash::Blake2x256,
        CallFlags,
        DefaultEnvironment,
    },
    prelude::{
        vec,
        vec::Vec,
    },
};
use openbrush::{
    modifier_definition,
    modifiers,
    storage::Mapping,
    traits::{
        AccountId,
        Hash,
        Storage,
        Timestamp,
    },
};
pub use timelock_controller::Internal as _;

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    #[lazy]
    pub min_delay: Timestamp,
    pub timestamps: Mapping<OperationId, Timestamp>,
}

/// Modifier to make a function callable only by a certain role. In
/// addition to checking the sender's role, zero account's role is also
/// considered. Granting a role to zero account is equivalent to enabling
/// this role for everyone.
#[modifier_definition]
pub fn only_role_or_open_role<T, F, R, E>(instance: &mut T, body: F, role: RoleType) -> Result<R, E>
where
    T: access_control::Internal + access_control::MembersManager + Storage<access_control::Data>,
    F: FnOnce(&mut T) -> Result<R, E>,
    E: From<AccessControlError>,
{
    if !instance._has_role(role, &None) {
        instance._check_role(role, Some(T::env().caller()))?;
    }
    body(instance)
}

pub const TIMELOCK_ADMIN_ROLE: RoleType = ink::selector_id!("TIMELOCK_ADMIN_ROLE");
pub const PROPOSER_ROLE: RoleType = ink::selector_id!("PROPOSER_ROLE");
pub const EXECUTOR_ROLE: RoleType = ink::selector_id!("EXECUTOR_ROLE");

pub const DONE_TIMESTAMP: Timestamp = 1;

pub trait TimelockControllerImpl:
    Internal + Storage<Data> + access_control::MembersManager + access_control::Internal + Storage<access_control::Data>
{
    fn is_operation(&self, id: OperationId) -> bool {
        self._is_operation(id)
    }

    fn is_operation_pending(&self, id: OperationId) -> bool {
        self._get_timestamp(id) > Self::_done_timestamp()
    }

    fn is_operation_ready(&self, id: OperationId) -> bool {
        self._is_operation_ready(id)
    }

    fn is_operation_done(&self, id: OperationId) -> bool {
        self._is_operation_done(id)
    }

    fn get_timestamp(&self, id: OperationId) -> Timestamp {
        self._get_timestamp(id)
    }

    fn get_min_delay(&self) -> Timestamp {
        self.data::<Data>().min_delay.get_or_default()
    }

    fn hash_operation(&self, transaction: Transaction, predecessor: Option<OperationId>, salt: [u8; 32]) -> Hash {
        self._hash_operation(&transaction, &predecessor, &salt)
    }

    fn hash_operation_batch(
        &self,
        transactions: Vec<Transaction>,
        predecessor: Option<OperationId>,
        salt: [u8; 32],
    ) -> Hash {
        self._hash_operation_batch(&transactions, &predecessor, &salt)
    }

    #[modifiers(access_control::only_role(<Self as Internal>::_proposal_role()))]
    fn schedule(
        &mut self,
        transaction: Transaction,
        predecessor: Option<OperationId>,
        salt: [u8; 32],
        delay: Timestamp,
    ) -> Result<(), TimelockControllerError> {
        let id = self._hash_operation(&transaction, &predecessor, &salt);

        self._schedule(id, &delay)?;

        self._emit_call_scheduled_event(id, 0, transaction, predecessor, delay);
        Ok(())
    }

    #[modifiers(access_control::only_role(<Self as Internal>::_proposal_role()))]
    fn schedule_batch(
        &mut self,
        transactions: Vec<Transaction>,
        predecessor: Option<OperationId>,
        salt: [u8; 32],
        delay: Timestamp,
    ) -> Result<(), TimelockControllerError> {
        let id = self._hash_operation_batch(&transactions, &predecessor, &salt);

        self._schedule(id, &delay)?;

        for (i, transaction) in transactions.into_iter().enumerate() {
            self._emit_call_scheduled_event(id, i as u8, transaction, predecessor, delay);
        }
        Ok(())
    }

    #[modifiers(access_control::only_role(<Self as Internal>::_proposal_role()))]
    fn cancel(&mut self, id: OperationId) -> Result<(), TimelockControllerError> {
        if !self.is_operation_pending(id) {
            return Err(TimelockControllerError::OperationCannonBeCanceled)
        }
        self.data::<Data>().timestamps.remove(&id);

        self._emit_cancelled_event(id);
        Ok(())
    }

    #[modifiers(only_role_or_open_role(<Self as Internal>::_executor_role()))]
    fn execute(
        &mut self,
        transaction: Transaction,
        predecessor: Option<OperationId>,
        salt: [u8; 32],
    ) -> Result<(), TimelockControllerError> {
        let id = self._hash_operation(&transaction, &predecessor, &salt);

        self._before_call(predecessor)?;
        self._call(id, 0, transaction)?;
        self._after_call(id)
    }

    #[modifiers(only_role_or_open_role(<Self as Internal>::_executor_role()))]
    fn execute_batch(
        &mut self,
        transactions: Vec<Transaction>,
        predecessor: Option<OperationId>,
        salt: [u8; 32],
    ) -> Result<(), TimelockControllerError> {
        let id = self._hash_operation_batch(&transactions, &predecessor, &salt);

        self._before_call(predecessor)?;

        for (i, transaction) in transactions.into_iter().enumerate() {
            self._call(id, i as u8, transaction)?;
        }
        self._after_call(id)
    }

    fn update_delay(&mut self, new_delay: Timestamp) -> Result<(), TimelockControllerError> {
        if Self::env().account_id() != Self::env().caller() {
            return Err(TimelockControllerError::CallerMustBeTimeLock)
        }

        let old_delay = self.data::<Data>().min_delay.get_or_default();
        self._emit_min_delay_change_event(old_delay, new_delay);

        self.data::<Data>().min_delay.set(&new_delay);
        Ok(())
    }
}

pub trait Internal {
    /// User must override those methods in their contract.
    fn _emit_min_delay_change_event(&self, old_delay: Timestamp, new_delay: Timestamp);

    fn _emit_call_scheduled_event(
        &self,
        id: OperationId,
        index: u8,
        transaction: Transaction,
        predecessor: Option<OperationId>,
        delay: Timestamp,
    );

    fn _emit_cancelled_event(&self, id: OperationId);

    fn _emit_call_executed_event(&self, id: OperationId, index: u8, transaction: Transaction);

    fn _init_with_caller(&mut self, min_delay: Timestamp, proposers: Vec<AccountId>, executors: Vec<AccountId>);

    fn _init_with_admin(
        &mut self,
        admin: Option<AccountId>,
        min_delay: Timestamp,
        proposers: Vec<AccountId>,
        executors: Vec<AccountId>,
    );

    fn _hash_operation(
        &self,
        transaction: &Transaction,
        predecessor: &Option<OperationId>,
        salt: &[u8; 32],
    ) -> OperationId;

    fn _hash_operation_batch(
        &self,
        transactions: &[Transaction],
        predecessor: &Option<OperationId>,
        salt: &[u8; 32],
    ) -> OperationId;

    /// Schedule an operation that is to become valid after a given delay.
    fn _schedule(&mut self, id: OperationId, delay: &Timestamp) -> Result<(), TimelockControllerError>;

    /// Checks before execution of an operation's calls.
    fn _before_call(&self, predecessor: Option<OperationId>) -> Result<(), TimelockControllerError>;

    /// Checks after execution of an operation's calls.
    fn _after_call(&mut self, id: OperationId) -> Result<(), TimelockControllerError>;

    /// Execute an operation's call.
    ///
    /// Emits a `CallExecuted` event.
    fn _call(&mut self, id: OperationId, index: u8, transaction: Transaction) -> Result<(), TimelockControllerError>;

    fn _timelock_admin_role() -> RoleType;

    fn _proposal_role() -> RoleType;

    fn _executor_role() -> RoleType;

    fn _done_timestamp() -> Timestamp;

    fn _is_operation(&self, id: OperationId) -> bool;

    fn _is_operation_ready(&self, id: OperationId) -> bool;

    fn _is_operation_done(&self, id: OperationId) -> bool;

    fn _get_timestamp(&self, id: OperationId) -> Timestamp;
}

pub trait InternalImpl: Internal + Storage<Data> + access_control::Internal {
    fn _emit_min_delay_change_event(&self, _old_delay: Timestamp, _new_delay: Timestamp) {}

    fn _emit_call_scheduled_event(
        &self,
        _id: OperationId,
        _index: u8,
        _transaction: Transaction,
        _predecessor: Option<OperationId>,
        _delay: Timestamp,
    ) {
    }

    fn _emit_cancelled_event(&self, _id: OperationId) {}

    fn _emit_call_executed_event(&self, _id: OperationId, _index: u8, _transaction: Transaction) {}

    fn _init_with_caller(&mut self, min_delay: Timestamp, proposers: Vec<AccountId>, executors: Vec<AccountId>) {
        Internal::_init_with_admin(self, Some(Self::env().caller()), min_delay, proposers, executors);
    }

    fn _init_with_admin(
        &mut self,
        admin: Option<AccountId>,
        min_delay: Timestamp,
        proposers: Vec<AccountId>,
        executors: Vec<AccountId>,
    ) {
        self._set_role_admin(
            <Self as Internal>::_timelock_admin_role(),
            <Self as Internal>::_timelock_admin_role(),
        );
        self._set_role_admin(
            <Self as Internal>::_proposal_role(),
            <Self as Internal>::_proposal_role(),
        );
        self._set_role_admin(
            <Self as Internal>::_executor_role(),
            <Self as Internal>::_executor_role(),
        );

        // admin + self administration
        self._setup_role(
            <Self as Internal>::_timelock_admin_role(),
            Some(Self::env().account_id()),
        );
        self._setup_role(<Self as Internal>::_timelock_admin_role(), admin);

        // register proposers
        proposers
            .into_iter()
            .for_each(|proposer| self._setup_role(<Self as Internal>::_proposal_role(), Some(proposer)));
        // register executors
        executors
            .into_iter()
            .for_each(|executor| self._setup_role(<Self as Internal>::_executor_role(), Some(executor)));

        let old_delay = self.data::<Data>().min_delay.get_or_default();
        self.data::<Data>().min_delay.set(&min_delay);
        Internal::_emit_min_delay_change_event(self, old_delay, min_delay);
    }

    fn _hash_operation(
        &self,
        transaction: &Transaction,
        predecessor: &Option<OperationId>,
        salt: &[u8; 32],
    ) -> OperationId {
        let mut hash_data: Vec<u8> = vec![];

        hash_data.append(&mut scale::Encode::encode(&transaction));
        if predecessor.is_some() {
            hash_data.append(&mut scale::Encode::encode(&predecessor.unwrap()));
        }
        hash_data.append(&mut scale::Encode::encode(&salt));

        Hash::try_from(Self::env().hash_bytes::<Blake2x256>(&hash_data).as_ref()).unwrap()
    }

    fn _hash_operation_batch(
        &self,
        transactions: &[Transaction],
        predecessor: &Option<OperationId>,
        salt: &[u8; 32],
    ) -> OperationId {
        let mut hash_data: Vec<u8> = vec![];

        hash_data.append(&mut scale::Encode::encode(&transactions));
        if predecessor.is_some() {
            hash_data.append(&mut scale::Encode::encode(&predecessor.unwrap()));
        }
        hash_data.append(&mut scale::Encode::encode(&salt));

        Hash::try_from(Self::env().hash_bytes::<Blake2x256>(&hash_data).as_ref()).unwrap()
    }

    fn _schedule(&mut self, id: OperationId, delay: &Timestamp) -> Result<(), TimelockControllerError> {
        if Internal::_is_operation(self, id) {
            return Err(TimelockControllerError::OperationAlreadyScheduled)
        }
        if delay < &self.data::<Data>().min_delay.get_or_default() {
            return Err(TimelockControllerError::InsufficientDelay)
        }

        self.data::<Data>()
            .timestamps
            .insert(&id, &(Self::env().block_timestamp() + delay));
        Ok(())
    }

    fn _before_call(&self, predecessor: Option<OperationId>) -> Result<(), TimelockControllerError> {
        if predecessor.is_some() && !Internal::_is_operation_done(self, predecessor.unwrap()) {
            return Err(TimelockControllerError::MissingDependency)
        }
        Ok(())
    }

    fn _after_call(&mut self, id: OperationId) -> Result<(), TimelockControllerError> {
        if !Internal::_is_operation_ready(self, id) {
            return Err(TimelockControllerError::OperationIsNotReady)
        }

        self.data::<Data>()
            .timestamps
            .insert(&id, &<Self as Internal>::_done_timestamp());
        Ok(())
    }

    fn _call(&mut self, id: OperationId, i: u8, transaction: Transaction) -> Result<(), TimelockControllerError> {
        let result = if let Some(callee) = transaction.callee {
            build_call::<DefaultEnvironment>()
                .call_type(
                    Call::new(callee)
                        .gas_limit(transaction.gas_limit)
                        .transferred_value(transaction.transferred_value),
                )
                .exec_input(ExecutionInput::new(transaction.selector.into()).push_arg(CallInput(&transaction.input)))
                .returns::<()>()
                .call_flags(CallFlags::default().set_allow_reentry(true))
                .try_invoke()
                .map_err(|_| TimelockControllerError::UnderlyingTransactionReverted)
        } else {
            Err(TimelockControllerError::CalleeZeroAddress)
        };

        result?.unwrap();
        Internal::_emit_call_executed_event(self, id, i, transaction);
        Ok(())
    }

    fn _timelock_admin_role() -> RoleType {
        TIMELOCK_ADMIN_ROLE
    }

    fn _proposal_role() -> RoleType {
        PROPOSER_ROLE
    }

    fn _executor_role() -> RoleType {
        EXECUTOR_ROLE
    }

    fn _done_timestamp() -> Timestamp {
        DONE_TIMESTAMP
    }

    fn _is_operation(&self, id: OperationId) -> bool {
        Internal::_get_timestamp(self, id) > Timestamp::default()
    }

    fn _is_operation_ready(&self, id: OperationId) -> bool {
        let timestamp = Internal::_get_timestamp(self, id);
        timestamp > <Self as Internal>::_done_timestamp() && timestamp <= Self::env().block_timestamp()
    }

    fn _is_operation_done(&self, id: OperationId) -> bool {
        Internal::_get_timestamp(self, id) == <Self as Internal>::_done_timestamp()
    }

    fn _get_timestamp(&self, id: OperationId) -> Timestamp {
        self.data::<Data>().timestamps.get(&id).unwrap_or(Timestamp::default())
    }
}

/// A wrapper that allows us to encode a blob of bytes.
///
/// We use this to pass the set of untyped (bytes) parameters to the `CallBuilder`.
pub struct CallInput<'a>(&'a [u8]);

impl<'a> scale::Encode for CallInput<'a> {
    fn encode_to<T: scale::Output + ?Sized>(&self, dest: &mut T) {
        dest.write(self.0);
    }
}
