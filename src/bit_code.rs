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

    pub fn index_values(&self, mut index_length: usize) -> Vec<BitVec> {
        let num_bits = self.len();
        // Ensure bits per index is within required range.
        if index_length < 1 { index_length = 1; };
        if index_length > num_bits { index_length = num_bits; };
        // Compute number of indexes required.
        let num_indexes = get_num_indexes(num_bits, index_length);
        // Set index values.
        let mut index_values = Vec::new();
        for i in 0..num_indexes {
            let mut index_value = BitVec::from_elem(index_length, false);
            for j in (i * index_length)..((i + 1) * index_length) {
                if j < num_bits {
                    index_value.set(j - (i * index_length), self.get(j).unwrap());
                }
            }
            index_values.push(index_value)
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
    use bit_vec::BitVec;
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
    fn index_values() {
        // Test that index values are what they should be.
        let bools = vec![true, false, true, false, true, false, true, false, true, false, true, true];
        let bc = BitCode::from_bools(&bools);
        let ivs1 = bc.index_values(4);
        let ivs1_correct = vec![
            BitVec::from_fn(4, |i| { vec![true, false, true, false][i] }),
            BitVec::from_fn(4, |i| { vec![true, false, true, false][i] }),
            BitVec::from_fn(4, |i| { vec![true, false, true, true][i] }),
        ];
        assert_eq!(ivs1.len(), ivs1_correct.len());
        assert_eq!(ivs1, ivs1_correct);
        let ivs2 = bc.index_values(10);
        let ivs2_correct = vec![
            BitVec::from_fn(10, |i| { vec![true, false, true, false, true, false, true, false, true, false][i] }),
            BitVec::from_fn(10, |i| { vec![true, true, false, false, false, false, false, false, false, false][i] }),
        ];
        assert_eq!(ivs2.len(), ivs2_correct.len());
        assert_eq!(ivs2, ivs2_correct);
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
