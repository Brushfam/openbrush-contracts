// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

use core::marker::PhantomData;
mod mapping;
mod multi_mapping;
mod raw_mapping;

pub use mapping::Mapping;
pub use multi_mapping::MultiMapping;
pub use raw_mapping::RawMapping;

pub trait TypeGuard<'a> {
    type Type: 'a;
}

impl<'a> TypeGuard<'a> for () {
    type Type = ();
}

pub struct ValueGuard<K>(PhantomData<K>);

impl<'a, K: 'a> TypeGuard<'a> for ValueGuard<K> {
    type Type = K;
}

pub struct RefGuard<K>(PhantomData<K>);

impl<'a, K: 'a> TypeGuard<'a> for RefGuard<K> {
    type Type = &'a K;
}
