/*! Bit Operations
 *
 * # Examples
 * [u8] chunks to mnemonic indices by 11 bits.
 * [u8] chunks to base64 indices by 6 bits.
 */

trait BitInner {
    /// [u8] with less than 32 bits are trailed with zeros
    fn window_value(&self, i: usize) -> u32;
}

impl BitInner for [u8] {
    fn window_value(&self, i: usize) -> u32 {
        if i + 3 < self.len() {
            u32::from_be_bytes([self[i], self[i + 1], self[i + 2], self[i + 3]])
        } else {
            u32::from_be_bytes(match i {
                _ if i + 3 == self.len() => [self[i], self[i + 1], self[i + 2], 0],
                _ if i + 2 == self.len() => [self[i], self[i + 1], 0, 0],
                _ if i + 1 == self.len() => [self[i], 0, 0, 0],
                _ => [0, 0, 0, 0],
            })
        }
    }
}

pub trait BitOperation {
    /// compound bits to u8 vector
    fn bit_from(iter: &mut impl Iterator<Item = bool>) -> Vec<u8>;
    /// split [u8] as bits array
    fn bit_chunks(&self, n: usize) -> impl Iterator<Item = u16>;
    /// iterator of bit values
    fn bit_iter(&self) -> impl Iterator<Item = bool>;
}

impl BitOperation for [u8] {
    fn bit_from(iter: &mut impl Iterator<Item = bool>) -> Vec<u8> {
        let mut v = 0_u8;
        iter.chain([false; 7])
            .enumerate()
            .filter_map(|(i, b)| {
                let m = (i + 1) % 8;
                if b {
                    v |= 1 << (8 - m);
                }
                if m != 0 {
                    return None;
                }
                let result = Some(v);
                v = 0;
                result
            })
            .collect()
    }

    fn bit_chunks(&self, n: usize) -> impl Iterator<Item = u16> {
        assert!(n <= 16);
        let bit_mask = (0..n).fold(0_u32, |acc, v| acc | (1 << v));
        let mut start = 0; // bit window start
        (0..self.len()).flat_map(move |i| {
            assert!(i * 8 <= start && start <= i * 8 + 32);
            let mut vs = vec![];
            let end = i * 8 + 32; // bit window end
            while (start + n) <= end && end < (self.len() * 8 + n) {
                start += n;
                let value = ((self.window_value(i) >> (end - start)) & bit_mask) as u16;
                vs.push(value);
            }
            vs
        })
    }

    fn bit_iter(&self) -> impl Iterator<Item = bool> {
        const MASK: [u8; 8] = [
            0b1000_0000,
            0b0100_0000,
            0b0010_0000,
            0b0001_0000,
            0b0000_1000,
            0b0000_0100,
            0b0000_0010,
            0b0000_0001,
        ];
        self.iter()
            .flat_map(|&v| MASK.iter().map(move |&m| (v & m) > 0))
    }
}

pub trait BitAggregation {
    fn to_bits(self) -> Vec<u8>;
}

impl<T: Iterator<Item = bool>> BitAggregation for T {
    fn to_bits(mut self) -> Vec<u8> {
        <[u8] as BitOperation>::bit_from(&mut self)
    }
}

#[cfg(test)]
mod bit_operation_test {
    use std::vec;

    use super::*;

    #[test]
    fn test_bit_chunks() {
        use bitcoin::{hashes::Hash, hex::FromHex};
        {
            /// 01010001011 10100101110 11000111011 10111011111 11000110111 00010111111 10111101001 11001000111 11011111011 01111110011 00000001000 11010100100 10001101000 11001010110 111001
            const ENTROPY_15: &str = "5174bb1dddfc6e2fef4e47df6fcc046a48d195b9";
            const INDICES_15: &[u16] = &[
                651, 1326, 1595, 1503, 1591, 191, 1513, 1607, 1787, 1011, 8, 1700, 1128, 1622, 1842,
            ];
            let mut data = Vec::from_hex(ENTROPY_15).expect("ENTROPY_15");
            let check = bitcoin::hashes::sha256::Hash::hash(&data).as_byte_array()[0];
            data.extend([check]);
            let indices = data.bit_chunks(11).collect::<Vec<_>>();
            assert_eq!(&indices[..15], INDICES_15);
        }
        {
            /// 11011000100 01001010110 00110011000 00000101111 00001001100 11001010011 01110000001 00000010110 01000001000 11100110000 00101100011 00100011001 01011010101 01111100110 00011110101 10110101101 10100101101 01011010001 00111000011 00111111110 01011011100 01100010011 00010010001 011
            const ENTROPY_24: &str =
                "d88958cc02f09994dc0816411cc0b19195aaf987adada5ab44e19fe5b8c4c48b";
            const INDICES_24: &[u16] = &[
                1732, 598, 408, 47, 76, 1619, 897, 22, 520, 1840, 355, 281, 725, 998, 245, 1453,
                1325, 721, 451, 510, 732, 787, 145, 844,
            ];
            let data = Vec::from_hex(ENTROPY_24).expect("ENTROPY_24");
            let check = bitcoin::hashes::sha256::Hash::hash(&data).as_byte_array()[0];
            let entropy = [data, vec![check]].concat();
            let indices = entropy.bit_chunks(11).collect::<Vec<_>>();
            assert_eq!(&indices, INDICES_24);
        }
    }

    #[test]
    fn test_to_bits() {
        const MATRIX: &[[u8; 8]] = &[
            [1, 1, 1, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 1, 1, 1, 1],
            [1, 1, 0, 0, 0, 0, 1, 1],
            [0, 0, 1, 1, 1, 1, 0, 0],
        ];
        const INDICES: &[u8] = &[0b1111_0000, 0b0000_1111, 0b1100_0011, 0b0011_1100];
        let vs = (0..MATRIX.len())
            .flat_map(|row| (0..8).map(move |col| MATRIX[row][col] == 1))
            .to_bits();
        assert_eq!(vs, INDICES);

        let bits = [true, true, true, false, false, true].into_iter().to_bits();
        assert_eq!(bits, [0b1110_0100]);
        let bits = [true, true, true, false, false, true, true, true, true]
            .into_iter()
            .to_bits();
        assert_eq!(bits, [0b1110_0111, 0b1000_0000]);
    }
}
