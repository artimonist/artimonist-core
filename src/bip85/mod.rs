#[allow(clippy::module_inception)]
mod bip85;
mod password;

pub use bip85::{Bip85, Wif};
pub use password::Password;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Invalid parameter
    #[error("invalid parameter: {0}")]
    InvalidParameter(&'static str),

    /// Bip39 error
    #[error("bip39 error: {0}")]
    Bip39Error(#[from] crate::bip39::Bip39Error),

    /// Address error
    #[error("address error")]
    AddressError(#[from] bitcoin::key::UncompressedPublicKeyError),

    /// P2sh error
    #[error("p2sh error")]
    P2shError(#[from] bitcoin::address::P2shError),

    /// Bip32 Error
    #[error("bip32 error: {0}")]
    Bip32Error(#[from] bitcoin::bip32::Error),

    /// Secp error
    #[error("runtime error")]
    SecpError(#[from] bitcoin::secp256k1::Error),

    /// Hex parse error
    #[error("hex error")]
    HexError(#[from] bitcoin::hex::HexToArrayError),
}
