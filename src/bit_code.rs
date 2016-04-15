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
        // Bits per index must be within range of block size.
        if bits_per_index < 1 { bits_per_index = 1 };
        if bits_per_index > 64 { bits_per_index = 64 };
        // Calculate number of indexes.
        let n = self.num_bits();
        let num_indexes = n / bits_per_index;
        // Iterate over bits setting index values.
        let mut index_values: Vec<u64> = vec![0; num_indexes];
        for i in 0..n {
            let index_num = i / bits_per_index;
            let position_num = i % bits_per_index;
            if self.get(i).unwrap() { index_values[index_num] |= 1 << position_num } ;
        }
        index_values
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
