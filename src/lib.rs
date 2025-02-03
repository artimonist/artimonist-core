#![warn(missing_docs)]
//! # Artimonist
//!
//! `Artimonist` is a chart-based tool for creating mnemonics.
//!
//! Try the live web page: <https://www.artimonist.org>
//!
//! # Examples
//! ```
//! use artimonist::{SimpleDiagram, GenericDiagram, BIP85, Language, Password, Wif};
//!
//! let values = vec!['🍔', '🍟', '🌭', '🍦', '🍩'];
//! let indices = vec![(1, 1), (1, 5), (5, 5), (5, 1), (3, 3)];
//! let diagram = SimpleDiagram::from_values(&values, &indices);
//! let master = diagram.bip32_master("🚲🍀🌈".as_bytes())?;
//!
//! let mnemonic = master.bip85_mnemonic(Language::English, 15, 0)?;
//! assert_eq!(&mnemonic, "lady announce wife please settle connect april hour caution split festival genuine logic digital dignity");
//!
//! assert_eq!(master.bip85_wif(0)?, "L25LxS22MwRpEnnFs81XitJyrkimpZGLjgKHRAikLxJoxWMkVuHd");
//! assert_eq!(master.bip85_xpriv(0)?, "xprv9s21ZrQH143K47Cxw6R8QnGdAru5BaK7kT5awzC9VvmpXnpCQPdEmPyJeR9w3FeJ3hmEBRCRLGhMNpnkcM9q2w3J3T55bSSqMLRDpJLZU4B");
//! assert_eq!(master.bip85_pwd(Password::Emoji, 20, 0)?, "🙏✋🍕🌻🎄🙏👍🔔🔔🍺💊🍄🍺⚡✋👌😍🚗🍎🚗");
//!
//! # Ok::<(), artimonist::Error>(())
//! ```
//! The simple diagram looks like this:
//!
//! |  |  |  |  |  |  |  |
//! |--|--|--|--|--|--|--|  
//! |  |🍔|  |  |  |🍟|  |
//! |  |  |  |  |  |  |  |
//! |  |  |  |🍩|  |  |  |
//! |  |  |  |  |  |  |  |
//! |  |🍦|  |  |  |🌭|  |
//! |  |  |  |  |  |  |  |
//!
pub(crate) mod bip39;
pub(crate) mod bip49;
pub(crate) mod bip85;
pub(crate) mod bits;
pub(crate) mod complex;
pub(crate) mod generic;
pub(crate) mod macros;
pub(crate) mod password;
pub(crate) mod simple;
pub(crate) mod words;

pub use bip39::Derivation as BIP39;
pub use bip49::Derivation as BIP49;
pub use bip85::{Derivation as BIP85, Language, Password, Wif};
#[doc(no_inline)]
pub use bitcoin::{self, bip32::Xpriv};
pub use complex::ComplexDiagram;
pub use generic::{GenericDiagram, VecDiagram};
pub use simple::SimpleDiagram;

///
/// Global error definition
///
pub mod error {
    pub use super::bip85::Bip85Error;
    pub use super::bitcoin::bip32::Error as Bip32Error;
    pub use super::generic::GenericError;

    use thiserror::Error;

    /// Artimonist Error
    #[derive(Error, Debug)]
    pub enum Error {
        /// Bip85 Error
        #[error("bip85 error")]
        Bip85Error(#[from] Bip85Error),
        /// Bip32 Error
        #[error("bip32 error")]
        Bip32Error(#[from] Bip32Error),
        /// Generic Error
        #[error("generic error")]
        GenericError(#[from] GenericError),
    }

    /// Artimonist Result
    pub type ArtResult<T = ()> = Result<T, Error>;
}

pub use error::Error;
