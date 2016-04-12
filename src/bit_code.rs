use std::cmp::min;


#[derive(Debug)]
pub struct BitCode {
    blocks: Vec<u64>, // Blocks of u64s to store bits.
}


impl BitCode {

    pub fn new(num_bits: usize) -> Self {
        let num_blocks = ((num_bits - 1) / 64) + 1;
        BitCode{ blocks: vec![0; num_blocks] }
    }

    pub fn from_bools(bools: &Vec<bool>) -> Self {
        let bit_blocks = ((bools.len() - 1) / 64) + 1;
        let mut bc = BitCode{ blocks: vec![0; bit_blocks] };
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
        let mut bc = BitCode{ blocks: vec![0; bit_blocks] };
        for (i, b) in bools.iter().enumerate() {
            bc.set(i, *b);
        }
        bc
    }

    #[inline]
    pub fn get(&self, bit_number: usize) -> Option<bool> {
        if bit_number >= self.num_bits() {
            return None;
        }
        let block_num = bit_number / 64;
        let block_pos = bit_number % 64;
        Some((self.blocks[block_num] & (1 << block_pos)) != 0)
    }

    #[inline]
    pub fn get_block(&self, block_number: usize) -> Option<u64> {
        if block_number >= self.num_blocks() {
            return None;
        }
        Some(self.blocks[block_number])
    }

    pub fn multi_index_values(&self, mut bits_per_index: usize) -> Vec<u64> {
        // Ensure bit_per_index is acceptable.
        if (bits_per_index < 2) || (bits_per_index > 64) || ((64 % bits_per_index) != 0) {
            bits_per_index = self.bits_per_index_ideal();
        }
        // Get masks to be applied to each block.
        let mut masks: Vec<u64> = Vec::new();
        for i in 0..(64 / bits_per_index) {
            let mut mask: u64 = 0;
            for j in 0..bits_per_index {
                let bit_index = i * bits_per_index + j;
                mask |= 1 << bit_index;
            }
            masks.push(mask);
        }
        // Get index keys by applying masks to each block.
        let mut keys: Vec<u64> = Vec::new();
        for block in &self.blocks {
            for mask in &masks {
                keys.push(block & mask);
            }
        }
        keys
    }

    #[inline]
    pub fn num_bits(&self) -> usize {
        self.blocks.len() * 64
    }

    #[inline]
    pub fn num_blocks(&self) -> usize {
        self.blocks.len()
    }

    #[inline]
    pub fn bits_per_index_ideal(&self) -> usize {
        let mut bits_per_index = (self.num_bits() as f64).log2().floor() as usize;
        loop {
            if (64 % bits_per_index) == 0 {
                break;
            }
            bits_per_index += 1;
        }
        if bits_per_index < 2 {
            bits_per_index = 2;
        }
        if bits_per_index > 64 {
            bits_per_index = 64;
        }
        bits_per_index
    }

    #[inline]
    pub fn set(&mut self, bit_number: usize, value: bool) {
        if bit_number < self.num_bits() {
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
    fn set_get() {
        let mut bc = BitCode::new(512);
        assert_eq!(bc.num_bits(), 512);
        assert_eq!(bc.num_blocks(), 8);
        assert_eq!(bc.count_ones(), 0);
        bc.set(10, true);
        bc.set(20, true);
        bc.set(30, true);
        assert_eq!(bc.count_ones(), 3);
        assert_eq!(bc.get(10), Some(true));
        bc.set(10, false);
        assert_eq!(bc.get(10), Some(false));
        assert_eq!(bc.count_ones(), 2);
    }

    #[test]
    fn multi_index_values() {
        let bc = BitCode::from_string("010101010101");
        let keys = bc.multi_index_values(4);
        assert_eq!(keys.len(), 16);
    }

    #[test]
    fn new_bit_code_from_string() {
        let bc = BitCode::from_string("010101010101");
        assert_eq!(bc.num_bits(), 64);
        assert_eq!(bc.num_blocks(), 1);
        assert_eq!(bc.count_ones(), 6);
        assert_eq!(bc.get_block(0), Some(2730));
        assert_eq!(bc.hamming_distance(&bc), 0);
    }

    #[test]
    fn new_bit_code_from_bools() {
        let bools: Vec<bool> = vec![false, true, false, true, false, true, false, true, false, true, false, true];
        let bc = BitCode::from_bools(&bools);
        assert_eq!(bc.num_bits(), 64);
        assert_eq!(bc.num_blocks(), 1);
        assert_eq!(bc.count_ones(), 6);
        assert_eq!(bc.get_block(0), Some(2730));
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
        assert_eq!(bc.num_bits(), 256);
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
