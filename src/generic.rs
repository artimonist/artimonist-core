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
        let mut items = Vec::new();
        let mut indices = Vec::with_capacity(H * W);

        (0..W).rev().for_each(|col| {
            (0..H).rev().for_each(|row| {
                if let Some(v) = &self[row][col] {
                    items.push(v);
                    indices.push(true);
                } else {
                    indices.push(false);
                }
            });
        });

        let indices = indices.into_iter().to_bits();
        let data = rmp_serde::to_vec(&(indices, items))?;
        Ok(data)
    }
}

/// transform vector to generic diagram
pub trait VecDiagram<T: Serialize> {
    /// transform vector to matrix
    fn to_matrix<const H: usize, const W: usize>(self) -> Matrix<H, W, T>;
}

impl<T: Serialize> VecDiagram<T> for Vec<Option<T>> {
    fn to_matrix<const H: usize, const W: usize>(mut self) -> Matrix<H, W, T> {
        self.resize_with(H * W, || None);
        self.reverse();
        core::array::from_fn(|_| core::array::from_fn(|_| self.pop().unwrap()))
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
        const ENTROPY: &str = "726ea65196a104c64f845d93792fba3a149aaa8b1af323e3d474394bf7c204b2";
        const XPRIV: &str = "xprv9s21ZrQH143K26wqw5cyn4qGD2CsyVH2Lpma622cgETpFvNfnPAGpmkFisKjr3G3SUKoCXXkctNssYpAXuVeZBw2HmihXxnwYUxicZM2Spt";
        {
            // MATRIX easy to use
            let entropy = MATRIX.to_entropy("test".as_bytes())?;
            let master = MATRIX.to_master("test".as_bytes())?;
            assert_eq!(entropy.to_lower_hex_string(), ENTROPY);
            assert_eq!(master.to_string(), XPRIV);
        }
        {
            // VECTOR equal to MATRIX
            let vector = VECTOR.to_vec();
            let master = vector.to_matrix::<3, 5>().to_master("test".as_bytes())?;
            assert_eq!(master.to_string(), XPRIV);
        }
        {
            // verify vector to matrix sequence
            const MATRIX: [[Option<u8>; 3]; 3] = [
                [Some(1), Some(2), Some(3)],
                [Some(4), Some(5), Some(6)],
                [Some(7), Some(8), Some(9)],
            ];
            let vector: Vec<Option<u8>> =
                [1, 2, 3, 4, 5, 6, 7, 8, 9].into_iter().map(Some).collect();
            let matrix = vector.to_matrix::<3, 3>();
            assert_eq!(matrix, MATRIX);
        }
        Ok(())
    }
}

/// Simple Diagram
pub type GenericSimple = Matrix<7, 7, char>;

/// Complex Diagram
pub type GenericComplex = Matrix<7, 7, String>;
