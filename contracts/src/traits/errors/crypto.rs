use openbrush::traits::String;

#[derive(scale::Decode, scale::Encode, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
pub enum CryptoError {
    EcdsaRecoverFailed,
    EcdsaToEthAddressFailed,
    Other(String),
}
