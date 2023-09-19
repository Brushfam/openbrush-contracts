// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::{
    psp37,
    psp37::extensions::metadata,
    traits::psp37::{
        extensions::metadata::*,
        *,
    },
};
pub use metadata::Internal as _;
use openbrush::{
    storage::{
        Mapping,
        TypeGuard,
    },
    traits::{
        Storage,
        String,
    },
};
pub use psp37::{
    BalancesManager as _,
    BalancesManagerImpl as _,
    Internal as _,
    InternalImpl as _,
    PSP37Impl,
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

pub trait PSP37MetadataImpl: Storage<Data> {
    fn get_attribute(&self, id: Id, key: String) -> Option<String> {
        self.data().attributes.get(&(&id, &key))
    }
}

pub trait Internal {
    fn _emit_attribute_set_event(&self, _id: &Id, _key: &String, _data: &String);

    fn _set_attribute(&mut self, id: &Id, key: &String, data: &String) -> Result<(), PSP37Error>;

    fn _get_attribute(&self, id: &Id, key: &String) -> Option<String>;
}

pub trait InternalImpl: Internal + Storage<Data> {
    fn _emit_attribute_set_event(&self, _id: &Id, _key: &String, _data: &String) {}

    fn _set_attribute(&mut self, id: &Id, key: &String, data: &String) -> Result<(), PSP37Error> {
        self.data().attributes.insert(&(id, key), data);
        Internal::_emit_attribute_set_event(self, id, key, data);
        Ok(())
    }

    fn _get_attribute(&self, id: &Id, key: &String) -> Option<String> {
        self.data().attributes.get(&(id, key))
    }
}
