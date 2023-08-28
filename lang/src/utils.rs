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

pub use const_format;
pub use xxhash_rust;

use crate::traits::AccountId;
use xxhash_rust::const_xxh32::xxh32;

/// The value 0 is a valid seed.
const XXH32_SEED: u32 = 0;

pub struct ConstHasher;

impl ConstHasher {
    pub const fn hash(str: &str) -> u32 {
        xxh32(str.as_bytes(), XXH32_SEED)
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, scale::Encode, scale::Decode)]
pub enum SignatureType {
    #[default]
    ECDSA,
    SR25519,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, scale::Encode, scale::Decode)]
pub struct Signature {
    pub signature_type: SignatureType,
    pub raw_signature: [u8],
}

impl Signature {
    pub fn verify(&self, message: &[u8], pub_key: &AccountId) -> bool {
        match self.signature_type {
            SignatureType::ECDSA => ink::env::ecdsa_recover(&self.raw_signature.into(), message.into()).is_ok(),
            SignatureType::SR25519 => {
                ink::env::sr25519_verify(&self.raw_signature.into(), message, pub_key.as_ref()).is_ok()
            }
        }
    }
}
