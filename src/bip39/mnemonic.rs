use super::{Bip39Error, Language};
use bitcoin::bip32::Xpriv;
use sha2::{Digest, Sha256};
use xbits::{FromBits, XBits};

type Result<T> = std::result::Result<T, Bip39Error>;

/// A BIP39 mnemonic phrase, which is a sequence of words
/// used to represent a seed for cryptographic purposes.
/// # Reference
/// [1] - [BIP39 spec](https://bips.dev/39/)
/// [2] - [Ref website](https://iancoleman.io/bip39/)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mnemonic {
    words: Vec<String>,
    language: Language,
}

#[allow(unused)]
impl Mnemonic {
    /// Validate mnemonic words count.
    pub const VALID_SIZES: &[usize] = &[12, 15, 18, 21, 24];

    /// Create a new mnemonic from raw entropy and language.
    /// # Arguments
    /// * `entropy` - A byte slice representing the entropy.  
    ///   The entropy length must be one of: 16, 20, 24, 28, or 32 bytes.
    ///   Mnemonic lengths will be 12, 15, 18, 21, or 24 words respectively.
    /// * `language` - The language of the mnemonic.
    /// # Returns
    /// * `Ok(Mnemonic)` - If the mnemonic is successfully created.
    pub fn new(entropy: &[u8], language: Language) -> Result<Self> {
        // verify length
        if !matches!(entropy.len(), 16 | 20 | 24 | 28 | 32) {
            return Err(Bip39Error::InvalidSize);
        }

        // calculate checksum
        let size = entropy.len() / 4 * 3;
        let check_mask = 0xff << (8 - size / 3);
        let checksum = Sha256::digest(entropy)[0] & check_mask;

        // convert entropy to indices
        let indices: Vec<usize> = [entropy.to_vec(), vec![checksum]]
            .concat()
            .bits()
            .chunks(11)
            .take(size)
            .collect();

        // convert indices to words
        let words = indices
            .iter()
            .map(|&i| language.word_at(i).unwrap_or_default().to_string())
            .collect();

        Ok(Mnemonic { words, language })
    }

    /// Generate a master key from the mnemonic phrase.
    pub fn to_master(&self, salt: &str) -> Result<Xpriv> {
        let mnemonic = self.to_string();
        let salt = format!("mnemonic{salt}");

        let mut seed: [u8; 64] = [0; 64];
        pbkdf2::pbkdf2_hmac::<sha2::Sha512>(
            mnemonic.as_bytes(),
            salt.as_bytes(),
            u32::pow(2, 11),
            &mut seed,
        );

        Ok(Xpriv::new_master(crate::NETWORK, &seed)?)
    }

    /// Mnemonic language
    #[inline]
    pub fn language(&self) -> Language {
        self.language
    }

    /// Mnemonic words count.
    #[inline]
    pub fn size(&self) -> usize {
        self.words.len()
    }

    /// Get an iterator over the words.
    #[inline]
    pub fn words(&self) -> impl Iterator<Item = &String> {
        self.words.iter()
    }

    /// Mnemonic words indices.
    #[inline]
    pub fn indices(&self) -> impl Iterator<Item = usize> {
        self.words
            .iter()
            .map(|w| self.language.index_of(w).unwrap())
    }

    /// Get mnemonic raw entropy
    #[inline]
    pub fn entropy(&self) -> Vec<u8> {
        let mut entropy: Vec<u8> = Vec::from_bits_chunk(self.indices(), 11);
        entropy.pop(); // remove checksum
        entropy
    }

    /// Detect the language of a mnemonic phrase based on its words.
    fn detect_language<T>(words: impl Iterator<Item = T>) -> Vec<Language>
    where
        T: AsRef<str>,
    {
        // words common languages
        words
            .map(|w| Language::detect(w.as_ref()))
            .reduce(|mut acc, v| {
                acc.retain(|x| v.contains(x));
                acc
            })
            .unwrap_or_default()
    }

    /// Verify the checksum of a mnemonic phrase based on its indices.
    fn verify_checksum(indices: &[usize]) -> Result<()> {
        // verify length
        if !Self::VALID_SIZES.contains(&indices.len()) {
            return Err(Bip39Error::InvalidSize);
        }

        let mut entropy = Vec::from_bits_chunk(indices.iter().copied(), 11);
        let tail = entropy.pop();
        let check_mask = 0xff << (8 - indices.len() / 3);
        let checksum = Sha256::digest(&entropy)[0] & check_mask;

        // verify checksum
        if Some(checksum) != tail {
            return Err(Bip39Error::InvalidChecksum);
        }
        Ok(())
    }
}

impl std::str::FromStr for Mnemonic {
    type Err = Bip39Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let words: Vec<&str> = s.split_whitespace().collect();

        // verify words count
        if !Self::VALID_SIZES.contains(&words.len()) {
            return Err(Bip39Error::InvalidSize);
        }

        // detect languages
        let mut languages = Mnemonic::detect_language(words.iter());

        // verify checksum
        languages.retain(|&language| {
            if let Ok(indices) = language.indices(words.iter()) {
                Mnemonic::verify_checksum(&indices).is_ok()
            } else {
                false
            }
        });

        // return mnemonic
        match languages.len() {
            0 => Err(Bip39Error::InvalidChecksum),
            1 => Ok(Mnemonic {
                words: words.into_iter().map(String::from).collect(),
                language: languages.pop().unwrap(),
            }),
            2.. => {
                use Language::*;
                if languages == [ChineseSimplified, ChineseTraditional]
                    || languages == [ChineseTraditional, ChineseSimplified]
                {
                    // chinese common words has same indices, choice any one.
                    Ok(Mnemonic {
                        words: words.into_iter().map(String::from).collect(),
                        language: ChineseSimplified,
                    })
                } else {
                    Err(Bip39Error::AmbiguousLanguages(languages))
                }
            }
        }
    }
}

impl std::fmt::Display for Mnemonic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.words.join(" "))
    }
}

trait Indices {
    fn indices<T>(&self, words: impl Iterator<Item = T>) -> Result<Vec<usize>>
    where
        T: AsRef<str>;
}
impl Indices for Language {
    fn indices<T>(&self, words: impl Iterator<Item = T>) -> Result<Vec<usize>>
    where
        T: AsRef<str>,
    {
        words
            .map(|w| self.index_of(w.as_ref()))
            .collect::<Option<Vec<_>>>()
            .ok_or(Bip39Error::InvalidLanguage)
    }
}

#[cfg(test)]
mod mnemonic_tests {
    use super::*;

    #[test]
    fn test_mnemonic_master() -> Result<()> {
        #[cfg(not(feature = "testnet"))]
        const TEST_DATA: &[[&str; 3]] = &[[
            "theme rain hollow final expire proud detect wife hotel taxi witness strategy park head forest",
            "🍔🍟🌭🍕",
            "xprv9s21ZrQH143K2k5PPw697AeKWWdeQueM2JCKu8bsmF7M7dDmPGHecHJJNGeujWTJ97Fy9PfobsgZfxhcpWaYyAauFMxcy4fo3x7JNnbYQyD",
        ]];
        #[cfg(feature = "testnet")]
        const TEST_DATA: &[[&str; 3]] = &[[
            "theme rain hollow final expire proud detect wife hotel taxi witness strategy park head forest",
            "🍔🍟🌭🍕",
            "tprv8ZgxMBicQKsPdZJv4VweGpGJpe3reRgMMr7SmZ2LFDbpuDxrNddQ82fkHSpZjsqcWYnk9VHZmEGN8pFMwivVnDrVn1AvdRPqy3ripW55kfq",
        ]];
        for data in TEST_DATA {
            let mnemonic: Mnemonic = data[0].parse()?;
            assert_eq!(mnemonic.to_master(data[1])?.to_string(), data[2]);
        }
        Ok(())
    }

    #[cfg(not(feature = "testnet"))]
    #[test]
    fn test_nfc_salt() -> Result<()> {
        use unicode_normalization::UnicodeNormalization;

        let mnemonic = "caution want scheme basic teach bulb shadow pioneer blue add expand guess";
        let salt1 = "música"; // no nfc
        let salt2 = "música"; // nfc
        assert_eq!(salt1.nfc().collect::<String>(), salt2);
        assert_ne!(salt1.chars().count(), salt2.chars().count());

        let master1 = mnemonic.parse::<Mnemonic>()?.to_master(salt1)?;
        let master2 = mnemonic.parse::<Mnemonic>()?.to_master(salt2)?;
        assert_ne!(master1, master2);

        use crate::BIP49;
        let wallet1 = master1.bip49_wallet(0, 0, false).unwrap();
        let wallet2 = master2.bip49_wallet(0, 0, false).unwrap();
        assert_ne!(wallet1, wallet2);

        println!("wallet1: {wallet1:?}");
        println!("wallet2: {wallet2:?}");
        let electrum_address = "3NgaBMn1fQ9wrAVAhhnVKaTVP5gFo2Wedn";
        assert_eq!(electrum_address, wallet1.0);
        Ok(())
    }
}
