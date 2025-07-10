use super::Mnemonic;
use crate::bip39::Bip39Error;
use bitcoin::bip32::Xpriv;
use std::str::FromStr;

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
/// let xprv = Xpriv::from_mnemonic("lake album jump occur hedgehog fantasy drama sauce oyster velvet gadget control behave hamster begin", "🌱")?;
/// # #[cfg(not(feature = "testnet"))]  
/// assert_eq!(xprv.to_string(), "xprv9s21ZrQH143K36NWXJp6dEYdnu27DM1GdQB7jxTtmXZDk4Bs65ZuHTV92tN5Dp42VPEnkAMknGM2FbStkEFUmH8g7AbPVi7jZNQgKMrAZYJ");
///
/// # Ok::<(), artimonist::Error>(())
/// ```
// # Reference
// [1] - [BIP39 spec](https://bips.dev/39/)
// [2] - [Ref website](https://iancoleman.io/bip39/)
//
pub trait Derivation {
    /// # Parameters
    ///   mnemonic: mnemonic str.
    fn from_mnemonic(mnemonic: &str, salt: &str) -> Result<Xpriv, Bip39Error> {
        let mnemonic = Mnemonic::from_str(mnemonic)?.to_string();
        let seed = {
            use pbkdf2::pbkdf2_hmac;
            let salt = format!("mnemonic{salt}").into_bytes();
            let mut output: [u8; 64] = [0; 64];
            pbkdf2_hmac::<sha2::Sha512>(mnemonic.as_bytes(), &salt, u32::pow(2, 11), &mut output);
            output
        };
        let xpriv = Xpriv::new_master(crate::NETWORK, &seed)?;
        Ok(xpriv)
    }
}

impl Derivation for Xpriv {}

#[cfg(test)]
mod bip39_test_english {
    use super::*;
    #[test]
    fn test_bip39() -> Result<(), Bip39Error> {
        #[cfg(not(feature = "testnet"))]
        const TEST_DATA: &[[&str; 3]] = &[
          ["theme rain hollow final expire proud detect wife hotel taxi witness strategy park head forest", "🍔🍟🌭🍕",
          "xprv9s21ZrQH143K2k5PPw697AeKWWdeQueM2JCKu8bsmF7M7dDmPGHecHJJNGeujWTJ97Fy9PfobsgZfxhcpWaYyAauFMxcy4fo3x7JNnbYQyD"],
        ];
        #[cfg(feature = "testnet")]
        const TEST_DATA: &[[&str; 3]] = &[
          ["theme rain hollow final expire proud detect wife hotel taxi witness strategy park head forest", "🍔🍟🌭🍕",
          "tprv8ZgxMBicQKsPdZJv4VweGpGJpe3reRgMMr7SmZ2LFDbpuDxrNddQ82fkHSpZjsqcWYnk9VHZmEGN8pFMwivVnDrVn1AvdRPqy3ripW55kfq"]
        ];
        for x in TEST_DATA {
            let xpriv = Xpriv::from_mnemonic(x[0], x[1])?;
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
    fn test_bip39() -> Result<(), Bip39Error> {
        const TEST_DATA: &[[&str; 3]] = &[
          ["solda osso frasco encontro donzela oficina colono vidraria fruteira sinal visto sacola mirtilo flamingo ereto", "",
            "xprv9s21ZrQH143K2KFS6iHoFXZC9Y5AWVKwxZis4GMRkQeaTFHiNRTkrjCsnBZ46s7VNihoMapH64FE93ZbzZ28Ld2oiHh6FYQx4eA8jEisYsc"],
          ["岗 跨 困 倒 考 邦 调 晒 慢 孟 畅 埋 黎 句 皮", "黎句皮",
            "xprv9s21ZrQH143K2SwhdXXWCKa3Sj3mw6123eUe4osWEbHavCv7FDqgFChzfedPDmgnHm9qnQrdveb8sVrywNxxBYCXTdaeNyxRRmhF4q33ovb"],
          ["클럽 작가 소설 부족 별도 일정 모금 확장 소형 콤플렉스 회복 촛불 위성 성별 비바람", "😎",
            "xprv9s21ZrQH143K43d7XRnapkCsoE2bLUJfA57hYseNpDaJxf5rpuhHgHjSXNMGMpaGYNNZfxxBzv1e2kW5CSy7p1rddfWYXtvYhgC6MPfHd9Z"],
          ["theme rain hollow final expire proud detect wife hotel taxi witness strategy park head forest", "🍔🍟🌭🍕",
            "xprv9s21ZrQH143K2k5PPw697AeKWWdeQueM2JCKu8bsmF7M7dDmPGHecHJJNGeujWTJ97Fy9PfobsgZfxhcpWaYyAauFMxcy4fo3x7JNnbYQyD"],
        ];
        for x in TEST_DATA {
            let xpriv = Xpriv::from_mnemonic(x[0], x[1])?;
            assert_eq!(xpriv.to_string(), x[2]);
        }

        const INVALID_CHECKSUM: &[&str] = &[
          "solda osso frasco encontro donzela oficina colono vidraria fruteira sinal visto sacola mirtilo flamingo final",
          "theme rain hollow sinal expire proud detect wife hotel taxi witness strategy park head forest",
          "岗 跨 困 倒 考 邦 调 晒 慢 孟 畅 句 埋 黎 皮"
        ];
        for x in INVALID_CHECKSUM {
            let r = Xpriv::from_mnemonic(*x, Default::default());
            assert!(matches!(r, Err(Bip39Error::InvalidChecksum)));
        }

        const INVALID_LENGTH: &[&str] = &[
            " 跨 困 倒 考 邦 调 晒 慢 孟 畅 句 埋 黎 皮",
            "theme rain hollow sinal expire proud detect wife hotel taxi witness",
        ];
        for x in INVALID_LENGTH {
            let r = Xpriv::from_mnemonic(*x, Default::default());
            assert!(matches!(r, Err(Bip39Error::InvalidLength)));
        }
        Ok(())
    }
}
