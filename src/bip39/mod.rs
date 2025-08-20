#[allow(clippy::module_inception)]
mod bip39;
mod language;
mod mnemonic;

pub use bip39::Bip39;
pub use language::Language;
pub use mnemonic::Mnemonic;

/// BIP39 error types
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Bip39Error {
    #[error("invalid BIP39 language")]
    LanguageError,

    #[error("invalid mnemonic size")]
    InvalidSize,

    #[error("invalid mnemonic language")]
    InvalidLanguage,

    #[error("inconclusive mnemonic languages: {0:?}")]
    AmbiguousLanguages(Vec<Language>),

    #[error("invalid mnemonic checksum")]
    InvalidChecksum,

    #[error("bip32 error: {0}")]
    Bip32Error(#[from] bitcoin::bip32::Error),
}
