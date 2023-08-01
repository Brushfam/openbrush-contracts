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
use ink::prelude::vec::Vec;

#[derive(scale::Decode, scale::Encode, Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
pub struct Checkpoints {
    pub checkpoints: Vec<Checkpoint>,
}

#[derive(scale::Decode, scale::Encode, Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
pub struct Checkpoint {
    pub key: u64,
    pub value: u128,
}

fn sqrt(x: u64) -> u64 {
    let mut z = (x + 1) / 2;
    let mut y = x;
    while z < y {
        y = z;
        z = (x / z + z) / 2;
    }
    y
}

impl Checkpoints {
    /// Pushes a (`key`, `value`) pair into a Trace224 so that it is stored as the checkpoint.
    /// Returns previous value and new value.
    pub fn push(&mut self, key: u64, value: u128) -> Result<(u128, u128), CheckpointsError> {
        self._insert(key, value)
    }

    /// Returns the value in the first (oldest) checkpoint with key greater or equal than the search key, or zero if there is none.
    pub fn lower_lookup(&self, key: u64) -> Option<u128> {
        let len = self.checkpoints.len();
        let pos = self._lower_binary_lookup(key, 0, len);
        match pos == len {
            true => None,
            false => Some(self.checkpoints[pos].value),
        }
    }

    /// Returns the value in the last (most recent) checkpoint with key lower or equal than the search key, or zero if there is none.
    pub fn upper_lookup(&self, key: u64) -> Option<u128> {
        let len = self.checkpoints.len();
        let pos = self._upper_binary_lookup(key, 0, len);
        match pos == 0 {
            true => None,
            false => Some(self.checkpoints[pos - 1].value),
        }
    }

    /// Returns the value in the last (most recent) checkpoint with key lower or equal than the search key, or zero if there is none.
    ///
    /// NOTE: This is a variant of {upperLookup} that is optimised to find "recent" checkpoint (checkpoints with high keys).
    pub fn upper_lookup_recent(&self, key: u64) -> Option<u128> {
        let len = self.checkpoints.len();

        let mut low = 0;
        let mut high = len;

        if len > 5 {
            let mid = len - sqrt(len as u64) as usize;
            if key < self.checkpoints[mid].key {
                high = mid;
            } else {
                low = mid + 1;
            }
        }

        let pos = self._upper_binary_lookup(key, low, high);

        match pos == 0 {
            true => None,
            false => Some(self.checkpoints[pos - 1].value),
        }
    }

    /// Returns the value in the most recent checkpoint, or 0 if there are no checkpoints.
    pub fn latest(&self) -> u128 {
        let len = self.checkpoints.len();
        match len == 0 {
            true => 0,
            false => self.checkpoints[len - 1].value,
        }
    }

    /// Returns whether there is a checkpoint in the structure (i.e. it is not empty), and if so the key and value
    /// in the most recent checkpoint.
    pub fn latest_checkpoint(&self) -> (bool, u64, u128) {
        let pos = self.checkpoints.len();

        match pos == 0 {
            true => (false, 0, 0),
            false => {
                let checkpoint = &self.checkpoints[pos - 1];
                (true, checkpoint.key, checkpoint.value)
            }
        }
    }

    /// Returns the number of checkpoint.
    pub fn len(&self) -> usize {
        self.checkpoints.len()
    }

    pub fn at(&self, index: usize) -> Option<&Checkpoint> {
        let len = self.checkpoints.len();
        match index < len {
            true => Some(&self.checkpoints[index]),
            false => None,
        }
    }

    fn _insert(&mut self, key: u64, value: u128) -> Result<(u128, u128), CheckpointsError> {
        let pos = self.checkpoints.len();

        if pos > 0 {
            let last = self.checkpoints[pos - 1].clone();

            if last.key > key {
                return Err(CheckpointsError::UnorderedInsertion)
            }

            if last.key == key {
                self.checkpoints[pos - 1].value = value;
            } else {
                self.checkpoints.push(Checkpoint { key, value });
            }
            Ok((last.value, value))
        } else {
            self.checkpoints.push(Checkpoint { key, value });
            Ok((0, value))
        }
    }

    fn _upper_binary_lookup(&self, key: u64, mut low: usize, mut high: usize) -> usize {
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

    fn _lower_binary_lookup(&self, key: u64, mut low: usize, mut high: usize) -> usize {
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
    #[test]
    fn push_works() {
        let mut checkpoints = Checkpoints::default();

        let (last, value) = checkpoints.push(1, 1).unwrap();
        assert_eq!(last, 0);
        assert_eq!(value, 1);
        assert_eq!(checkpoints.len(), 1);
    }

    #[test]
    fn lower_lookup_works() {
        let mut checkpoints = Checkpoints::default();
        checkpoints.push(1, 1).unwrap();
        checkpoints.push(2, 2).unwrap();
        checkpoints.push(5, 5).unwrap();
        assert_eq!(checkpoints.lower_lookup(0), Some(1));
        assert_eq!(checkpoints.lower_lookup(1), Some(1));
        assert_eq!(checkpoints.lower_lookup(2), Some(2));
        assert_eq!(checkpoints.lower_lookup(3), Some(5));
        assert_eq!(checkpoints.lower_lookup(5), Some(5));
        assert_eq!(checkpoints.lower_lookup(6), None);
    }

    #[test]
    fn upper_lookup_works() {
        let mut checkpoints = Checkpoints::default();

        checkpoints.push(1, 1).unwrap();
        checkpoints.push(2, 2).unwrap();
        checkpoints.push(5, 5).unwrap();
        assert_eq!(checkpoints.upper_lookup(0), None);
        assert_eq!(checkpoints.upper_lookup(1), Some(1));
        assert_eq!(checkpoints.upper_lookup(2), Some(2));
        assert_eq!(checkpoints.upper_lookup(3), Some(2));
        assert_eq!(checkpoints.upper_lookup(5), Some(5));
        assert_eq!(checkpoints.upper_lookup(6), Some(5));
    }

    #[test]
    fn upper_lookup_recent_works() {
        let mut checkpoints = Checkpoints::default();

        checkpoints.push(1, 1).unwrap();
        checkpoints.push(2, 2).unwrap();
        checkpoints.push(5, 5).unwrap();
        assert_eq!(checkpoints.upper_lookup_recent(0), None);
        assert_eq!(checkpoints.upper_lookup_recent(1), Some(1));
        assert_eq!(checkpoints.upper_lookup_recent(2), Some(2));
        assert_eq!(checkpoints.upper_lookup_recent(3), Some(2));
        assert_eq!(checkpoints.upper_lookup_recent(5), Some(5));
        assert_eq!(checkpoints.upper_lookup_recent(6), Some(5));
    }

    #[test]
    fn latest_works() {
        let mut checkpoints = Checkpoints::default();
        assert_eq!(checkpoints.latest(), 0);
        checkpoints.push(1, 1).unwrap();
        checkpoints.push(2, 2).unwrap();
        checkpoints.push(5, 5).unwrap();
        assert_eq!(checkpoints.latest(), 5);
    }

    #[test]
    fn latest_checkpoint_works() {
        let mut checkpoints = Checkpoints::default();
        assert_eq!(checkpoints.latest_checkpoint(), (false, 0, 0));
        checkpoints.push(1, 1).unwrap();
        checkpoints.push(2, 2).unwrap();
        checkpoints.push(5, 5).unwrap();
        assert_eq!(checkpoints.latest_checkpoint(), (true, 5, 5));
    }

    #[test]
    fn len_works() {
        let mut checkpoints = Checkpoints::default();
        assert_eq!(checkpoints.len(), 0);
        checkpoints.push(1, 1).unwrap();
        checkpoints.push(2, 2).unwrap();
        checkpoints.push(5, 5).unwrap();
        assert_eq!(checkpoints.len(), 3);
    }

    #[test]
    fn test_sqrt() {
        for i in 0..(1 << 16) {
            assert_eq!(sqrt(i * i), i);
        }
    }
}
