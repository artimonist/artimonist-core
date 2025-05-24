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

///
#[derive(Debug, Default, Clone, PartialEq)]
pub struct AnimateDiagram(pub Vec<[[Option<char>; 7]; 7]>);

impl CubeDiagram<7, 7> for AnimateDiagram {
    type Item = char;

    fn to_bytes(&self) -> GenericResult<Vec<u8>> {
        let mut chars = Vec::new();
        let mut frames = Vec::new();
        self.0.rev().for_each(|mx| {
            let mut indices = [0; 7];
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
        let mut secret = [&str.as_bytes(), &frames[..]].concat();
        let check = sha256::Hash::hash(&secret).as_byte_array()[0];
        secret.push(check);
        Ok(secret)
    }
}

impl std::convert::From<Vec<[[Option<char>; 7]; 7]>> for AnimateDiagram {
    fn from(mx: Vec<[[Option<char>; 7]; 7]>) -> Self {
        Self(mx)
    }
}
