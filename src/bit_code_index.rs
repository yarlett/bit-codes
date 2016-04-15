use std::collections::{HashMap, HashSet};


#[derive(Debug)]
pub struct BitCodeIndex {
    indexes: Vec<HashMap<u64, HashSet<usize>>>
}


impl BitCodeIndex {
    pub fn new() -> Self {
        BitCodeIndex { indexes: Vec::new() }
    }

    // Add multi-index values to the index.
    pub fn add(&mut self, index_values: &Vec<u64>, value: usize) {
        for (i, k) in index_values.iter().enumerate() {
            if !self.indexes[i].contains_key(k) {
                self.indexes[i].insert(*k, HashSet::new());
            }
            self.indexes[i].get_mut(k).unwrap().insert(value);
        }
    }

    pub fn candidates(&self, needle_index_values: &Vec<u64>) -> HashSet<usize> {
        let mut candidates = HashSet::new();
        for (i, index_value) in needle_index_values.iter().enumerate() {
            if self.indexes[i].contains_key(index_value) {
                let entries = self.indexes[i].get(index_value).unwrap();
                for entry in entries {
                    candidates.insert(*entry);
                }
            }
        }
        candidates
    }

    pub fn init(&mut self, num_indexes: usize) {
        self.indexes = Vec::with_capacity(num_indexes);
        for _ in 0..num_indexes {
            self.indexes.push(HashMap::new());
        }
    }

    pub fn len(&self) -> usize {
        self.indexes.len()
    }

    pub fn max_searchable_radius(&self) -> usize {
        self.len() - 1
    }
}
