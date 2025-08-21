use super::Result;
use bitcoin::bip32::Xpriv;

const DEFAULT_SALT: &[u8] = b"Thanks Satoshi!";

/// Generic Diagram  
///   diagram implementation for any matrix
pub trait GenericDiagram {
    /// cell item type
    type Item;

    /// serialize diagram to binary data
    fn to_bytes(&self) -> Result<Vec<u8>>;

    /// generate warp entropy
    ///
    /// see:
    /// [warp wallet](https://keybase.io/warp),
    /// [go impl](https://github.com/ellisonch/warpwallet)
    fn to_entropy(&self, salt: &[u8]) -> Result<[u8; 32]> {
        let secret = self.to_bytes()?;
        let mut s1 = {
            let secret = [&secret[..], &[1u8]].concat();
            let salt = [DEFAULT_SALT, salt, &[1u8]].concat();
            let mut output: [u8; 32] = [0; 32];
            let param = scrypt::Params::new(20, 8, 1, 32)?;
            scrypt::scrypt(&secret, &salt, &param, &mut output)?;
            output
        };
        let s2 = {
            let secret = [&secret[..], &[2u8]].concat();
            let salt = [DEFAULT_SALT, salt, &[2u8]].concat();
            let mut output: [u8; 32] = [0; 32];
            argon2::Argon2::default().hash_password_into(&secret, &salt, &mut output)?;
            output
        };
        s1.iter_mut().zip(s2.iter()).for_each(|(a, b)| *a ^= b);
        Ok(s1)
    }

    /// generate extended private key
    fn to_master(&self, salt: &[u8]) -> Result<Xpriv> {
        let seed = self.to_entropy(salt)?;
        Ok(Xpriv::new_master(crate::NETWORK, &seed)?)
    }

    /// generate extended private key
    fn to_master_v1(&self, salt: &[u8]) -> Result<Xpriv> {
        let secret = self.to_bytes()?;
        let mut s1 = {
            let secret = [&secret[..], &[1u8]].concat();
            let salt = [salt, &[1u8]].concat();
            let mut output: [u8; 32] = [0; 32];
            let param = scrypt::Params::new(18, 8, 1, 32)?;
            scrypt::scrypt(&secret, &salt, &param, &mut output)?;
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

        let seed = s1;
        Ok(Xpriv::new_master(crate::NETWORK, &seed)?)
    }
}

#[cfg(feature = "serde")]
impl<T: serde::Serialize, const H: usize, const W: usize> GenericDiagram for Matrix<T, H, W> {
    type Item = T;

    fn to_bytes(&self) -> Result<Vec<u8>> {
        use xbits::FromBits;

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

        let indices = Vec::from_bits(indices.into_iter());
        let data = rmp_serde::to_vec(&(indices, items))?;
        Ok(data)
    }
}

/// Matrix type for generic diagram
#[cfg(feature = "serde")]
pub type Matrix<T, const H: usize = 7, const W: usize = 7> = [[Option<T>; W]; H];

/// Transform to generic diagram
#[cfg(feature = "serde")]
pub trait ToMatrix<T> {
    /// transform to matrix
    fn to_matrix<const H: usize, const W: usize>(self) -> Matrix<T, H, W>;
}

#[cfg(feature = "serde")]
impl<T> ToMatrix<T> for Vec<Option<T>> {
    fn to_matrix<const H: usize, const W: usize>(mut self) -> Matrix<T, H, W> {
        self.resize_with(H * W, || None);
        self.reverse();
        core::array::from_fn(|_| core::array::from_fn(|_| self.pop().unwrap()))
    }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod generic_test {
    use super::*;
    use bitcoin::hex::DisplayHex;

    #[test]
    fn test_generic() -> Result {
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
        const ENTROPY: &str = "832ad08e52ac5ef61ca43fb03741d8a8126f57b71f308c48cfbd2ab79095b11f";
        #[cfg(not(feature = "testnet"))]
        mod wif {
            pub const MASTER_V1: &str = "xprv9s21ZrQH143K26wqw5cyn4qGD2CsyVH2Lpma622cgETpFvNfnPAGpmkFisKjr3G3SUKoCXXkctNssYpAXuVeZBw2HmihXxnwYUxicZM2Spt";
            pub const MASTER: &str = "xprv9s21ZrQH143K2dxnQzW6fR8yUaoUiRtXH9hjS1j78gazBwCDQHSLKSDw3QvvE3HfY9zAM4qJ6cLQhghuRm71rkM1XZhvVRNkHsq5ftcfiLL";
        }
        #[cfg(feature = "testnet")]
        mod wif {
            pub const MASTER_V1: &str = "tprv8ZgxMBicQKsPcvBNbeUUwiTFX9d6D1K2gNggxST5ACxJ3X7kmkW2LX7he3VPrQeMouraCd9WnExgLQMuf7qbNFCcpQw1CKWzTai94JYzs9K";
            pub const MASTER: &str = "tprv8ZgxMBicQKsPdTCK5ZMbq4kxniDgwwvXchcrJS9Zcf5TyXwJPen5qBbNxb6aEQfyubWwMAT4FxvDAYFeYySxfocc4CvE9n6oCyaW7g2dWq8";
        }
        {
            // MATRIX easy to use
            let entropy = MATRIX.to_entropy("test".as_bytes())?;
            assert_eq!(entropy.to_lower_hex_string(), ENTROPY);
            let master = MATRIX.to_master("test".as_bytes())?;
            assert_eq!(master.to_string(), wif::MASTER);

            let master_v1 = MATRIX.to_master_v1("test".as_bytes())?;
            assert_eq!(master_v1.to_string(), wif::MASTER_V1);
        }
        {
            // VECTOR equal to MATRIX
            let matrix: Matrix<u128, 3, 5> = VECTOR.to_vec().to_matrix::<3, 5>();
            let master = matrix.to_master("test".as_bytes())?;
            assert_eq!(master.to_string(), wif::MASTER);

            let master_v1 = matrix.to_master_v1("test".as_bytes())?;
            assert_eq!(master_v1.to_string(), wif::MASTER_V1);
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

    #[test]
    fn test_serialize() -> Result {
        const MATRIX: [[Option<u8>; 5]; 3] = [
            [Some(123), None, None, None, Some(99)],
            [Some(222), None, None, None, Some(0)],
            [None, None, Some(1), None, None],
        ];
        let buf = rmp_serde::to_vec(&MATRIX)?;
        let mx: Matrix<u8, 3, 5> = rmp_serde::from_slice(&buf)?;
        assert_eq!(mx, MATRIX);
        Ok(())
    }
}
