// Copyright (c) 2012-2023 727.ventures
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

pub use crate::traits::errors::CheckpointsError;
use openbrush::{
    storage::Mapping,
    traits::{AccountId, Storage},
};

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Checkpoints {
    pub checkpoints: Vec<Checkpoint>,
}

pub struct Checkpoint {
    pub key: u32,
    pub value: u128,
}

impl Checkpoints {
    pub fn push(&mut self, key: u32, value: u128) {
        self._insert(key, value);
    }

    pub fn lower_lookup(&self, key: u32) -> u128 {
        let len = self.checkpoints.len();
        let pos = self._lower_binary_lookup(key, 0, len);
        match pos == len {
            true => 0,
            false => self.checkpoints[pos].value,
        }
    }

    pub fn upper_lookup(&self, key: u32) -> u128 {
        let len = self.checkpoints.len();
        let pos = self._upper_binary_lookup(key, 0, len);
        match pos == 0 {
            true => 0,
            false => self.checkpoints[pos-1].value,
        }
    }

    pub fn upper_lookup_recent(&self, key: u32) -> u128 {
        let len = self.checkpoints.len();

        let mut low = 0;
        let mut high = len;

        if len > 5 {
            let mid = len - f64::sqrt(len as f64) as usize;
            if key < self.checkpoints[mid].key {
                high = mid;
            } else {
                low = mid + 1;
            }
        }

        let pos = self._upper_binary_lookup(key, low, high);

        match pos == 0 {
            true => 0,
            false => self.checkpoints[pos-1].value,
        }
    }

    pub fn latest(&self) -> u128 {
        let len = self.checkpoints.len();
        match len == 0 {
            true => 0,
            false => self.checkpoints[len-1].value,
        }
    }

    pub fn latest_checkpoint(&self) -> (bool, u32, u128) {
        let pos = self.checkpoints.len();

        match pos == 0 {
            true => (false, 0, 0),
            false => {
                let checkpoint = &self.checkpoints[pos-1];
                (true, checkpoint.key, checkpoint.value)
            },
        }
    }

    fn _insert(&mut self, key: u32, value: u128) -> Result<(u128, u128), CheckpointsError> {
        let pos = self.checkpoints.len();

        if pos > 0 {
            let last = &self.checkpoints[pos-1];

            if last.key > key {
                return Err(CheckpointsError::UnorderedInsertion);
            }

            if last.key == key {
                self.checkpoints[pos-1].value = value;
            } else {
                self.checkpoints.push(Checkpoint { key, value });
            }
            Ok((last.value, value))
        } else {
            self.checkpoints.push(Checkpoint { key, value });
            Ok((0, value))
        }
    }

    fn _upper_binary_lookup(
        &self,
        key: u32,
        mut low: usize,
        mut high: usize,
    ) -> usize {
        while low < high {
            let mid = low / 2 + high / 2;
            if key < self.checkpoints[mid].key {
                high = mid;
            } else {
                low = mid + 1;
            }
        }
        high
    }

    fn _lower_binary_lookup(
        &self,
        key: u32,
        mut low: usize,
        mut high: usize,
    ) -> usize {
        while low < high {
            let mid = low / 2 + high / 2;
            if key > self.checkpoints[mid].key {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        high
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[ink::test]
    fn push_works() {

    }
}