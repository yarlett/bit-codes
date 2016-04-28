use std::collections::{HashMap, HashSet};
use utils::FastHasher;


#[derive(Debug)]
pub struct BitCodeIndex {
    bits_per_index: usize,
    indexes: Vec<HashMap<u64, HashSet<usize, FastHasher>, FastHasher>>,
}


impl BitCodeIndex {
    pub fn new() -> Self {
        BitCodeIndex { bits_per_index: 0, indexes: Vec::new() }
    }

    // Add multi-index values to the index.
    pub fn add(&mut self, index_values: &Vec<u64>, value: usize) {
        for (i, k) in index_values.iter().enumerate() {
            if !self.indexes[i].contains_key(k) {
                let hashset: HashSet<usize, FastHasher> = HashSet::default();
                self.indexes[i].insert(*k, hashset);
            }
            self.indexes[i].get_mut(k).unwrap().insert(value);
        }
    }

    pub fn bits_per_index(&self) -> usize {
        self.bits_per_index
    }

    pub fn candidate_indices(&self, needle_index_values: &Vec<u64>) -> HashSet<usize, FastHasher> {
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

    pub fn init(&mut self, bits_per_index: usize, num_indexes: usize) {
        self.bits_per_index = bits_per_index;
        self.indexes = Vec::with_capacity(num_indexes);
        for _ in 0..num_indexes {
            let hashmap: HashMap<u64, HashSet<usize, FastHasher>, FastHasher> = HashMap::default();
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
