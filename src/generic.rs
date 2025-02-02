use std::fmt::Debug;

use crate::bits::{BitAggregation, BitOperation};
use bitcoin::{bip32::Xpriv, NetworkKind};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Generic Diagram
///   diagram implementation for any matrix
///
/// # Parameters
///   H: matrix height
///   W: matrix weight
///   T: matrix item
///   (items, indices)
///
/// # Examples
/// ```
/// ```
#[derive(Serialize, Deserialize)]
pub struct GenericDiagram<const H: usize, const W: usize, T: Serialize>(Vec<T>, Vec<u8>);

impl<const H: usize, const W: usize, T: Serialize> GenericDiagram<H, W, T> {
    /// generate warp entropy
    ///
    /// see:
    /// [warp wallet](https://keybase.io/warp),
    /// [go impl](https://github.com/ellisonch/warpwallet)
    ///
    pub fn to_entropy(&self, salt: &[u8]) -> GenericResult<[u8; 32]> {
        let secret = rmp_serde::to_vec(&self)?;
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

impl<const H: usize, const W: usize, T: Serialize> From<Matrix<H, W, T>>
    for GenericDiagram<H, W, T>
{
    fn from(matrix: Matrix<H, W, T>) -> Self {
        let indices = matrix
            .iter()
            .flat_map(|r| r.iter())
            .map(|v| v.is_some())
            .to_bits();
        let items: Vec<_> = matrix
            .into_iter()
            .flat_map(|r| r.into_iter())
            .flatten()
            .collect();
        Self(items, indices)
    }
}

impl<const H: usize, const W: usize, T: Serialize> From<GenericDiagram<H, W, T>>
    for Matrix<H, W, T>
{
    fn from(value: GenericDiagram<H, W, T>) -> Self {
        let mut indices = value.1.bit_iter().take(H * W);
        let mut items = value.0;
        items.reverse();
        core::array::from_fn(|_| {
            core::array::from_fn(|_| match indices.next() {
                Some(true) => items.pop(),
                _ => None,
            })
        })
    }
}

/// Vec<Option<T>> to GenericDiagram by size of H * W
impl<const H: usize, const W: usize, T: Serialize> From<Vec<Option<T>>>
    for GenericDiagram<H, W, T>
{
    fn from(mut value: Vec<Option<T>>) -> Self {
        // if not enough, append None values
        if value.len() < H * W {
            let mut tail = Vec::with_capacity(H * W - value.len());
            tail.fill_with(|| None);
            value.append(&mut tail);
        }
        let indices = value.iter().take(H * W).map(|v| v.is_some()).to_bits();
        let items = value.into_iter().take(H * W).flatten().collect();
        Self(items, indices)
    }
}

/// GenericDiagram to Vec<Option<T>> by size of H * W
impl<const H: usize, const W: usize, T: Serialize> From<GenericDiagram<H, W, T>>
    for Vec<Option<T>>
{
    fn from(value: GenericDiagram<H, W, T>) -> Self {
        let mut items = value.0;
        items.reverse();
        value
            .1
            .bit_iter()
            .take(H * W)
            .map(|b| match b {
                true => items.pop(),
                false => None,
            })
            .collect()
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
        const ENTROPY: &str = "4f782731ac9a96a4f76360a7f95c55d76c684015b1e6dfa2b07fbe4635e95f80";
        const XPRIV: &str = "xprv9s21ZrQH143K3UyPCjUsk2BqTEJpVpNQNHUXmf3sgQvM5rgzs4LPzNf1AN7R6yzo88gjz4eaEnkwt2M9rj32MX1tqw4mvtvgHUBeHgdeD4i";
        {
            let diagram: GenericDiagram<3, 5, u128> = MATRIX.into();
            let entropy = diagram.to_entropy("test".as_bytes())?;
            let master = diagram.to_master("test".as_bytes())?;
            assert_eq!(entropy.to_lower_hex_string(), ENTROPY);
            assert_eq!(master.to_string(), XPRIV);
            let matrix: [[Option<u128>; 5]; 3] = diagram.into();
            assert_eq!(matrix, MATRIX);
        }
        {
            let diagram: GenericDiagram<3, 5, u128> = VECTOR.to_vec().into();
            let entropy = diagram.to_entropy("test".as_bytes())?;
            let master = diagram.to_master("test".as_bytes())?;
            assert_eq!(entropy.to_lower_hex_string(), ENTROPY);
            assert_eq!(master.to_string(), XPRIV);
            let matrix: [[_; 5]; 3] = diagram.into();
            assert_eq!(matrix, MATRIX);
        }
        Ok(())
    }

    #[test]
    fn test_restore() -> GenericResult {
        const VECTOR: [u8; 18] = [1, 2, 3, 4, 5, 6, 1, 2, 3, 4, 5, 6, 1, 2, 3, 4, 5, 6];
        let diagram: GenericDiagram<3, 6, u8> =
            VECTOR.into_iter().map(Some).collect::<Vec<_>>().into();

        let entropy = diagram.to_entropy("test".as_bytes())?;
        let ser = rmp_serde::to_vec(&diagram)?;
        println!("{:?}", ser.to_lower_hex_string());
        let restore: GenericDiagram<3, 6, u8> = rmp_serde::from_slice(&ser).expect("restore");
        assert_eq!(restore.to_entropy("test".as_bytes())?, entropy);

        let matrix: [[Option<u8>; 6]; 3] = diagram.into();
        assert_eq!(
            matrix.into_iter().flatten().flatten().collect::<Vec<u8>>(),
            VECTOR
        );
        Ok(())
    }
}
