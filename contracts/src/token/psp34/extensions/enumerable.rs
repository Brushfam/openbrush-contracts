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
    psp34,
    psp34::extensions::enumerable,
    traits::psp34::{
        extensions::enumerable::*,
        *,
    },
};
pub use ink::prelude::vec::Vec;
use openbrush::{
    storage::{
        MultiMapping,
        TypeGuard,
    },
    traits::{
        AccountId,
        Balance,
        Storage,
    },
};
pub use psp34::{
    BalancesManager as _,
    Internal as _,
    InternalImpl as _,
    Operator,
    Owner,
    PSP34Impl,
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub enumerable: MultiMapping<Option<AccountId>, Id, EnumerableKey /* optimization */>,
    pub _reserved: Option<()>,
}

#[cfg(feature = "upgradeable")]
pub type DataType = Lazy<Data>;
#[cfg(not(feature = "upgradeable"))]
pub type DataType = Data;

pub struct EnumerableKey;

impl<'a> TypeGuard<'a> for EnumerableKey {
    type Type = &'a Option<&'a AccountId>;
}

pub trait BalancesManagerImpl: Storage<Data> {
    fn _balance_of(&self, owner: &Owner) -> u32 {
        self.data().enumerable.count(&Some(owner)) as u32
    }

    fn _increase_balance(&mut self, owner: &Owner, id: &Id, increase_supply: bool) {
        self.data().enumerable.insert(&Some(owner), id);
        if increase_supply {
            self.data().enumerable.insert(&None, id);
        }
    }

    fn _decrease_balance(&mut self, owner: &Owner, id: &Id, decrease_supply: bool) {
        self.data().enumerable.remove_value(&Some(owner), id);
        if decrease_supply {
            self.data().enumerable.remove_value(&None, id);
        }
    }

    fn _total_supply(&self) -> Balance {
        self.data().enumerable.count(&None)
    }
}

pub trait PSP34EnumerableImpl: Storage<Data> {
    fn owners_token_by_index(&self, owner: AccountId, index: u128) -> Result<Id, PSP34Error> {
        self.data()
            .enumerable
            .get_value(&Some(&owner), &index)
            .ok_or(PSP34Error::TokenNotExists)
    }

    fn token_by_index(&self, index: u128) -> Result<Id, PSP34Error> {
        self.data()
            .enumerable
            .get_value(&None, &index)
            .ok_or(PSP34Error::TokenNotExists)
    }
}
