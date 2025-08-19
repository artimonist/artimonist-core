/*!
 * # Reference
 *
 * [1] - Unicode characters
 *      <https://www.unicodepedia.com/>
 *
 * # Descriptions
 *
 * [1] - Animate Diagram secret data construction
 *      (diagram version == 2)  
 *      |--utf8 chars---|----n*7 bytes----|-1 byte-|  
 *      |Char1|Char2|...| n Frame Indices |CheckSum|  
 *      |------>>>------|-------<<<-------|--------|  
 *      frame order from right to left, until end frame.
 *      chars count = indices.count_ones() - n - 1
 *      chars order from left to right.
 *
 * [2] - Animate Diagram frame indices data construction
 *      0b0xxx_xxxx
 *      0b1xxx_xxxx
 *      0b0xxx_xxxx
 *      0b0xxx_xxxx
 *      0b0xxx_xxxx
 *      0b0xxx_xxxx
 *      0b0xxx_xxxx
 *      x bits indices char position in diagram.
 * [3] - Animate Diagram end frame indices data construction
 *      0b1xxx_xxxx
 *      0b1xxx_xxxx
 *      0b0xxx_xxxx
 *      0b0xxx_xxxx
 *      0b0xxx_xxxx
 *      0b0xxx_xxxx
 *      0b0xxx_xxxx
 *      x bits indices char position in diagram.
**/
use super::{GenericDiagram, Result};
use bitcoin::hashes::{Hash, sha256};

/// Animate Diagram
#[derive(Debug, Clone, PartialEq)]
pub struct AnimateDiagram(pub Vec<[[Option<char>; 7]; 7]>);

impl GenericDiagram<7, 7> for AnimateDiagram {
    type Item = char;

    fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut chars = Vec::new();
        let mut frames = Vec::new();
        self.0.iter().rev().for_each(|mx| {
            let mut indices = [0_u8; 7];
            (0..7).rev().for_each(|col| {
                (0..7).rev().for_each(|row| {
                    if let Some(ch) = mx[row][col] {
                        chars.push(ch);
                        indices[row] |= 1 << (6 - col);
                    }
                });
            });
            indices[1] |= 1 << 7; // version number of animate diagram
            frames.push(indices);
        });
        frames[0][0] |= 1 << 7; // end frame of animate diagram

        let str = chars.into_iter().collect::<String>();
        let mut secret = [str.as_bytes(), &frames.concat()].concat();
        let check = sha256::Hash::hash(&secret).as_byte_array()[0];
        secret.push(check);
        Ok(secret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animate() -> Result<()> {
        let mut cube = vec![[[None; 7]; 7], [[Some('X'); 7]; 7], [[None; 7]; 7]];
        cube[0][3][3] = Some('X');
        cube[2][6][6] = Some('X');
        let diagram = AnimateDiagram(cube);

        let bytes = diagram.to_bytes()?;
        assert_eq!(
            &bytes[51..],
            [
                128, 128, 0, 0, 0, 0, 1, // end frame
                127, 255, 127, 127, 127, 127, 127, // frame 1
                0, 128, 0, 8, 0, 0, 0,  // frame 0
                24, // checksum
            ]
        );

        let master = diagram.bip32_master("123456".as_bytes())?;
        #[cfg(not(feature = "testnet"))]
        const MASTER: &str = "xprv9s21ZrQH143K3LZCYpVpieDdNgMkymbiawMFUAPysrTJJrkrVCpaFtEUXBpr32nVVeHjPtqRCqih7ptuiu5A34VHPPPZMZfqisJyV6jhCwU";
        #[cfg(feature = "testnet")]
        const MASTER: &str = "tprv8ZgxMBicQKsPe9njDPMKtHqcgomyDHdivVGNLapSMpwn6TVwUaAKmdbvSMzW3QAos5pWPzTBNCJVagSer7R6r7ksv2bs1vPtdy4Pvr2uKuZ";
        assert_eq!(master.to_string(), MASTER);
        Ok(())
    }
}
