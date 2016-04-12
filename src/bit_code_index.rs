use bit_code::BitCode;
use std::collections::HashMap;


pub struct BitCodeIndex {
    num_indexes: usize,
    hashes: Vec<HashMap<u64, Vec<u64>>>,
}


impl BitCodeIndex {
    pub fn new(num_indexes: usize) -> Self {
        BitCodeIndex{
            hashes: Vec::new(),
            num_indexes: num_indexes,
        }
    }

    // pub fn add(bit_code: BitCode) {
    //     for v in bit_code.multi_index_values() {
    //     }
    // }
}
