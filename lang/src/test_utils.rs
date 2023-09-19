// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

use ink::env::hash::{
    Blake2x256,
    CryptoHash,
    HashOutput,
};

#[cfg(feature = "std")]
use ink::env::{
    test::DefaultAccounts,
    DefaultEnvironment,
    Environment,
};
use ink::primitives::{
    Clear,
    Hash,
};

pub fn encoded_into_hash<T>(entity: &T) -> Hash
where
    T: scale::Encode,
{
    let mut result = Hash::CLEAR_HASH;
    let len_result = result.as_ref().len();
    let encoded = entity.encode();
    let len_encoded = encoded.len();
    if len_encoded <= len_result {
        result.as_mut()[..len_encoded].copy_from_slice(&encoded);
        return result
    }
    let mut hash_output = <<Blake2x256 as HashOutput>::Type as Default>::default();
    <Blake2x256 as CryptoHash>::hash(&encoded, &mut hash_output);
    let copy_len = core::cmp::min(hash_output.len(), len_result);
    result.as_mut()[0..copy_len].copy_from_slice(&hash_output[0..copy_len]);
    result
}

/// For calculating the event topic hash.
pub struct PrefixedValue<'a, 'b, T> {
    pub prefix: &'a [u8],
    pub value: &'b T,
}

impl<X> scale::Encode for PrefixedValue<'_, '_, X>
where
    X: scale::Encode,
{
    #[inline]
    fn size_hint(&self) -> usize {
        self.prefix.size_hint() + self.value.size_hint()
    }

    #[inline]
    fn encode_to<T: scale::Output + ?Sized>(&self, dest: &mut T) {
        self.prefix.encode_to(dest);
        self.value.encode_to(dest);
    }
}

#[cfg(feature = "std")]
pub fn accounts() -> DefaultAccounts<DefaultEnvironment> {
    ink::env::test::default_accounts::<DefaultEnvironment>()
}

#[cfg(feature = "std")]
pub fn change_caller(new_caller: <DefaultEnvironment as Environment>::AccountId) {
    ink::env::test::set_caller::<ink::env::DefaultEnvironment>(new_caller);
}
