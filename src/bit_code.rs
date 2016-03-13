use std::cmp::min;


pub struct BitCode {
    bits: Vec<u64>,
}


impl BitCode {
    /*
    Create a BitCode from a string of binary digits.

    Raw data is stored as bits packed into as many u8s as are required.
    */
    pub fn from_str(bit_string: &str) -> BitCode {
        let mut bits: Vec<u64> = Vec::new();
        for (i, c) in bit_string.chars().enumerate() {
            let vector_pos = i / 64;
            let digit_pos = (i as u32) % 64u32;
            let value = 2u64.pow(digit_pos);
            if bits.len() <= vector_pos {
                bits.push(0);
            }
            if c == '1' { bits[vector_pos] += value; }
        }
        BitCode{ bits: bits }
    }

    /*
    Returns the Hamming distance between 2 BitCodes.
    */
    pub fn hamming_distance(&self, other: &BitCode) -> usize {
        let mut d: usize = 0;
        let n = min(self.bits.len(), other.bits.len());
        for i in 0..n {
            d += (self.bits[i] ^ other.bits[i]).count_ones() as usize;
        }
        d
    }
}


#[cfg(test)]
mod tests {
    use super::BitCode;
    use utils::random_bit_string;

    #[test]
    fn new_bit_code_from_bit_string() {
        let bc = BitCode::from_str("010101010101");
        assert_eq!(bc.bits.len(), 1);
        assert_eq!(bc.bits[0], 2730);
    }

    #[test]
    fn new_bit_code_from_random_bit_string() {
        let s = random_bit_string(256);
        let bc = BitCode::from_str(&s);
        assert_eq!(bc.bits.len(), 4);
    }

    #[test]
    fn hamming_distance() {
        let bc1 = BitCode::from_str("010101010101");
        assert_eq!(bc1.hamming_distance(&bc1), 0);
        let bc2 = BitCode::from_str("101010101010");
        assert_eq!(bc1.hamming_distance(&bc2), 12);
    }
}
