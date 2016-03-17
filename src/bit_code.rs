use std::cmp::min;


pub struct BitCode {
    bits: Vec<u64>,
}


impl BitCode {
    /*
    Create a BitCode from a string of binary digits.
    Raw data is stored as bits packed into as many u8s as are required.
    */
    pub fn from_bit_string(bit_string: &str) -> BitCode {
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

    pub fn from_bit_vector(bit_vector: Vec<bool>) -> BitCode {
        let mut bits: Vec<u64> = Vec::new();
        for (i, b) in bit_vector.iter().enumerate() {
            let vector_pos = i / 64;
            let digit_pos = (i as u32) % 64u32;
            let value = 2u64.pow(digit_pos);
            if bits.len() <= vector_pos {
                bits.push(0);
            }
            if *b { bits[vector_pos] += value; }
        }
        BitCode{ bits: bits }
    }

    /*
    Returns the Hamming distance between 2 BitCodes.
    */
    pub fn hamming_distance(&self, other: &BitCode) -> usize {
        let mut d: usize = 0;
        for i in 0..min(self.bits.len(), other.bits.len()) {
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
        let bc = BitCode::from_bit_string("010101010101");
        assert_eq!(bc.bits.len(), 1);
        assert_eq!(bc.bits[0], 2730);
        assert_eq!(bc.hamming_distance(&bc), 0);
    }

    #[test]
    fn new_bit_code_from_bit_vector() {
        let bit_vector: Vec<bool> = vec![false, true, false, true, false, true, false, true, false, true, false, true];
        let bc = BitCode::from_bit_vector(bit_vector);
        assert_eq!(bc.bits.len(), 1);
        assert_eq!(bc.bits[0], 2730);
        assert_eq!(bc.hamming_distance(&bc), 0);
    }

    #[test]
    fn bit_codes_are_equal() {
        let bc1 = BitCode::from_bit_string("010101010101");
        let bc2 = BitCode::from_bit_vector(vec![false, true, false, true, false, true, false, true, false, true, false, true]);
        assert_eq!(bc1.hamming_distance(&bc2), 0);
    }

    #[test]
    fn new_bit_code_from_random_bit_string() {
        let bc = BitCode::from_bit_string(&random_bit_string(256));
        assert_eq!(bc.bits.len(), 4);
    }

    #[test]
    fn hamming_distance() {
        let bc1 = BitCode::from_bit_string("010101010101");
        assert_eq!(bc1.hamming_distance(&bc1), 0);
        let bc2 = BitCode::from_bit_string("101010101010");
        assert_eq!(bc1.hamming_distance(&bc2), 12);
    }
}
