// use bit_code::BitCode;
use std::collections::HashMap;


#[derive(Debug)]
pub struct BitCodeIndex {
    indexes: Vec<HashMap<u64, Vec<usize>>>,
}


impl BitCodeIndex {
    pub fn new(num_indexes: usize) -> Self {
        let mut indexes: Vec<HashMap<u64, Vec<usize>>> = Vec::with_capacity(num_indexes);
        for _ in 0..num_indexes {
            indexes.push(HashMap::new());
        }
        BitCodeIndex {
            indexes: indexes,
        }
    }

    pub fn add(&mut self, keys: &Vec<u64>, value: usize) {
        for (i, k) in keys.iter().enumerate() {
            if !self.indexes[i].contains_key(k) {
                self.indexes[i].insert(*k, Vec::new());
            }
            match self.indexes[i].get_mut(k) {
                Some(x) => x.push(value),
                None => (),
            }
        }
    }
}
