pub struct BitCode {
    code: Vec<u8>,
    key: u64,
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
}


#[cfg(test)]
mod tests {
    use super::BitCode;

    #[test]
    fn new_bit_code_from_str() {
        let bc = BitCode::from_str("010101010101", 13);
        assert_eq!(bc.code.len(), 2);
        assert_eq!(bc.code[0], 170);
        assert_eq!(bc.code[1], 10);
        assert_eq!(bc.key, 13);
    }
}
