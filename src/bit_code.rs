use std::cmp::min;


pub struct BitCode {
    code: Vec<u8>,
    pub key: u64,
}


impl BitCode {
    // Create a BitCode from a string of binary digits (0|1).
    pub fn from_str(string_bits: &str, key: u64) -> BitCode {
        let mut code: Vec<u8> = Vec::new();
        for (i, c) in string_bits.chars().enumerate() {
            let vector_pos = i / 8;
            let digit_pos = (i as u32) % 8u32;
            let value = 2u8.pow(digit_pos);
            if code.len() <= vector_pos {
                code.push(0);
            }
            if c == '1' { code[vector_pos] += value as u8; }
        }
        BitCode{ code: code, key: key }
    }

    // Returns Hamming distance between 2 BitCodes.
    pub fn hamming_distance(&self, other: &BitCode) -> u32 {
        let mut d: u32 = 0;
        let n = min(self.code.len(), other.code.len());
        for i in 0..n {
            d += (self.code[i] ^ other.code[i]).count_ones()
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
        let bc = BitCode::from_str("010101010101", 13);
        assert_eq!(bc.code.len(), 2);
        assert_eq!(bc.code[0], 170);
        assert_eq!(bc.code[1], 10);
        assert_eq!(bc.key, 13);
    }

    #[test]
    fn new_bit_code_from_random_bit_string() {
        let s = random_bit_string(256);
        println!("{:}", s);
        let _ = BitCode::from_str(&s, 13);
    }

    #[test]
    fn hamming_distance() {
        let bc1 = BitCode::from_str("010101010101", 0);
        assert_eq!(bc1.hamming_distance(&bc1), 0);
        let bc2 = BitCode::from_str("101010101010", 0);
        assert_eq!(bc1.hamming_distance(&bc2), 12);
    }
}
