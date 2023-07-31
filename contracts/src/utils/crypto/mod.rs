use ink::primitives::AccountId;
use openbrush::traits::String;

pub type SignatureType = [u8; 65];

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

pub fn hash_message(message: &[u8]) -> Result<[u8; 32], CryptoError> {
    let mut output = [0u8; 32];

    ink::env::hash_bytes::<ink::env::hash::Blake2x256>(message, &mut output);

    Ok(output)
}

pub fn pub_key_to_ss58(pub_key: &[u8; 33]) -> Result<AccountId, CryptoError> {
    hash_message(pub_key).map(|hash| ink::primitives::AccountId::from(hash))
}

pub fn pub_key_to_eth_address(pub_key: &[u8; 33]) -> Result<[u8; 20], CryptoError> {
    let mut output = [0u8; 20];

    ink::env::ecdsa_to_eth_address(pub_key, &mut output).map_err(|_| CryptoError::EcdsaToEthAddressFailed)?;

    Ok(output)
}

#[derive(scale::Decode, scale::Encode, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
pub enum CryptoError {
    EcdsaRecoverFailed,
    EcdsaToEthAddressFailed,
    Other(String),
}
