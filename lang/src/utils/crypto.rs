// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

use crate::traits::String;
use ink::env::hash;

use crate::traits::AccountId;

/// Hashing function for bytes
pub fn hash_blake2b256(input: &[u8]) -> [u8; 32] {
    let mut output = <hash::Blake2x256 as hash::HashOutput>::Type::default();
    ink::env::hash_bytes::<hash::Blake2x256>(input, &mut output);
    output
}

/// Converts a compressed public key to SS58 format
pub fn pub_key_to_ss58(pub_key: &[u8; 33]) -> AccountId {
    AccountId::from(hash_blake2b256(pub_key))
}

/// Converts a public key to an Ethereum address
pub fn pub_key_to_eth_address(pub_key: &[u8; 33]) -> Result<[u8; 20], CryptoError> {
    let mut output = [0u8; 20];

    ink::env::ecdsa_to_eth_address(pub_key, &mut output).map_err(|_| CryptoError::EcdsaToEthAddressFailed)?;

    Ok(output)
}

/// Enum to represent different signature types
///
/// # Support of signatures
///
/// - `ECDSA` - ECDSA signature with 65 bytes
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Signature {
    ECDSA([u8; 65]),
}

impl Signature {
    /// Verifies different type of signatures
    ///
    /// # Arguments
    ///
    /// - `message` - The message to verify
    /// - `pub_key` - The public key to verify the message with
    ///
    /// # Returns
    ///
    /// - `true` if the signature is valid
    /// - `false` if the signature is invalid
    ///
    /// # Supported signatures
    ///
    /// - `ECDSA`
    #[allow(unreachable_patterns)]
    pub fn verify(&self, message: &[u8], address: &AccountId) -> bool {
        match self {
            // Verifies ECDSA signature
            Signature::ECDSA(sig) => {
                let mut output: [u8; 33] = [0; 33];
                let message_hash = hash_blake2b256(message);

                let result = ink::env::ecdsa_recover(sig, &message_hash, &mut output);

                return result.is_ok() && pub_key_to_ss58(&output) == address.clone()
            }
            _ => false,
        }
    }
}

#[derive(scale::Decode, scale::Encode, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
pub enum CryptoError {
    EcdsaRecoverFailed,
    EcdsaToEthAddressFailed,
    Other(String),
}
