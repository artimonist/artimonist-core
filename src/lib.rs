#![warn(missing_docs)]
//! # Artimonist
//!
//! `Artimonist` is a chart-based tool for creating mnemonics and wallets.
//!
//! <p>
//!   <a href="https://github.com/artimonist/artimonist-cli/releases">
//!     <img alt="cli" src="https://img.shields.io/github/v/release/artimonist/cli?label=artimonist-cli">
//!   </a>&nbsp;&nbsp;&nbsp;
//!   <a href="https://www.artimonist.org">
//!     <img alt="web" src="https://img.shields.io/badge/artimonist.org-gray?logo=html5">
//!   </a>
//! </p>
//!
//! # Examples
//! ```
//! use artimonist::{SimpleDiagram, GenericDiagram, BIP85, Password, Wif};
//! let mut mx = [[None; 7]; 7];
//! mx[1][1] = Some('🍔');
//! mx[1][5] = Some('🍟');
//! mx[3][3] = Some('🍩');
//! mx[5][1] = Some('🍦');
//! mx[5][5] = Some('🌭');
//! let master = SimpleDiagram(mx).to_master("🚲🍀🌈".as_bytes())?;
//!
//! let mnemonic = master.bip85_mnemonic(0, 15, Default::default())?;
//! assert_eq!(&mnemonic, "nice dumb debris type pair until convince pet depend dragon seed exchange rebel belt alcohol");
//!
//! # #[cfg(not(feature = "testnet"))]
//! assert_eq!(master.bip85_wallet(0)?.pk, "L3X74sTsPBpoJx7MNWt36qnsGE1ubdf2BQVTczU7i14D94BhNdmc");
//! # #[cfg(not(feature = "testnet"))]
//! assert_eq!(master.bip85_master(0)?, "xprv9s21ZrQH143K3Dm1EavEpwYyfZD7su3245k5CES8qnt6VVxznxnPqMnUx5cFLJDao9mcyELm9UVNbBTMYodNabKmvcj3WazUaRNqpz4qd7A");
//! # #[cfg(not(feature = "testnet"))]
//! assert_eq!(master.bip85_password(0, 20, Password::Emoji)?, "🐬🔑🍎🐔🔒🚗🐸💧🌷✋🐍🚲💀💪💀🍄✈🍀🍉⚡");
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

mod bip38;
mod bip39;
mod bip85;
mod derive;
mod diagram;
mod macros;

#[doc(no_inline)]
pub use bitcoin::{self, bip32::Xpriv, bip32::Xpub};

pub use bip38::{Bip38 as BIP38, MnemonicEncryption};
pub use bip39::{Bip39 as BIP39, Language, Mnemonic};
pub use bip85::{Bip85 as BIP85, Password, Wif};
pub use derive::{Bip32 as BIP32, Bip44 as BIP44, Bip49 as BIP49, Bip84 as BIP84};
pub use diagram::{AnimateDiagram, ComplexDiagram, Diagram, GenericDiagram, SimpleDiagram};

#[cfg(feature = "serde")]
pub use diagram::{Matrix, ToMatrix};

///
/// Global error definition
///
pub mod error {
    /// Artimonist Error
    #[derive(thiserror::Error, Debug)]
    pub enum Error {
        /// Diagram error
        #[error("Diagram: {0}")]
        DiagramError(#[from] crate::diagram::Error),

        /// Bip38 Error
        #[error("BIP38: {0}")]
        Bip38Error(#[from] crate::bip38::Bip38Error),

        /// Bip39 Error
        #[error("BIP39: {0}")]
        Bip39Error(#[from] crate::bip39::Bip39Error),

        /// Bip85 Error
        #[error("BIP85: {0}")]
        Bip85Error(#[from] crate::bip85::Error),

        /// Derive Error
        #[error("Derive: {0}")]
        DeriveError(#[from] crate::derive::DeriveError),

        /// Bip32 Error
        #[error("bip32 error: {0}")]
        Bip32Error(#[from] bitcoin::bip32::Error),
    }
}
pub use error::Error;

/// Bitcoin network
#[cfg(not(feature = "testnet"))]
pub const NETWORK: bitcoin::NetworkKind = bitcoin::NetworkKind::Main;
/// Bitcoin network
#[cfg(feature = "testnet")]
pub const NETWORK: bitcoin::NetworkKind = bitcoin::NetworkKind::Test;
