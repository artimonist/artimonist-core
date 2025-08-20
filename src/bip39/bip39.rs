use super::{Bip39Error, Mnemonic};
use bitcoin::bip32::Xpriv;
use std::str::FromStr;

type Result<T> = std::result::Result<T, Bip39Error>;

/// BIP39 Derivation for Xpriv
///
/// Create bip32 master key from mnemonic words list.
///
/// see: [BIP39 spec](https://bips.dev/39/)
///
/// # Examples
/// ```
/// use artimonist::{BIP39, Xpriv};
///
/// let mnemonic = "lake album jump occur hedgehog fantasy drama sauce oyster velvet gadget control behave hamster begin";
/// let xprv = mnemonic.mnemonic_to_master("🌱")?;
/// # #[cfg(not(feature = "testnet"))]
/// assert_eq!(xprv.to_string(), "xprv9s21ZrQH143K36NWXJp6dEYdnu27DM1GdQB7jxTtmXZDk4Bs65ZuHTV92tN5Dp42VPEnkAMknGM2FbStkEFUmH8g7AbPVi7jZNQgKMrAZYJ");
///
/// # Ok::<(), artimonist::Error>(())
/// ```
/// # Reference
/// [1] - [BIP39 spec](https://bips.dev/39/)
/// [2] - [Ref website](https://iancoleman.io/bip39/)
pub trait Bip39 {
    /// Convert a mnemonic phrase to a master key.
    fn mnemonic_to_master(&self, salt: &str) -> Result<Xpriv>;
}

impl<T> Bip39 for T
where
    T: AsRef<str>,
{
    #[inline]
    fn mnemonic_to_master(&self, salt: &str) -> Result<Xpriv> {
        Mnemonic::from_str(self.as_ref())?.to_master(salt)
    }
}

#[cfg(test)]
mod bip39_test_english {
    use super::*;
    #[test]
    fn test_bip39() -> Result<()> {
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
        for x in TEST_DATA {
            let xpriv = x[0].mnemonic_to_master(x[1])?;
            assert_eq!(xpriv.to_string(), x[2]);
        }
        Ok(())
    }
}

#[cfg(not(feature = "testnet"))]
#[cfg(test)]
mod bip39_test_multilingual {
    use super::*;

    /// # Reference
    ///     <https://iancoleman.io/bip39>
    #[test]
    fn test_bip39() -> Result<()> {
        const TEST_DATA: &[[&str; 3]] = &[
            [
                "solda osso frasco encontro donzela oficina colono vidraria fruteira sinal visto sacola mirtilo flamingo ereto",
                "",
                "xprv9s21ZrQH143K2KFS6iHoFXZC9Y5AWVKwxZis4GMRkQeaTFHiNRTkrjCsnBZ46s7VNihoMapH64FE93ZbzZ28Ld2oiHh6FYQx4eA8jEisYsc",
            ],
            [
                "岗 跨 困 倒 考 邦 调 晒 慢 孟 畅 埋 黎 句 皮",
                "黎句皮",
                "xprv9s21ZrQH143K2SwhdXXWCKa3Sj3mw6123eUe4osWEbHavCv7FDqgFChzfedPDmgnHm9qnQrdveb8sVrywNxxBYCXTdaeNyxRRmhF4q33ovb",
            ],
            [
                "클럽 작가 소설 부족 별도 일정 모금 확장 소형 콤플렉스 회복 촛불 위성 성별 비바람",
                "😎",
                "xprv9s21ZrQH143K43d7XRnapkCsoE2bLUJfA57hYseNpDaJxf5rpuhHgHjSXNMGMpaGYNNZfxxBzv1e2kW5CSy7p1rddfWYXtvYhgC6MPfHd9Z",
            ],
            [
                "theme rain hollow final expire proud detect wife hotel taxi witness strategy park head forest",
                "🍔🍟🌭🍕",
                "xprv9s21ZrQH143K2k5PPw697AeKWWdeQueM2JCKu8bsmF7M7dDmPGHecHJJNGeujWTJ97Fy9PfobsgZfxhcpWaYyAauFMxcy4fo3x7JNnbYQyD",
            ],
        ];
        for x in TEST_DATA {
            let xpriv = x[0].mnemonic_to_master(x[1])?;
            assert_eq!(xpriv.to_string(), x[2]);
        }

        const INVALID_CHECKSUM: &[&str] = &[
            "solda osso frasco encontro donzela oficina colono vidraria fruteira sinal visto sacola mirtilo flamingo final",
            "theme rain hollow sinal expire proud detect wife hotel taxi witness strategy park head forest",
            "岗 跨 困 倒 考 邦 调 晒 慢 孟 畅 句 埋 黎 皮",
        ];
        for x in INVALID_CHECKSUM {
            let r = x.mnemonic_to_master("");
            assert!(matches!(r, Err(Bip39Error::InvalidChecksum)));
        }

        const INVALID_LENGTH: &[&str] = &[
            " 跨 困 倒 考 邦 调 晒 慢 孟 畅 句 埋 黎 皮",
            "theme rain hollow sinal expire proud detect wife hotel taxi witness",
        ];
        for x in INVALID_LENGTH {
            let r = x.mnemonic_to_master("");
            assert!(matches!(r, Err(Bip39Error::InvalidSize)));
        }
        Ok(())
    }
}
