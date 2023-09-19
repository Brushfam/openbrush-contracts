// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::{
    psp34,
    psp34::extensions::metadata,
    traits::psp34::{
        extensions::metadata::*,
        *,
    },
};
pub use metadata::Internal as _;
pub use openbrush::traits::String;
use openbrush::{
    storage::{
        Mapping,
        TypeGuard,
    },
    traits::Storage,
};
pub use psp34::{
    BalancesManager as _,
    Internal as _,
    InternalImpl as _,
    Operator,
    Owner,
    PSP34Impl,
};

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    pub attributes: Mapping<(Id, String), String, AttributesKey>,
}

pub struct AttributesKey;

impl<'a> TypeGuard<'a> for AttributesKey {
    type Type = &'a (&'a Id, &'a String);
}

pub trait PSP34MetadataImpl: Storage<Data> {
    fn get_attribute(&self, id: Id, key: String) -> Option<String> {
        self.data().attributes.get(&(&id, &key))
    }
}

pub trait Internal {
    /// Event is emitted when an attribute is set for a token.
    fn _emit_attribute_set_event(&self, id: Id, key: String, data: String);

    fn _set_attribute(&mut self, id: Id, key: String, value: String);
}

pub trait InternalImpl: Internal + Storage<Data> {
    fn _emit_attribute_set_event(&self, _id: Id, _key: String, _data: String) {}

    fn _set_attribute(&mut self, id: Id, key: String, value: String) {
        self.data().attributes.insert(&(&id, &key), &value);
        Internal::_emit_attribute_set_event(self, id, key, value);
    }
}
