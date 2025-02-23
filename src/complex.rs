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

use crate::macros::ImpDeref;

use super::generic::{GenericDiagram, GenericResult};
use bitcoin::hashes::{sha256, Hash};
use serde::{Deserialize, Serialize};

/// Complex Diagram
///
/// Complex diagram contains strings in 7 * 7 grid cells.
/// All UTF-8 strings with less than 50 characters are supported.
///
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplexDiagram(pub [[Option<String>; 7]; 7]);
ImpDeref!(ComplexDiagram, [[Option<String>; 7]; 7]);

impl GenericDiagram<7, 7> for ComplexDiagram {
    type Item = String;

    /// Compatible with previous versions
    fn to_bytes(&self) -> GenericResult<Vec<u8>> {
        let mut str_list: Vec<&str> = vec![];
        let mut str_lens: Vec<u8> = vec![];
        let mut indices: [u8; 7] = [0; 7];

        (0..7).rev().for_each(|col| {
            (0..7).rev().for_each(|row| {
                if let Some(s) = &self[row][col] {
                    if !s.is_empty() && s.len() < u8::MAX as usize {
                        str_list.push(s);
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

const INDICES_MASK: [u8; 7] = [
    0b0100_0000,
    0b0010_0000,
    0b0001_0000,
    0b0000_1000,
    0b0000_0100,
    0b0000_0010,
    0b0000_0001,
];
const VERSION_MASK: u8 = 0b1000_0000;

impl ComplexDiagram {
    /// cell chars count limit
    pub const CELL_CHARS_LIMIT: usize = 50;

    /// create complex diagram
    pub fn new() -> Self {
        Self(core::array::from_fn(|_| core::array::from_fn(|_| None)))
    }

    /// create ComplexDiagram from items
    pub fn from_values(items: &[&str], indices: &[(usize, usize)]) -> Self {
        let mut diagram = ComplexDiagram::new();
        indices.iter().zip(items).for_each(|(&(r, c), &s)| {
            diagram[r][c] = match s.is_empty() {
                false => Some(s.to_owned()),
                true => None,
            }
        });
        diagram
    }
}

#[cfg(test)]
mod complex_diagram_test {
    use super::*;
    use bitcoin::hex::DisplayHex;

    #[test]
    fn test_complex_diagram() -> GenericResult {
        const STR_LIST: &[&str] = &["ABC", "123", "测试", "混A1", "A&*王😊"];
        const INDICES: &[(usize, usize)] = &[(0, 6), (1, 1), (1, 3), (4, 2), (6, 6)];
        const SECRET_HEX: &str =
            "41262ae78e8bf09f988a414243e6b58be8af95e6b7b741313132330a0306050381280000100001c8";

        let cdm = ComplexDiagram::from_values(STR_LIST, INDICES);
        assert_eq!(cdm.to_bytes()?.to_lower_hex_string(), SECRET_HEX);
        assert_eq!(cdm[6][6], Some(STR_LIST[4].to_owned()));

        Ok(())
    }

    #[test]
    fn test_complex_entropy() -> GenericResult<()> {
        const STR_LIST: &[&str] = &["ABC", "混A1", "123", "测试", "A&*王😊"];
        const INDICES: &[(usize, usize)] = &[(0, 6), (1, 1), (1, 3), (4, 2), (6, 0)];
        const SECRET_HEX: &str =
            "414243313233e6b58be8af95e6b7b7413141262ae78e8bf09f988a030306050a8128000010004052";
        const RAW_ENTROPY: &str =
            "f273657eb2394dbe4874571abf8d6f78b149bd86d1eec6c666509371e93004d3";
        const SALT_STR: &str = "123abc";
        const SALT_ENTROPY: &str =
            "3ff854b9f188d428068e3a9b7655d37795f1aaf1e6461b757f12935dee796bbf";

        let cdm = ComplexDiagram::from_values(STR_LIST, INDICES);
        assert_eq!(cdm.to_bytes()?.to_lower_hex_string(), SECRET_HEX);

        let entropy = cdm.warp_entropy(Default::default())?;
        assert_eq!(entropy.to_lower_hex_string(), RAW_ENTROPY);

        let salt_entropy = cdm.warp_entropy(SALT_STR.as_bytes())?;
        assert_eq!(salt_entropy.to_lower_hex_string(), SALT_ENTROPY);

        Ok(())
    }
}
