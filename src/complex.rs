/*!
 * # Reference
 *
 * [1] - Unicode characters
 *      <https://www.unicodepedia.com/>
 *
 * # Descriptions
 *
 * [1] - Complex Diagram secret data construction
 *      |-----n segments----|-n bytes-|-7 bytes-|-1 byte-|
 *      |String1|String2|...|N1|N2|...| Indices |CheckSum|
 *      |-------------------|---------|---------|--------|
 *      n = indices.count_ones() - 1  (version == 1)
 *      N1,N2... is bytes count of String1,String2...
 *
 * [2] - Complex Diagram indices data construction
 *      0b1xxx_xxxx
 *      0b0xxx_xxxx
 *      0b0xxx_xxxx
 *      0b0xxx_xxxx
 *      0b0xxx_xxxx
 *      0b0xxx_xxxx
 *      0b0xxx_xxxx
 *      1 bit at top left corner is version of complex diagram.
 *      others x bits indices string position in diagram.
**/

use super::diagram::*;
use super::generic::{GenericDiagram, GenericResult, GenericSerialization};
use bitcoin::hashes::{sha256, Hash};
use serde::{Deserialize, Serialize};

pub const CELL_CHARS_LIMIT: usize = 50;

/// Complex Diagram
///
/// Complex diagram contains strings in 7 * 7 grid cells.
/// All UTF-8 strings with less than 50 characters are supported.
///
/// # Examples
/// ```
/// # use artimonist::{Diagram, ComplexDiagram};
/// # use bitcoin::hex::FromHex;
/// let secret = Vec::from_hex("41262ae78e8bf09f988a414243e6b58be8af95e6b7b741313132330a0306050381280000100001c8").unwrap_or_default();
/// let mut diagram = ComplexDiagram::from_secret(secret)?;
///
/// assert_eq!(diagram[6][6], Some("A&*王😊".to_owned()));
///
/// # Ok::<(), artimonist::Error>(())
/// ```
///
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplexDiagram(pub [[Option<String>; 7]; 7]);

impl std::ops::Deref for ComplexDiagram {
    type Target = [[Option<String>; 7]; 7];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ComplexDiagram {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl GenericDiagram<7, 7, char> for ComplexDiagram {}
impl GenericSerialization for ComplexDiagram {
    /// Compatible with previous versions
    fn binary(&self) -> GenericResult<Vec<u8>> {
        let mut str_list: Vec<&str> = vec![];
        let mut str_lens: Vec<u8> = vec![];
        let mut indices: [u8; 7] = [0; 7];

        (0..7).rev().for_each(|col| {
            (0..7).rev().for_each(|row| {
                if let Some(s) = &self[row][col] {
                    if 0 < s.len() && s.len() < u8::MAX as usize {
                        str_list.push(&s);
                        str_lens.push(s.len() as u8);
                        indices[row] |= INDICES_MASK[col];
                    }
                }
            });
        });

        indices[0] |= VERSION_MASK; // version number of complex diagram
        let mut secret = [str_list.join("").as_bytes(), &str_lens[..], &indices[..]].concat();
        let check = sha256::Hash::hash(&secret).as_byte_array()[0];
        secret.push(check);
        Ok(secret)
    }
}

use super::diagram::DiagramResult;

impl ComplexDiagram {
    /// create complex diagram
    pub fn new() -> Self {
        Self(core::array::from_fn(|_| core::array::from_fn(|_| None)))
    }

    /// create diagram from secret data
    pub fn from_secret(mut secret: Vec<u8>) -> DiagramResult<Self> {
        // must have content
        if secret.len() < 10 {
            return Err(DiagramError::InvalidParameter("secret too short.")); // invalid len
        }
        // tail byte is checksum
        if let Some(check) = secret.pop() {
            if check != sha256::Hash::hash(&secret).as_byte_array()[0] {
                return Err(DiagramError::InvalidParameter("checksum fail.")); // invalid checksum
            }
        }

        // tail 7 bytes is indices
        let indices: Vec<u8> = secret.split_off(secret.len() - 7);
        let (version, item_count) = indices
            .iter()
            .enumerate()
            .map(|(i, &v)| {
                (
                    (v & VERSION_MASK) >> (7 - i),
                    (v & INDICES_ALL).count_ones() as usize,
                )
            })
            .reduce(|(ver, count), (v, n)| (ver + v, count + n))
            .unwrap_or_default();
        if version != 1 {
            return Err(DiagramError::InvalidVersion); // invalid version
        }
        if item_count == 0 {
            return Err(DiagramError::InvalidParameter("indices is empty.")); // empty indices
        }

        // string lens
        let mut str_lens: Vec<u8> = secret.split_off(secret.len() - item_count);
        let (amount, has_zero) = str_lens
            .iter()
            .fold((0, false), |(a, z), &v| (a + v as usize, z || v == 0));
        if str_lens.is_empty() || has_zero || amount != secret.len() {
            return Err(DiagramError::InvalidParameter("str lens invalid.")); // invalid str lens
        }

        // fill data
        let mut diagram = ComplexDiagram::new();
        for (col, mask) in INDICES_MASK.iter().enumerate() {
            for (row, ind) in indices.iter().enumerate().take(7) {
                if ind & mask > 0 {
                    let n = str_lens.pop().unwrap_or_default();
                    let bs = secret.split_off(secret.len() - n as usize);
                    match String::from_utf8(bs) {
                        Ok(s) => diagram[row][col] = Some(s),
                        Err(_) => {
                            return Err(DiagramError::InvalidParameter("invalid utf8 string."))
                        } // invalid utf8
                    }
                }
            }
        }
        Ok(diagram)
    }
}

#[cfg(test)]
mod complex_diagram_test {
    use super::super::generic::{GenericDiagram, GenericSerialization};
    use super::*;
    use bitcoin::hex::{DisplayHex, FromHex};

    #[test]
    fn test_complex_secret() {
        const STR_LIST: &[&str] = &["ABC", "123", "测试", "混A1", "A&*王😊"];
        const INDICES: &[(usize, usize)] = &[(0, 6), (1, 1), (1, 3), (4, 2), (6, 6)];
        const SECRET_HEX: &str =
            "41262ae78e8bf09f988a414243e6b58be8af95e6b7b741313132330a0306050381280000100001c8";

        let mut diagram = crate::ComplexDiagram::new();
        INDICES
            .iter()
            .zip(STR_LIST)
            .for_each(|(&(row, col), &s)| diagram[row][col] = Some(s.to_owned()));
        let secret = diagram.binary().unwrap_or_default();
        assert_eq!(secret.to_lower_hex_string(), SECRET_HEX);

        if let Ok(diagram) = ComplexDiagram::from_secret(secret) {
            assert_eq!(diagram[6][6], Some(STR_LIST[4].to_owned()));
            let secret = diagram.binary().unwrap_or_default();
            assert_eq!(secret.to_lower_hex_string(), SECRET_HEX);
        } else {
            assert!(false, "from_secret() fail");
        }
    }

    #[test]
    fn test_complex_entropy() -> GenericResult<()> {
        const SECRET_HEX: &str =
            "414243313233e6b58be8af95e6b7b7413141262ae78e8bf09f988a030306050a8128000010004052";
        const RAW_ENTROPY: &str =
            "f273657eb2394dbe4874571abf8d6f78b149bd86d1eec6c666509371e93004d3";
        const SALT_STR: &str = "123abc";
        const SALT_ENTROPY: &str =
            "3ff854b9f188d428068e3a9b7655d37795f1aaf1e6461b757f12935dee796bbf";

        let secret = Vec::from_hex(SECRET_HEX).expect("SECRET_HEX");
        let diag = ComplexDiagram::from_secret(secret)?;
        let entropy = diag.warp_entropy(Default::default())?;
        assert_eq!(entropy.to_lower_hex_string(), RAW_ENTROPY);

        let salt_entropy = diag.warp_entropy(SALT_STR.as_bytes())?;
        assert_eq!(salt_entropy.to_lower_hex_string(), SALT_ENTROPY);

        Ok(())
    }
}
