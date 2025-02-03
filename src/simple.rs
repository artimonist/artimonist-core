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
use super::diagram::*;
use super::generic::{GenericDiagram, GenericResult, GenericSerialization};
use bitcoin::hashes::{sha256, Hash};

/// Simple Diagram
///
/// `Simple Diagram' contains arbitrary characters in 7 * 7 grid cells.
/// All Unicode characters are supported.
///
/// # Examples
/// ```
/// # use artimonist::{Diagram, SimpleDiagram, GenericDiagram};
/// # use bitcoin::hex::DisplayHex;
/// let mut diagram = SimpleDiagram([[None; 7]; 7]);
/// diagram[2][1] = Some('🐶');
/// diagram[3][6] = Some('☕');
///
/// let entropy = diagram.warp_entropy("🎄🎈🔑".as_bytes())?;
/// assert_eq!(entropy.to_lower_hex_string(), "3f07bac0334f6c1733e590f6421d8dbd773e686b8d55eff462c007aa017365d3");
/// # Ok::<(), artimonist::Error>(())
/// ```
///
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SimpleDiagram(pub [[Option<char>; 7]; 7]);

impl std::ops::Deref for SimpleDiagram {
    type Target = [[Option<char>; 7]; 7];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for SimpleDiagram {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl GenericDiagram<7, 7, char> for SimpleDiagram {}
impl GenericSerialization for SimpleDiagram {
    fn serialize(&self) -> GenericResult<Vec<u8>> {
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

use crate::diagram::{DiagramError, INDICES_MASK, VERSION_MASK};

impl SimpleDiagram {
    /// restore SimpleDiagram from binary data
    pub fn deserialize(mut secret: Vec<u8>) -> DiagramResult<SimpleDiagram> {
        // must have content
        if secret.len() <= 8 {
            return Err(DiagramError::InvalidParameter("secret too short."));
        }
        // tail byte is checksum
        if let Some(check) = secret.pop() {
            if check != sha256::Hash::hash(&secret).as_byte_array()[0] {
                return Err(DiagramError::InvalidParameter("checksum fail."));
            }
        }
        // 7 bytes indices
        let indices: Vec<u8> = secret.split_off(secret.len() - 7);
        // check version
        if indices.iter().any(|v| v & VERSION_MASK != 0) {
            return Err(DiagramError::InvalidVersion);
        }
        // residue must be a valid UTF-8 string
        let s = String::from_utf8(secret)
            .or(Err(DiagramError::InvalidParameter("invalid utf8 chars.")))?;
        let mut items: Vec<Option<char>> = s.chars().map(Some).collect();

        // fill diagram
        let mut data = [[None; 7]; 7];
        for (col, mask) in INDICES_MASK.iter().enumerate() {
            for (row, ind) in indices.iter().enumerate().take(7) {
                if ind & mask > 0 {
                    match items.pop() {
                        Some(Some(ch)) => data[row][col] = Some(ch),
                        _ => return Err(DiagramError::InvalidParameter("items len invalid.")),
                    }
                }
            }
        }
        if !items.is_empty() {
            return Err(DiagramError::InvalidParameter("items len invalid."));
        }
        Ok(SimpleDiagram(data))
    }

    /// create SimpleDiagram from items
    pub fn from_values(items: &[char], indices: &[(usize, usize)]) -> DiagramResult<Self> {
        let mut data = [[None; 7]; 7];
        items
            .iter()
            .zip(indices)
            .for_each(|(&v, &(r, c))| data[r][c] = Some(v));
        Ok(SimpleDiagram(data))
    }
}

#[cfg(test)]
mod simple_diagram_test {
    use super::*;
    use bitcoin::hex::{DisplayHex, FromHex};

    #[test]
    fn test_simple_secret() -> GenericResult<()> {
        const CHARS_STR: &str = "A&*王😊";
        const CHARS_INDICES: &[(usize, usize)] = &[(0, 6), (1, 1), (1, 3), (4, 2), (6, 6)];
        const SECRET_HEX: &str = "f09f988a412ae78e8b26012800001000012d";

        let mut art = SimpleDiagram([[None; 7]; 7]);
        CHARS_INDICES
            .iter()
            .zip(CHARS_STR.chars())
            .for_each(|(&(row, col), ch)| art[row][col] = Some(ch));
        assert_eq!(art.serialize()?.to_lower_hex_string(), SECRET_HEX);

        // from_raw
        let art = SimpleDiagram::deserialize(Vec::from_hex(SECRET_HEX).expect("TEST_SECRET_HEX"))?;
        assert_eq!(art[6][6], Some('😊'));
        assert_eq!(art.serialize()?.to_lower_hex_string(), SECRET_HEX);
        Ok(())
    }

    #[test]
    fn test_simple_entropy() -> GenericResult<()> {
        const RAW_SECRET_HEX: &str = "41262ae78e8bf09f988a012800001000406d";
        const WARP_ENTROPY: &str =
            "0948fd6d7b1dc397d26080804870913abc086636d3ed11d4fcb0f16f7c31a91a";
        const SALT_STR: &str = "123abc";
        const SALT_ENTROPY: &str =
            "e06ffd848c7901ca5757d848e5e81d69f9853273bee6772dcd25f56c506a1635";

        let secret = Vec::from_hex(RAW_SECRET_HEX).expect("RAW_SECRET_HEX");
        let art = SimpleDiagram::deserialize(secret)?;
        let entropy = art.warp_entropy(Default::default())?;
        assert_eq!(entropy.to_lower_hex_string(), WARP_ENTROPY);

        let salt_entropy = art.warp_entropy(SALT_STR.as_bytes())?;
        assert_eq!(salt_entropy.to_lower_hex_string(), SALT_ENTROPY);

        Ok(())
    }
}
