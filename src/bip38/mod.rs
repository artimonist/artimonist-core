mod bip38_inner;
mod mnemonic;

pub use bip38_inner::Bip38;
pub use mnemonic::MnemonicEncryption;

#[derive(thiserror::Error, Debug)]
pub enum Bip38Error {
    #[error("Invalid encrypted key")]
    InvalidKey,
    #[error("Invalid passphrase")]
    InvalidPassphrase,
    #[error("Invalid lot or sequence number: lot: {0}, seq: {1}")]
    InvalidEcNumber(u32, u32),
    #[error("Invalid ec passphrase")]
    InvalidEcFactor,
    #[error("Base58 error: {0}")]
    Base58Error(#[from] bitcoin::base58::Error),
    #[error("Invalid WIF: {0}")]
    InvalidWif(#[from] bitcoin::key::FromWifError),
    #[error("Mnemonic error: {0}")]
    MnemonicError(#[from] crate::bip39::Bip39Error),
    #[error("Invalid word count: {0}")]
    InvalidWordCount(usize),
    #[error("Inner error: {0}")]
    InnerError(String),
}

macro_rules! derive_error {
    ($e:expr, $source:ty) => {
        impl From<$source> for Bip38Error {
            fn from(e: $source) -> Self {
                $e(e.to_string())
            }
        }
    };
}
derive_error!(Bip38Error::InnerError, aes::cipher::InvalidLength);
derive_error!(Bip38Error::InnerError, scrypt::errors::InvalidOutputLen);
derive_error!(Bip38Error::InnerError, scrypt::errors::InvalidParams);
derive_error!(Bip38Error::InnerError, bitcoin::secp256k1::Error);
derive_error!(Bip38Error::InnerError, bitcoin::key::FromSliceError);
derive_error!(
    Bip38Error::InnerError,
    bitcoin::secp256k1::scalar::OutOfRangeError
);
derive_error!(Bip38Error::InnerError, argon2::Error);
derive_error!(Bip38Error::InnerError, bitcoin::bip32::Error);
