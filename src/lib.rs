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
//! # #[cfg(not(feature = "testnet"))]
//! assert_eq!(master.bip85_wif(0)?.pk, "L25LxS22MwRpEnnFs81XitJyrkimpZGLjgKHRAikLxJoxWMkVuHd");
//! # #[cfg(not(feature = "testnet"))]
//! assert_eq!(master.bip85_xpriv(0)?, "xprv9s21ZrQH143K47Cxw6R8QnGdAru5BaK7kT5awzC9VvmpXnpCQPdEmPyJeR9w3FeJ3hmEBRCRLGhMNpnkcM9q2w3J3T55bSSqMLRDpJLZU4B");
//! # #[cfg(not(feature = "testnet"))]
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
pub(crate) mod bip85;
pub(crate) mod complex;
pub(crate) mod derive;
pub(crate) mod generic;
pub(crate) mod macros;
pub(crate) mod matrix;
pub(crate) mod password;
pub(crate) mod simple;
pub(crate) mod words;

#[doc(no_inline)]
pub use bitcoin::{self, bip32::Xpriv};

pub use bip39::Derivation as BIP39;
pub use bip85::{Derivation as BIP85, Language, Password, Wif};
pub use complex::ComplexDiagram;
pub use derive::{Bip44 as BIP44, Bip49 as BIP49, Bip84 as BIP84};
pub use generic::GenericDiagram;
pub use matrix::{Matrix, ToMatrix};
pub use simple::SimpleDiagram;

///
/// Global error definition
///
pub mod error {
    /// Artimonist Error
    #[derive(thiserror::Error, Debug)]
    pub enum Error {
        /// Invalid parameter
        #[error("invalid parameter: {0}")]
        InvalidParameter(&'static str),
        /// Bip32 Error
        #[error("bip32 error")]
        Bip32Error(#[from] bitcoin::bip32::Error),
        /// Secp error
        #[error("runtime error")]
        SecpError(#[from] bitcoin::secp256k1::Error),
        /// Hex parse error
        #[error("hex error")]
        HexError(#[from] bitcoin::hex::HexToArrayError),
        /// Address error
        #[error("address error")]
        AddressError(#[from] bitcoin::key::UncompressedPublicKeyError),
        /// P2sh error
        #[error("p2sh error")]
        P2shError(#[from] bitcoin::address::P2shError),
        #[cfg(feature = "serde")]
        /// serialize error
        #[error("serialize error")]
        Serialize(#[from] rmp_serde::encode::Error),
        #[cfg(feature = "serde")]
        /// deserialize eror
        #[error("deserialize error")]
        Deserialize(#[from] rmp_serde::decode::Error),
    }
}
pub use error::Error;

/// Bitcoin network
#[cfg(not(feature = "testnet"))]
pub const NETWORK: bitcoin::NetworkKind = bitcoin::NetworkKind::Main;
/// Bitcoin network
#[cfg(feature = "testnet")]
pub const NETWORK: bitcoin::NetworkKind = bitcoin::NetworkKind::Test;
