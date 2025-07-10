mod bip39;
mod language;
mod mnemonic;

pub use bip39::*;
pub use language::Language;

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
}
