/*!
 * # Reference
 *
 * [1] - Unicode characters
 *      <https://www.unicodepedia.com/>
 *
 * # Descriptions
 *
 * [1] - Simple Diagram secret data construction
 *      |--utf8 chars---|-7 bytes-|-1 byte-|
 *      |Char1|Char2|...| Indices |CheckSum|
 *      |---------------|---------|--------|
 *      n = indices.count_ones()  (version == 0)
 *
 * [2] - Simple Diagram indices data construction
 *      0b0xxx_xxxx
 *      0b0xxx_xxxx
 *      0b0xxx_xxxx
 *      0b0xxx_xxxx
 *      0b0xxx_xxxx
 *      0b0xxx_xxxx
 *      0b0xxx_xxxx
 *      x bits indices string position in diagram.
**/
use super::generic::{GenericDiagram, GenericResult};
use super::macros::ImpDeref;
use bitcoin::hashes::{sha256, Hash};
use serde::{Deserialize, Serialize};

/// Simple Diagram
///
/// `Simple Diagram' contains arbitrary characters in 7 * 7 grid cells.
/// All Unicode characters are supported.
///
/// # Examples
/// ```
/// # use artimonist::{GenericDiagram, SimpleDiagram};
/// # use bitcoin::hex::DisplayHex;
/// let mut diagram = SimpleDiagram::new();
/// diagram[2][1] = Some('🐶');
/// diagram[3][6] = Some('☕');
///
/// let entropy = diagram.warp_entropy("🎄🎈🔑".as_bytes())?;
/// assert_eq!(entropy.to_lower_hex_string(), "3f07bac0334f6c1733e590f6421d8dbd773e686b8d55eff462c007aa017365d3");
/// # Ok::<(), artimonist::Error>(())
/// ```
///
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimpleDiagram(pub [[Option<char>; 7]; 7]);
ImpDeref!(SimpleDiagram, [[Option<char>; 7]; 7]);

impl GenericDiagram<7, 7> for SimpleDiagram {
    type Item = char;

    /// Compatible with previous versions
    fn to_bytes(&self) -> GenericResult<Vec<u8>> {
        let mut chars = Vec::with_capacity(7 * 7);
        let mut indices = [0; 7];
        (0..7).rev().for_each(|col| {
            (0..7).rev().for_each(|row| {
                if let Some(ch) = self[row][col] {
                    chars.push(ch);
                    indices[row] |= INDICES_MASK[col];
                }
            });
        });

        let str = chars.into_iter().collect::<String>();
        let mut secret = [str.as_bytes(), &indices].concat();
        let check = sha256::Hash::hash(&secret).as_byte_array()[0];
        secret.push(check);

        Ok(secret)
    }
}

const INDICES_MASK: [u8; 7] = [
    0b0100_0000,
    0b0010_0000,
    0b0001_0000,
    0b0000_1000,
    0b0000_0100,
    0b0000_0010,
    0b0000_0001,
];

impl SimpleDiagram {
    /// create simple diagram
    pub fn new() -> Self {
        Self([[None; 7]; 7])
    }

    /// create SimpleDiagram from items
    pub fn from_values(items: &[char], indices: &[(usize, usize)]) -> Self {
        let mut data = [[None; 7]; 7];
        indices
            .iter()
            .zip(items)
            .for_each(|(&(r, c), &v)| data[r][c] = Some(v));
        SimpleDiagram(data)
    }
}

#[cfg(test)]
mod simple_diagram_test {
    use super::*;
    use bitcoin::hex::DisplayHex;

    #[test]
    fn test_simple_diagram() -> GenericResult {
        const CHARS_STR: &str = "A&*王😊";
        const CHARS_INDICES: &[(usize, usize)] = &[(0, 6), (1, 1), (1, 3), (4, 2), (6, 6)];
        const SECRET_HEX: &str = "f09f988a412ae78e8b26012800001000012d";
        const WARP_ENTROPY: &str =
            "cff2b0d401d54f32d9035a2eed41f48f57960ac76fb472267ffd6597b3684d82";
        #[cfg(not(feature = "testnet"))]
        const MASTER_WIF: &str = "xprv9s21ZrQH143K2r6v9GGWezApYmVuaGiZYoCpsQFVe9Vwh47yZ2CCgqXJY6g2Kk8Ajrz2PbVNnY5HLw4dPkshmcqX8YBEhcwj4wWQ8UgY5m7";
        #[cfg(feature = "testnet")]
        const MASTER_WIF: &str = "tprv8ZgxMBicQKsPdfLSoq81pdnortv7onkZtM7wjpfx87zRUes4YPXxCatkTGqgL7WV7JWoPh78wtf5oncNWyDeag77fBPYMyfmz3FpaCED928";
        const SALT_STR: &str = "123abc";
        const SALT_ENTROPY: &str =
            "7981de9ab25fb45394130deca46b1ad9e18a84717be708cb39343e0700beba67";
        #[cfg(not(feature = "testnet"))]
        const SALT_MASTER: &str = "xprv9s21ZrQH143K3m9k6SE8k9kYgPUS2YiuWyV2LZN43xMPSWe8w1vriyFgPh4BnFGevHto27pmDCcnpJRAWLybqaaZeucx9fmJHFd2CWFMwkw";
        #[cfg(feature = "testnet")]
        const SALT_MASTER: &str = "tprv8ZgxMBicQKsPeaPGm15duoNXzWteG4kurXQ9CynWXvqsE7PDvPGcEid8JsDqnceyHjRa2DSXNZCbH9xudZKYedrABYqFp2VMCMNSeBkeo4Y";

        let items: Vec<char> = CHARS_STR.chars().collect();
        let sdm = SimpleDiagram::from_values(&items, CHARS_INDICES);
        assert_eq!(sdm.to_bytes()?.to_lower_hex_string(), SECRET_HEX);
        assert_eq!(sdm[6][6], Some('😊'));

        let entropy = sdm.warp_entropy(Default::default())?;
        assert_eq!(entropy.to_lower_hex_string(), WARP_ENTROPY);
        assert_eq!(sdm.bip32_master(&vec![])?.to_string(), MASTER_WIF);

        let entropy = sdm.warp_entropy(SALT_STR.as_bytes())?;
        assert_eq!(entropy.to_lower_hex_string(), SALT_ENTROPY);
        let master = sdm.bip32_master(SALT_STR.as_bytes())?;
        assert_eq!(master.to_string(), SALT_MASTER);

        Ok(())
    }

    #[test]
    fn test_simple_diagram2() -> GenericResult<()> {
        const CHARS_STR: &str = "A王&*😊";
        const CHARS_INDICES: &[(usize, usize)] = &[(0, 6), (1, 1), (1, 3), (4, 2), (6, 0)];
        const SECRET_HEX: &str = "41262ae78e8bf09f988a012800001000406d";
        const WARP_ENTROPY: &str =
            "0948fd6d7b1dc397d26080804870913abc086636d3ed11d4fcb0f16f7c31a91a";
        const SALT_STR: &str = "123abc";
        const SALT_ENTROPY: &str =
            "e06ffd848c7901ca5757d848e5e81d69f9853273bee6772dcd25f56c506a1635";

        let items: Vec<char> = CHARS_STR.chars().collect();
        let sdm = SimpleDiagram::from_values(&items, CHARS_INDICES);
        assert_eq!(sdm.to_bytes()?.to_lower_hex_string(), SECRET_HEX);
        assert_eq!(sdm[6][0], Some('😊'));

        let entropy = sdm.warp_entropy(Default::default())?;
        assert_eq!(entropy.to_lower_hex_string(), WARP_ENTROPY);
        let entropy = sdm.warp_entropy(SALT_STR.as_bytes())?;
        assert_eq!(entropy.to_lower_hex_string(), SALT_ENTROPY);

        Ok(())
    }
}
