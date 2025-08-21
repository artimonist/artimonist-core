mod animate;
mod complex;
#[allow(clippy::module_inception)]
mod diagram;
mod generic;
mod simple;

pub use animate::AnimateDiagram;
pub use complex::ComplexDiagram;
pub use diagram::Diagram;
pub use generic::GenericDiagram;
pub use simple::SimpleDiagram;

#[cfg(feature = "serde")]
pub use generic::{Matrix, ToMatrix};

type Result<T = ()> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// String too long
    #[error("string too long: {0}")]
    StringTooLong(String),
    /// Bip32 error
    #[error("Derive error: {0}")]
    Bip32Err(#[from] bitcoin::bip32::Error),
    /// Argon2 error
    #[error("Encrypt error: {0}")]
    EncryptErr(String),
    /// Decode error
    #[cfg(feature = "serde")]
    #[error("Serde error: {0}")]
    SerdeErr(String),
}

macro_rules! derive_error {
    ($e:expr, $source:ty) => {
        impl From<$source> for Error {
            fn from(e: $source) -> Self {
                $e(e.to_string())
            }
        }
    };
}

derive_error!(Error::EncryptErr, scrypt::errors::InvalidParams);
derive_error!(Error::EncryptErr, scrypt::errors::InvalidOutputLen);
derive_error!(Error::EncryptErr, argon2::Error);
#[cfg(feature = "serde")]
derive_error!(Error::SerdeErr, rmp_serde::encode::Error);
#[cfg(feature = "serde")]
derive_error!(Error::SerdeErr, rmp_serde::decode::Error);
