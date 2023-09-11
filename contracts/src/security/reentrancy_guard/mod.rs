// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::{
    reentrancy_guard,
    traits::errors::ReentrancyGuardError,
};
use ink::storage::traits::Storable;
use openbrush::{
    modifier_definition,
    traits::Storage,
};

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    #[lazy]
    pub status: u8,
}

const NOT_ENTERED: u8 = 0;
const ENTERED: u8 = 1;

/// Prevents a contract from calling itself, directly or indirectly.
/// Calling a `non_reentrant` function from another `non_reentrant`
/// function is not supported. It is possible to prevent this from happening
/// by making the `non_reentrant` function external, and make it call a
/// `private` function that does the actual work.
///
/// This modifier flushes the struct into storage with `ENTERED`
/// status before calling the original method.
#[modifier_definition]
pub fn non_reentrant<T, F, R, E>(instance: &mut T, body: F) -> Result<R, E>
where
    T: Storage<Data> + Storable,
    F: FnOnce(&mut T) -> Result<R, E>,
    E: From<ReentrancyGuardError>,
{
    if instance.data().status.get_or_default() == ENTERED {
        return Err(From::from(ReentrancyGuardError::ReentrantCall))
    }
    // Any calls to nonReentrant after this point will fail
    instance.data().status.set(&ENTERED);

    let result = body(instance);
    instance.data().status.set(&NOT_ENTERED);

    result
}
