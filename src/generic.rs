use std::fmt::Debug;

use crate::bits::BitAggregation;
use bitcoin::{bip32::Xpriv, NetworkKind};
use serde::Serialize;
use thiserror::Error;

/// Generic Diagram
///   diagram implementation for any matrix
///
/// # Parameters
///   H: matrix height
///   W: matrix weight
///   T: matrix item
///
/// # Examples
/// ```
/// ```
pub trait GenericDiagram<const H: usize, const W: usize, T: Serialize> {
    /// serialize diagram
    fn to_secret(&self) -> GenericResult<Vec<u8>>;

    /// generate warp entropy
    ///
    /// see:
    /// [warp wallet](https://keybase.io/warp),
    /// [go impl](https://github.com/ellisonch/warpwallet)
    ///
    fn to_entropy(&self, salt: &[u8]) -> GenericResult<[u8; 32]> {
        let secret = self.to_secret()?;
        let mut s1 = {
            let secret = [&secret[..], &[1u8]].concat();
            let salt = [salt, &[1u8]].concat();
            let mut output: [u8; 32] = [0; 32];
            let param = scrypt::Params::new(18, 8, 1, 32).unwrap();
            scrypt::scrypt(&secret, &salt, &param, &mut output).unwrap();
            output
        };
        let s2 = {
            let secret = [&secret[..], &[2u8]].concat();
            let salt = [salt, &[2u8]].concat();
            let mut output: [u8; 32] = [0; 32];
            pbkdf2::pbkdf2_hmac::<sha2::Sha256>(&secret, &salt, 65536, &mut output);
            output
        };
        s1.iter_mut().zip(s2.iter()).for_each(|(a, b)| *a ^= b);
        Ok(s1)
    }

    /// generate extended private key
    fn to_master(&self, salt: &[u8]) -> GenericResult<Xpriv> {
        let seed = self.to_entropy(salt)?;
        Ok(Xpriv::new_master(NetworkKind::Main, &seed)?)
    }
}

type Matrix<const H: usize, const W: usize, T> = [[Option<T>; W]; H];

impl<const H: usize, const W: usize, T: Serialize> GenericDiagram<H, W, T> for Matrix<H, W, T> {
    fn to_secret(&self) -> GenericResult<Vec<u8>> {
        let indices = self
            .iter()
            .flat_map(|r| r.iter())
            .map(|v| v.is_some())
            .to_bits();
        let items: Vec<_> = self
            .into_iter()
            .flat_map(|r| r.into_iter())
            .flatten()
            .collect();
        let vs = rmp_serde::to_vec(&(indices, items))?;
        Ok(vs)
    }
}

/// transform vector to generic diagram
pub trait VecDiagram<const H: usize, const W: usize, T: Serialize> {
    /// transform vector to generic diagram
    fn to_diagram(self) -> Matrix<H, W, T>;
}

impl<const H: usize, const W: usize, T: Serialize + Copy> VecDiagram<H, W, T> for Vec<Option<T>> {
    fn to_diagram(mut self) -> Matrix<H, W, T> {
        // if not enough items, padding None values
        {
            let len = match self.len() < H * W {
                true => H * W - self.len(),
                false => 0,
            };
            self.append(&mut vec![None; len]);
        };
        core::array::from_fn(|row| core::array::from_fn(|col| self[row * W + col]))
    }
}

/// GenericError
#[derive(Error, Debug)]
pub enum GenericError {
    /// serialize error
    #[error("serialize error")]
    SerializeError(#[from] rmp_serde::encode::Error),
    /// bip32 error
    #[error("bip32 error")]
    Bip32Error(#[from] bitcoin::bip32::Error),
}
/// GenericResult
type GenericResult<T = ()> = Result<T, GenericError>;

#[cfg(test)]
mod generic_test {
    use super::*;
    use bitcoin::hex::DisplayHex;

    #[test]
    fn test_generic() -> GenericResult {
        const MATRIX: [[Option<u128>; 5]; 3] = [
            [Some(111222333444555666), None, None, None, Some(0)],
            [Some(555666777888999000), None, None, None, Some(0)],
            [None, None, None, None, None],
        ];
        const VECTOR: [Option<u128>; 10] = [
            Some(111222333444555666),
            None,
            None,
            None,
            Some(0),
            Some(555666777888999000),
            None,
            None,
            None,
            Some(0),
        ];
        const ENTROPY: &str = "5cb583949c32a6bcfc8252e7b0a0ca5663ded466cb1ead68f539ba784f8f6da3";
        const XPRIV: &str = "xprv9s21ZrQH143K2HQAyrRqc9HvvxEBsehBm2V2XdxFLqrATdboEjatXbSmYtPSBxKRtrTAqQQ4NZ93rnjBoDvw5poWpH2Tfxm27M6imD8doLn";
        {
            let entropy = MATRIX.to_entropy("test".as_bytes())?;
            let master = MATRIX.to_master("test".as_bytes())?;
            assert_eq!(entropy.to_lower_hex_string(), ENTROPY);
            assert_eq!(master.to_string(), XPRIV);
        }
        {
            let matrix: Matrix<3, 5, u128> = VECTOR.to_vec().to_diagram();
            let entropy = matrix.to_entropy("test".as_bytes())?;
            let master = matrix.to_master("test".as_bytes())?;
            assert_eq!(entropy.to_lower_hex_string(), ENTROPY);
            assert_eq!(master.to_string(), XPRIV);
        }
        Ok(())
    }
}
