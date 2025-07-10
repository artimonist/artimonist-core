mod bip39_inner;
mod language;
mod mnemonic;

pub use bip39_inner::*;
pub use language::Language;
pub use mnemonic::Mnemonic;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Bip39Error {
    #[error("invalid BIP39 language")]
    LanguageError,

    #[error("invalid mnemonic length")]
    InvalidLength,

    #[error("invalid mnemonic language")]
    InvalidLanguage,

    #[error("inconclusive mnemonic language")]
    InconclusiveLanguage(Vec<Language>),

    #[error("invalid mnemonic checksum")]
    InvalidChecksum,

    #[error("bip32 error")]
    Bip32Error(#[from] bitcoin::bip32::Error),
}
