use bit_vec::BitVec;
use encoding::string_to_bit_code;
use encoding_options::EncodingOptions;
use std::cmp::min;
use utils::get_num_indexes;


#[derive(Debug)]
pub struct BitCode {
    bits: BitVec,
}


impl BitCode {

    pub fn new(num_bits: usize) -> Self {
        BitCode{ bits: BitVec::from_elem(num_bits, false) }
    }

    pub fn from_bools(bools: &Vec<bool>) -> Self {
        let mut bits = BitVec::from_elem(bools.len(), false);
        for (i, b) in bools.iter().enumerate() {
            bits.set(i, *b);
        }
        BitCode{ bits: bits }
    }

    pub fn from_bit_string(string: &str) -> Self {
        let mut bits = BitVec::from_elem(string.len(), false);
        for (i, c) in string.chars().enumerate() {
            if c == '1' { bits.set(i, true); }
            else { bits.set(i, false); }
        }
        BitCode{ bits: bits }
    }

    pub fn from_string(string: &str, encoding_options: &EncodingOptions) -> Self {
        string_to_bit_code(string, encoding_options)
    }

    #[inline]
    pub fn count_ones(&self) -> usize {
        let mut n = 0;
        for block in self.bits.storage() { n += block.count_ones(); }
        n as usize
    }

    #[inline]
    pub fn get(&self, bit_number: usize) -> Option<bool> {
        self.bits.get(bit_number)
    }

    #[inline]
    pub fn hamming_distance(&self, other: &BitCode) -> usize {
        let mut d: usize = 0;
        let storage1 = self.bits.storage();
        let storage2 = other.bits.storage();
        for i in 0..min(storage1.len(), storage2.len()) {
            d += (storage1[i] ^ storage2[i]).count_ones() as usize;
        }
        d
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.bits.len()
    }

    pub fn multi_index_values(&self, mut bits_per_index: usize) -> Vec<u64> {
        if bits_per_index < 1 { bits_per_index = 1 };
        // Calculate number of indexes.
        let num_bits = self.len();
        let num_indexes = get_num_indexes(num_bits, bits_per_index);
        // Iterate over bits setting index values.
        let mut index_values: Vec<u64> = vec![0; num_indexes];
        for i in 0..num_bits {
            let index_num = i / bits_per_index;
            let position_num = i % bits_per_index;
            if self.get(i).unwrap() { index_values[index_num] |= 1 << position_num } ;
        }
        index_values
    }

    #[inline]
    pub fn set(&mut self, bit_number: usize, value: bool) {
        self.bits.set(bit_number, value);
    }
}


#[cfg(test)]
mod tests {
    use super::BitCode;
    use utils::random_bit_string;

    #[test]
    fn new() {
        let bc = BitCode::new(2);
        assert_eq!(bc.len(), 2);
        let bc = BitCode::new(64);
        assert_eq!(bc.len(), 64);
        let bc = BitCode::new(100);
        assert_eq!(bc.len(), 100);
    }

    #[test]
    fn set_get() {
        let mut bc = BitCode::new(512);
        assert_eq!(bc.len(), 512);
        assert_eq!(bc.count_ones(), 0);
        bc.set(10, true);
        bc.set(20, true);
        bc.set(30, true);
        assert_eq!(bc.count_ones(), 3);
        assert_eq!(bc.get(10), Some(true));
        assert_eq!(bc.get(20), Some(true));
        assert_eq!(bc.get(30), Some(true));
        bc.set(10, false);
        assert_eq!(bc.get(10), Some(false));
        assert_eq!(bc.count_ones(), 2);
    }

    #[test]
    fn multi_index_values() {
        let bc = BitCode::from_bit_string("010101010101");
        let keys = bc.multi_index_values(4);
        assert_eq!(keys.len(), 3);
        let keys = bc.multi_index_values(10);
        assert_eq!(keys.len(), 2);
    }

    #[test]
    fn new_bit_code_from_bool_string() {
        let bc = BitCode::from_bit_string("010101010101");
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
        let bc1 = BitCode::from_bit_string("010101010101");
        let bc2 = BitCode::from_bools(&vec![false, true, false, true, false, true, false, true, false, true, false, true]);
        assert_eq!(bc1.hamming_distance(&bc2), 0);
    }

    #[test]
    fn new_bit_code_from_random_string() {
        let bc = BitCode::from_bit_string(&random_bit_string(256));
        assert_eq!(bc.len(), 256);
    }

    #[test]
    fn hamming_distance() {
        let bc1 = BitCode::from_bit_string("010101010101");
        assert_eq!(bc1.hamming_distance(&bc1), 0);
        let bc2 = BitCode::from_bit_string("101010101010");
        assert_eq!(bc2.hamming_distance(&bc2), 0);
        assert_eq!(bc1.hamming_distance(&bc2), 12);
    }
}
