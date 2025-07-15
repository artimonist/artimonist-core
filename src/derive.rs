use bitcoin::{
    Address, CompressedPublicKey, PublicKey, ScriptBuf,
    bip32::{DerivationPath, Xpriv, Xpub},
    key::Secp256k1,
    script::Builder,
};
use std::str::FromStr;

pub trait DeriveInner {
    fn derive(&self, path: &str) -> Result<(Xpub, Xpriv), crate::Error>;
    fn multisig<const M: u8>(&self, paths: &[String]) -> Result<ScriptBuf, crate::Error>;
}

impl DeriveInner for Xpriv {
    /// Derive a key pair from derivation path
    #[inline]
    fn derive(&self, path_str: &str) -> Result<(Xpub, Xpriv), crate::Error> {
        let secp = Secp256k1::default();
        let path = DerivationPath::from_str(path_str)?;
        let xpriv = self.derive_priv(&secp, &path)?;
        let xpub = Xpub::from_priv(&secp, &xpriv);
        Ok((xpub, xpriv))
    }

    /// Create a multisig script
    /// # Arguments
    ///  - `M`: Number of required signatures
    ///  - `paths.len()`: Number of total signatures
    ///  - `paths`: Derivation paths
    /// # Returns
    ///  - multisig script
    fn multisig<const M: u8>(&self, paths: &[String]) -> Result<ScriptBuf, crate::Error> {
        assert!(
            M <= paths.len() as u8 && paths.len() <= 15,
            "[artimonist] Overflow: M <= paths.len() <= 15"
        );
        // collect public keys
        let secp = Secp256k1::default();
        let mut pub_keys = paths
            .iter()
            .map(|path| {
                let path = DerivationPath::from_str(path)?;
                let priv_key = self.derive_priv(&secp, &path)?.to_priv();
                Ok(PublicKey::from_private_key(&secp, &priv_key))
            })
            .collect::<Result<Vec<_>, crate::Error>>()?;
        pub_keys.sort();
        // create multisig script
        let script = pub_keys
            .iter()
            .fold(Builder::new().push_int(M as i64), |builder, key| {
                builder.push_key(key)
            })
            .push_int(pub_keys.len() as i64)
            .push_opcode(bitcoin::opcodes::all::OP_CHECKMULTISIG)
            .into_script();
        Ok(script)
    }
}

type DeriveResult = Result<(String, String), crate::Error>;

#[cfg(not(feature = "testnet"))]
const COIN: u8 = 0;
#[cfg(feature = "testnet")]
const COIN: u8 = 1;

/// BIP32 derivation
pub trait Bip32
where
    Self: DeriveInner,
{
    /// Derive a BIP32 account
    /// # Derivation path
    ///   m/0/0, m/0'/0', etc.
    /// # Returns
    ///   (xpub, xpriv)
    fn bip32_account(&self, path: &str) -> DeriveResult {
        self.derive(path)
            .map(|(xpub, xpriv)| (xpub.to_string(), xpriv.to_string()))
    }

    /// Derive a BIP32 wallet with custom path
    /// # Derivation path sample  
    ///   Electrum wallet derive, based on master key: `m/0/{index}`
    ///   Ref: <https://iancoleman.io/bip39/>
    ///     Bitcoin Core: `m/0'/0'/{index}'`
    ///     blockchain.info: `m/44'/0'/0'/{index}`
    ///     MultiBit HD: `m/0'/0/{index}`
    ///     Coinomi, Ledger: `m/44'/0'/0'/{index}`
    /// # Returns
    ///  (address, private_key): (p2pkh, wif)
    fn bip32_wallet(&self, path: &str) -> DeriveResult {
        self.derive(path).map(|(xpub, xpriv)| {
            let address = Address::p2pkh(CompressedPublicKey(xpub.public_key), crate::NETWORK);
            (address.to_string(), xpriv.to_priv().to_wif())
        })
    }
}

/// BIP44 derivation
pub trait Bip44
where
    Self: DeriveInner,
{
    /// Derive a BIP44 account
    /// # Derivation path
    ///   m/44'/0'/account'
    /// # Returns
    ///   (xpub, xpriv)
    fn bip44_account(&self, account: u32) -> DeriveResult {
        self.derive(&format!("m/44'/{COIN}'/{account}'"))
            .map(|(xpub, xpriv)| (xpub.to_string(), xpriv.to_string()))
    }

    /// Derive a wallet from a BIP44 account
    /// # Derivation path
    ///   m/44'/0'/account'/0/index
    /// # Returns
    ///   (address, private_key): (p2pkh, wif)
    fn bip44_wallet(&self, account: u32, index: u32, change: bool) -> DeriveResult {
        let change = if change { 1 } else { 0 };
        self.derive(&format!("m/44'/{COIN}'/{account}'/{change}/{index}"))
            .map(|(xpub, xpriv)| {
                let address = Address::p2pkh(CompressedPublicKey(xpub.public_key), crate::NETWORK);
                (address.to_string(), xpriv.to_priv().to_wif())
            })
    }

    /// Derive a wallet from a BIP44 account
    /// # Derivation path
    ///   m/44'/0'/account'/0/index'
    /// # Returns
    ///   (address, private_key): (p2pkh, wif)
    #[deprecated]
    fn bip44_harden(&self, account: u32, index: u32) -> DeriveResult {
        self.derive(&format!("m/44'/{COIN}'/{account}'/0/{index}'"))
            .map(|(xpub, xpriv)| {
                let address = Address::p2pkh(CompressedPublicKey(xpub.public_key), crate::NETWORK);
                (address.to_string(), xpriv.to_priv().to_wif())
            })
    }

    /// Derive a multisig wallet from BIP44 accounts
    /// # Parameters
    /// - `M`: Number of required signatures
    /// - `N`: Number of total signatures
    /// - `account`: Account start index (total use N accounts)
    /// - `index`: Wallet index
    /// # Derivation path
    ///   m/44'/0'/account'/0/index
    ///   m/44'/0'/(account + 1)'/0/index
    ///   m/44'/0'/(account + 2)'/0/index
    ///   ...
    /// # Returns
    ///   (address, redeem_script)
    fn bip44_multisig<const M: u8, const N: u8>(&self, account: u32, index: u32) -> DeriveResult {
        assert!(M <= N && N <= 15, "[artimonist] Overflow: M <= N <= 15");
        let paths = (account..account + N as u32)
            .map(|account: u32| format!("m/44'/{COIN}'/{account}'/0/{index}"))
            .collect::<Vec<_>>();
        let script = self.multisig::<M>(paths.as_slice())?;
        Ok((
            Address::p2sh(&script, crate::NETWORK)?.to_string(),
            script.to_hex_string(),
        ))
    }
}

/// BIP49 derivation  
/// Derivation scheme for P2WPKH-nested-in-P2SH based accounts.  
///
/// # Examples
/// ```
/// use artimonist::{BIP49, Xpriv, Error};
/// # use std::str::FromStr;
///
/// let master = Xpriv::from_str("xprv9s21ZrQH143K2sW69WDMTge7PMoK1bfeMy3cpNJxfSkqpPsU7DeHZmth8Sw7DVV2AMbC4jR3fKKgDEPJNNvsqhgTfyZwmWj439MWXUW5U5K")?;
/// let (addr, priv_key) = master.bip49_wallet(0, 12, false)?;
/// # #[cfg(not(feature = "testnet"))]
/// assert_eq!((addr.as_str(), priv_key.as_str()), ("3H8oU4pWZoVxrUt6huGhqAxecBTrpxjaYW", "L3nsXsh8rctbRKcL3xgRWZYALffdtKv43VpzDnbFFfAU6TAQxUgz"));
///
/// # Ok::<(), artimonist::Error>(())
/// ```
// # Reference
// [1] - [BIP49 spec](https://bips.dev/49/)
// [2] - [Ref website](https://iancoleman.io/bip39/)
pub trait Bip49
where
    Self: DeriveInner,
{
    /// Derive a BIP49 account
    /// # Derivation path
    ///   m/49'/0'/account'
    /// # Returns
    ///   (xpub, xpriv)
    fn bip49_account(&self, account: u32) -> DeriveResult {
        self.derive(&format!("m/49'/{COIN}'/{account}'"))
            .map(|(xpub, xpriv)| (xpub.to_ypub(), xpriv.to_ypriv()))
    }

    /// Derive a wallet from BIP49 account
    /// # Derivation path
    ///   m/49'/0'/account'/0/index
    /// # Returns
    ///   (address, private_key): (p2shwpkh, wif)
    fn bip49_wallet(&self, account: u32, index: u32, change: bool) -> DeriveResult {
        let change = if change { 1 } else { 0 };
        self.derive(&format!("m/49'/{COIN}'/{account}'/{change}/{index}"))
            .map(|(xpub, xpriv)| {
                let address =
                    Address::p2shwpkh(&CompressedPublicKey(xpub.public_key), crate::NETWORK);
                (address.to_string(), xpriv.to_priv().to_wif())
            })
    }

    /// Derive a wallet from BIP49 account
    /// # Derivation path
    ///   m/49'/0'/account'/0/index'
    /// # Returns
    ///   (address, private_key): (p2shwpkh, wif)
    #[deprecated]
    fn bip49_harden(&self, account: u32, index: u32) -> DeriveResult {
        self.derive(&format!("m/49'/{COIN}'/{account}'/0/{index}'"))
            .map(|(xpub, xpriv)| {
                let address =
                    Address::p2shwpkh(&CompressedPublicKey(xpub.public_key), crate::NETWORK);
                (address.to_string(), xpriv.to_priv().to_wif())
            })
    }

    /// Derive a multisig wallet from BIP49 accounts
    /// # Parameters
    /// - `M`: Number of required signatures
    /// - `N`: Number of total signatures
    /// - `account`: Account start index (total use N accounts)
    /// - `index`: Wallet index
    /// # Derivation path
    ///   m/49'/0'/account'/0/index
    ///   m/49'/0'/(account + 1)'/0/index
    ///   m/49'/0'/(account + 2)'/0/index
    ///   ...
    /// # Returns
    ///   (address, redeem_script)
    fn bip49_multisig<const M: u8, const N: u8>(&self, account: u32, index: u32) -> DeriveResult {
        assert!(M <= N && N <= 15, "[artimonist] Overflow: M <= N <= 15");
        let paths = (account..account + N as u32)
            .map(|account: u32| format!("m/49'/{COIN}'/{account}'/0/{index}"))
            .collect::<Vec<_>>();
        let script = self.multisig::<M>(paths.as_slice())?;
        Ok((
            Address::p2sh(&script, crate::NETWORK)?.to_string(),
            script.to_hex_string(),
        ))
    }
}

/// BIP84 derivation
pub trait Bip84
where
    Self: DeriveInner,
{
    /// Derive a BIP84 account
    /// # Derivation path
    ///   m/84'/0'/account'
    /// # Returns
    ///   (xpub, xpriv)
    fn bip84_account(&self, account: u32) -> DeriveResult {
        self.derive(&format!("m/84'/{COIN}'/{account}'"))
            .map(|(xpub, xpriv)| (xpub.to_zpub(), xpriv.to_zpriv()))
    }

    /// Derive a wallet from BIP84 account
    /// # Derivation path
    ///   m/84'/0'/account'/0/index
    /// # Returns
    ///   (address, private_key): (p2wpkh, wif)
    fn bip84_wallet(&self, account: u32, index: u32, change: bool) -> DeriveResult {
        let change = if change { 1 } else { 0 };
        let network = match crate::NETWORK.is_mainnet() {
            true => bitcoin::Network::Bitcoin,
            false => bitcoin::Network::Testnet,
        };
        self.derive(&format!("m/84'/{COIN}'/{account}'/{change}/{index}"))
            .map(|(xpub, xpriv)| {
                let address = Address::p2wpkh(&CompressedPublicKey(xpub.public_key), network);
                (address.to_string(), xpriv.to_priv().to_wif())
            })
    }

    /// Derive a wallet from BIP84 account
    /// # Derivation path
    ///   m/84'/0'/account'/0/index'
    /// # Returns
    ///   (address, private_key): (p2wpkh, wif)
    #[deprecated]
    fn bip84_harden(&self, account: u32, index: u32) -> DeriveResult {
        let network = match crate::NETWORK.is_mainnet() {
            true => bitcoin::Network::Bitcoin,
            false => bitcoin::Network::Testnet,
        };
        self.derive(&format!("m/84'/{COIN}'/{account}'/0/{index}'"))
            .map(|(xpub, xpriv)| {
                let address = Address::p2wpkh(&CompressedPublicKey(xpub.public_key), network);
                (address.to_string(), xpriv.to_priv().to_wif())
            })
    }

    /// Derive a multisig wallet from BIP84 accounts
    /// # Parameters
    /// - `M`: Number of required signatures
    /// - `N`: Number of total signatures
    /// - `account`: Account start index (total use N accounts)
    /// - `index`: Wallet index
    /// # Derivation path
    ///   m/84'/0'/account'/0/index
    ///   m/84'/0'/(account + 1)'/0/index
    ///   m/84'/0'/(account + 2)'/0/index
    ///   ...
    /// # Returns
    ///   (address, redeem_script)
    fn bip84_multisig<const M: u8, const N: u8>(&self, account: u32, index: u32) -> DeriveResult {
        assert!(M <= N && N <= 15, "[artimonist] Overflow: M <= N <= 15");
        let paths = (account..account + N as u32)
            .map(|account: u32| format!("m/84'/{COIN}'/{account}'/0/{index}"))
            .collect::<Vec<_>>();
        let script = self.multisig::<M>(paths.as_slice())?;
        Ok((
            Address::p2sh(&script, crate::NETWORK)?.to_string(),
            script.to_hex_string(),
        ))
    }
}

impl Bip32 for Xpriv {}
impl Bip44 for Xpriv {}
impl Bip49 for Xpriv {}
impl Bip84 for Xpriv {}

const BIP49_VERSION_BYTES_MAINNET_PRIVATE: u32 = 0x049d7878; // ypriv
const BIP49_VERSION_BYTES_MAINNET_PUBLIC: u32 = 0x049d7cb2; // ypub
const BIP49_VERSION_BYTES_TESTNET_PRIVATE: u32 = 0x044a4e28; // upriv
const BIP49_VERSION_BYTES_TESTNET_PUBLIC: u32 = 0x044a5262; // upub

const BIP84_VERSION_BYTES_MAINNET_PRIVATE: u32 = 0x04b2430c; // zpriv
const BIP84_VERSION_BYTES_MAINNET_PUBLIC: u32 = 0x04b24746; // zpub
const BIP84_VERSION_BYTES_TESTNET_PRIVATE: u32 = 0x045f18bc; // vpriv
const BIP84_VERSION_BYTES_TESTNET_PUBLIC: u32 = 0x045f1cf6; // vpub

trait XprivEncode {
    fn ext_encode<const PRE: u32>(&self) -> String;
    fn to_ypriv(&self) -> String;
    fn to_zpriv(&self) -> String;
}
impl XprivEncode for Xpriv {
    #[inline]
    fn ext_encode<const PRE: u32>(&self) -> String {
        let data = [&PRE.to_be_bytes(), &self.encode()[4..]].concat();
        bitcoin::base58::encode_check(&data[..])
    }
    #[inline(always)]
    fn to_ypriv(&self) -> String {
        if cfg!(feature = "extfmt") {
            match crate::NETWORK.is_mainnet() {
                true => self.ext_encode::<BIP49_VERSION_BYTES_MAINNET_PRIVATE>(),
                false => self.ext_encode::<BIP49_VERSION_BYTES_TESTNET_PRIVATE>(),
            }
        } else {
            self.to_string()
        }
    }
    #[inline(always)]
    fn to_zpriv(&self) -> String {
        if cfg!(feature = "extfmt") {
            match crate::NETWORK.is_mainnet() {
                true => self.ext_encode::<BIP84_VERSION_BYTES_MAINNET_PRIVATE>(),
                false => self.ext_encode::<BIP84_VERSION_BYTES_TESTNET_PRIVATE>(),
            }
        } else {
            self.to_string()
        }
    }
}

trait XpubEncode {
    fn ext_encode<const PRE: u32>(&self) -> String;
    fn to_ypub(&self) -> String;
    fn to_zpub(&self) -> String;
}

impl XpubEncode for Xpub {
    #[inline]
    fn ext_encode<const PRE: u32>(&self) -> String {
        let data = [&PRE.to_be_bytes(), &self.encode()[4..]].concat();
        bitcoin::base58::encode_check(&data[..])
    }
    #[inline(always)]
    fn to_ypub(&self) -> String {
        if cfg!(feature = "extfmt") {
            match crate::NETWORK.is_mainnet() {
                true => self.ext_encode::<BIP49_VERSION_BYTES_MAINNET_PUBLIC>(),
                false => self.ext_encode::<BIP49_VERSION_BYTES_TESTNET_PUBLIC>(),
            }
        } else {
            self.to_string()
        }
    }
    #[inline(always)]
    fn to_zpub(&self) -> String {
        if cfg!(feature = "extfmt") {
            match crate::NETWORK.is_mainnet() {
                true => self.ext_encode::<BIP84_VERSION_BYTES_MAINNET_PUBLIC>(),
                false => self.ext_encode::<BIP84_VERSION_BYTES_TESTNET_PUBLIC>(),
            }
        } else {
            self.to_string()
        }
    }
}
