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
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub attributes: Mapping<(Id, String), String, AttributesKey>,
    pub _reserved: Option<()>,
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
        self.data().attributes.insert(&(&id, &key), data);
        Internal::_emit_attribute_set_event(self, id, key, data);
        Ok(())
    }

    fn _get_attribute(&self, id: &Id, key: &String) -> Option<String> {
        self.data().attributes.get(&(&id, &key))
    }
}
