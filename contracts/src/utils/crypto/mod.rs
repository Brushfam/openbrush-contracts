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

use crate::traits::{errors::CryptoError, types::SignatureType};
use ink::primitives::AccountId;

/// Verifies the signature of a message hash with the account
pub fn verify_signature(
    message_hash: &[u8; 32],
    account: &AccountId,
    signature: &SignatureType,
) -> Result<bool, CryptoError> {
    let mut output = [0u8; 33];

    ink::env::ecdsa_recover(&signature, message_hash, &mut output).map_err(|_| CryptoError::EcdsaRecoverFailed)?;

    let recovered_account = pub_key_to_ss58(&output)?;

    if recovered_account != account.clone() {
        return Ok(false);
    }

    Ok(true)
}

/// Hashes a message
pub fn hash_message(message: &[u8]) -> Result<[u8; 32], CryptoError> {
    let mut output = [0u8; 32];

    ink::env::hash_bytes::<ink::env::hash::Blake2x256>(message, &mut output);

    Ok(output)
}

/// Converts a public key to SS58
pub fn pub_key_to_ss58(pub_key: &[u8; 33]) -> Result<AccountId, CryptoError> {
    hash_message(pub_key).map(|hash| AccountId::from(hash))
}

/// Converts a public key to an Ethereum address
pub fn pub_key_to_eth_address(pub_key: &[u8; 33]) -> Result<[u8; 20], CryptoError> {
    let mut output = [0u8; 20];

    ink::env::ecdsa_to_eth_address(pub_key, &mut output).map_err(|_| CryptoError::EcdsaToEthAddressFailed)?;

    Ok(output)
}
