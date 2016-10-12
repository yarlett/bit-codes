use bit_vec::BitVec;
use std::collections::{HashMap, HashSet};
use utils::FastHasher;


#[derive(Debug)]
pub struct BitCodeIndex {
    index_length: usize,
    indexes: Vec<HashMap<BitVec, HashSet<usize, FastHasher>, FastHasher>>,
}


impl BitCodeIndex {
    pub fn new() -> Self {
        BitCodeIndex { index_length: 0, indexes: Vec::new() }
    }

    // Add index values to the index.
    pub fn add(&mut self, index_values: &Vec<BitVec>, value: usize) {
        for (i, bv) in index_values.iter().enumerate() {
            if !self.indexes[i].contains_key(bv) {
                let hashset: HashSet<usize, FastHasher> = HashSet::default();
                self.indexes[i].insert(bv.clone(), hashset);
            }
            self.indexes[i].get_mut(bv).unwrap().insert(value);
        }
    }

    pub fn index_length(&self) -> usize {
        self.index_length
    }

    pub fn candidate_indices(&self, needle_index_values: &Vec<BitVec>) -> HashSet<usize, FastHasher> {
        let mut candidates: HashSet<usize, FastHasher> = HashSet::default();
        for i in 0..self.len() {
            match self.indexes[i].get(&needle_index_values[i]) {
                Some(new_candidates) => {
                    for entry in new_candidates { candidates.insert(*entry); }
                },
                None => (),
            }
        }
        candidates
    }

    pub fn init(&mut self, index_length: usize, num_indexes: usize) {
        self.index_length = index_length;
        self.indexes = Vec::with_capacity(num_indexes);
        for _ in 0..num_indexes {
            let hashmap: HashMap<BitVec, HashSet<usize, FastHasher>, FastHasher> = HashMap::default();
            self.indexes.push(hashmap);
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.indexes.len()
    }

    #[inline]
    pub fn max_searchable_radius(&self) -> usize {
        self.len() - 1
    }
}
