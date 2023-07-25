// Copyright (c) 2023 Brushfam
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
    governance::governance::*,
    traits::governance::{
        extensions::{
            counting::*,
            votes::*,
        },
        utils::votes::*,
        *,
    },
};
use openbrush::traits::{AccountId, Balance, StorageAsRef, Timestamp};
pub use governance::governance::{
    Internal as _,
    InternalImpl as _,
    GovernorImpl,
};
use openbrush::storage::Mapping;
use openbrush::traits::Storage;

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    #[lazy]
    token: AccountId,
}

pub trait GovernorCountingImpl: governor::Internal + Internal + Storage<Data> + GovernorImpl{
    fn clock(&self) -> u64 {
        //todo
        //VotesRef::clock_builder(&token).invoke()
        Self::env().block_number() as u64
    }

    fn clock_mode(&self) -> String {
        //todo
        //VotesRef::clock_mode_builder()
        "mode=blocknumber&from=default".to_string()
    }
}

pub trait Internal {
    fn _get_votes(&self, account: AccountId, timestamp: Timestamp) -> u128;
}

pub trait InternalImpl: Internal + Storage<Data> + GovernorCountingImpl + GovernorImpl {
    fn _get_votes(&self, account: AccountId, timestamp: Timestamp) -> u128 {
        //todo
        VotesRef::get_past_votes_builder(&token, &account, &timestamp).invoke()
    }
}