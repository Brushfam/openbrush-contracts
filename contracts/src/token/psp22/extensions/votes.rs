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

use crate::{
    checkpoint::Checkpoint,
    governance::utils::votes::VotesInternal,
    psp22,
    psp22::PSP22Error,
    traits::errors::GovernanceError,
};
use ink::prelude::vec;
use openbrush::traits::AccountId;

pub trait PSP22VotesImpl: VotesInternal {
    fn num_checkpoints(&self, account: AccountId) -> Result<u32, GovernanceError> {
        VotesInternal::_num_checkpoints(self, &account)
    }

    fn checkpoints(&self, account: AccountId, pos: u32) -> Result<Checkpoint, GovernanceError> {
        VotesInternal::_checkpoints(self, &account, pos)
    }
}

pub trait PSP22VotesInternal: VotesInternal + psp22::Internal {
    fn _max_supply(&self) -> u128 {
        u128::MAX
    }

    fn _update(&mut self, from: AccountId, to: AccountId, amount: u128) -> Result<(), PSP22Error> {
        psp22::Internal::_transfer_from_to(self, from, to, amount, vec![])?;

        Ok(())
    }

    fn _get_voting_units(&self, account: &AccountId) -> u128 {
        psp22::Internal::_balance_of(self, account)
    }
}
