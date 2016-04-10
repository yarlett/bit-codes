use std::cmp::min;

#[derive(Debug)]
pub struct BitCode {
    blocks: Vec<u64>,
    nbits: usize,
}

impl BitCode {

    pub fn new(nbits: usize) -> Self {
        let bit_blocks = ((nbits - 1) / 64) + 1;
        BitCode{ blocks: vec![0; bit_blocks], nbits: nbits}
    }

    pub fn from_bools(bools: &Vec<bool>) -> Self {
        let bit_blocks = ((bools.len() - 1) / 64) + 1;
        let mut bc = BitCode{ blocks: vec![0; bit_blocks], nbits: bools.len() };
        for (i, b) in bools.iter().enumerate() {
            bc.set(i, *b);
        }
        bc
    }

    pub fn from_string(string: &str) -> Self {
        let mut bools: Vec<bool> = Vec::new();
        for c in string.chars() {
            if c == '1' { bools.push(true); }
            else { bools.push(false); }
        }
        let bit_blocks = ((bools.len() - 1) / 64) + 1;
        let mut bc = BitCode{ blocks: vec![0; bit_blocks], nbits: bools.len() };
        for (i, b) in bools.iter().enumerate() {
            bc.set(i, *b);
        }
        bc
    }

    #[inline]
    pub fn get(&self, bit_number: usize) -> Option<bool> {
        if bit_number >= self.nbits {
            return None;
        }
        let block_num = bit_number / 64;
        let block_pos = bit_number % 64;
        return Some((self.blocks[block_num] & (1 << block_pos)) != 0)
    }

    pub fn len(&self) -> usize {
        self.nbits
    }

    #[inline]
    pub fn set(&mut self, bit_number: usize, value: bool) {
        if bit_number < self.nbits {
            let block_num = bit_number / 64;
            let block_pos = bit_number % 64;
            let flag = 1 << block_pos;
            if value {
                self.blocks[block_num] |= flag;
            } else {
                self.blocks[block_num] &= !flag;
            }
        }
    }

    #[inline]
    pub fn count_ones(&self) -> u32 {
        let mut o: u32 = 0;
        for block in &self.blocks {
            o += block.count_ones();
        }
        o
    }

    #[inline]
    pub fn hamming_distance(&self, other: &BitCode) -> u32 {
        let mut d: u32 = 0;
        for i in 0..min(self.blocks.len(), other.blocks.len()) {
            d += (self.blocks[i] ^ other.blocks[i]).count_ones();
        }
        d
    }
}


#[cfg(test)]
mod tests {
    use super::BitCode;
    use utils::random_bit_string;

    #[test]
    fn new_bit_code_from_string() {
        let bc = BitCode::from_string("010101010101");
        assert_eq!(bc.len(), 12);
        assert_eq!(bc.count_ones(), 6);
        assert_eq!(bc.hamming_distance(&bc), 0);
    }

    #[test]
    fn new_bit_code_from_bools() {
        let bools: Vec<bool> = vec![false, true, false, true, false, true, false, true, false, true, false, true];
        let bc = BitCode::from_bools(&bools);
        assert_eq!(bc.len(), 12);
        assert_eq!(bc.count_ones(), 6);
        for (i, b) in bools.iter().enumerate() {
            match bc.get(i) {
                Some(x) => assert_eq!(&x, b),
                None => assert!(false),
            }
        }
        assert_eq!(bc.hamming_distance(&bc), 0);
    }

    #[test]
    fn bit_codes_are_equal() {
        let bc1 = BitCode::from_string("010101010101");
        let bc2 = BitCode::from_bools(&vec![false, true, false, true, false, true, false, true, false, true, false, true]);
        assert_eq!(bc1.hamming_distance(&bc2), 0);
    }

    #[test]
    fn new_bit_code_from_random_string() {
        let bc = BitCode::from_string(&random_bit_string(256));
        assert_eq!(bc.len(), 256);
    }

    #[test]
    fn hamming_distance() {
        let bc1 = BitCode::from_string("010101010101");
        assert_eq!(bc1.hamming_distance(&bc1), 0);
        let bc2 = BitCode::from_string("101010101010");
        assert_eq!(bc1.hamming_distance(&bc1), 0);
        assert_eq!(bc2.hamming_distance(&bc2), 0);
        assert_eq!(bc1.hamming_distance(&bc2), 12);
    }
}
