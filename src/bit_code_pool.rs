use bit_code::BitCode;
use bit_code_index::BitCodeIndex;
use encoders::string_to_bit_code_no_allocation;
use random_projections::RandomProjections;
use std::collections::HashSet;
use utils::{get_num_indexes, num_blocks_needed, FastHasher};


#[derive(Debug)]
pub struct BitCodePool {
    bit_codes: Vec<BitCode>,                // Bit codes in the pool.
    bools: Vec<bool>,                       // Bool storage to compute bit codes.
    features: Vec<f64>,                     // Feature storage to compute bit codes.
    ids: Vec<u64>,                          // Identifiers associated with bit codes (e.g. primary keys in database representation).
    index: BitCodeIndex,                    // Multi-index to enable sublinear-time searching.
    num_bits: usize,                        // Number of bits in bit codes.
    num_blocks: usize,                      // Number of u64 blocks in bit codes.
    random_projections: RandomProjections,  // Random projections used to convert features to bits.
}


impl BitCodePool {
    pub fn new(num_features: usize, mut num_bits: usize) -> Self {
        if num_bits == 0 { num_bits = 64 };
        BitCodePool {
            bit_codes: Vec::new(),
            bools: vec![false; num_bits],
            features: vec![0.0; num_features],
            ids: Vec::new(),
            index: BitCodeIndex::new(),
            num_bits: num_bits,
            num_blocks: num_blocks_needed(num_bits),
            random_projections: RandomProjections::new(num_features, num_bits),
        }
    }

    // Add a bit code created from a string to the pool.
    pub fn add(&mut self, string: &str, id: u64) {
        let bit_code = string_to_bit_code_no_allocation(
                &string,
                &self.random_projections,
                &mut self.features,
                &mut self.bools,
        );
        self.bit_codes.push(bit_code);
        self.ids.push(id);
    }

    // Get a BitCode from the pool.
    pub fn get(&self, i: usize) -> Option<&BitCode> {
        if i < self.len() {
            return Some(&self.bit_codes[i]);
        }
        return None;
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.bit_codes.len()
    }

    #[inline]
    pub fn index_max_searchable_radius(&self) -> usize {
        self.index.max_searchable_radius()
    }

    // TODO: Figure out a way to expire the index when new bit codes added?
    // Set multi-index on the bit codes currently in the pool.
    pub fn index(&mut self, mut bits_per_index: usize) {
        if bits_per_index == 0 { bits_per_index = 1; }
        if bits_per_index > self.num_bits { bits_per_index = self.num_bits }
        // Number of indexes.
        let num_indexes = get_num_indexes(self.num_bits, bits_per_index);
        // Construct index.
        self.index.init(bits_per_index, num_indexes);
        for (i, bit_code) in self.bit_codes.iter().enumerate() {
            let index_values = bit_code.multi_index_values(bits_per_index);
            self.index.add(&index_values, i);
        }
    }

    pub fn index_show(&self) {
        println!("{:?}", self.index);
    }

    pub fn resolve_entities(&self, radius: u32) -> Vec<Vec<usize>> {
        // Initialize indices of bit codes to search through.
        let mut population: HashSet<usize, FastHasher> = HashSet::with_capacity_and_hasher(self.len(), FastHasher::default());
        for i in 0..self.len() { population.insert(i); }
        // Compute entity sets.
        let mut entity_sets: Vec<Vec<usize>> = Vec::new();
        while !population.is_empty() {
            let mut entity_set: Vec<usize> = Vec::new();
            let i = population.iter().next().unwrap().clone();
            for j in &population {
                if self.bit_codes[i].hamming_distance(&self.bit_codes[*j]) <= radius { entity_set.push(*j); }
            }
            for i in &entity_set { population.remove(i); }
            entity_sets.push(entity_set);
        }
        entity_sets
    }

    /// Returns the indices of bit codes with Hamming distance <= radius from the needle.
    pub fn search(&self, needle: &BitCode, radius: u32) -> Vec<usize> {
        let mut indices: Vec<usize> = Vec::new();
        for i in 0..self.bit_codes.len() {
            if self.bit_codes[i].hamming_distance(&needle) <= radius { indices.push(i); }
        }
        indices
    }

    /// Returns the k nearest neighbors of the needle.
    pub fn search_knn(&self, needle: &BitCode, k: usize) -> Vec<SearchResult> {
        let mut results: Vec<SearchResult> = Vec::with_capacity(k);
        for idx in 0..self.bit_codes.len() {
            let d = self.bit_codes[idx].hamming_distance(&needle);
            let n = results.len();
            if (n == 0) || (d <= results[n - 1].distance) {
                results.push(SearchResult{ idx: idx, distance: d });
                // Sort result by ascending distance, and truncate to length k without losing any entries that are the same distance from the needle as the kth (these would be arbitrary exclusions).
                results.sort_by_key(|key| key.by_distance());
                if results.len() > k {
                    let distance_threshold = results[k - 1].distance;
                    results.retain(|sr| sr.distance <= distance_threshold);
                }
            }
        }
        results
    }

    /// Returns the indices of bit codes with Hamming distance <= radius from the needle using indexed search.
    pub fn search_with_index(&self, needle: &BitCode, radius: u32) -> Option<Vec<usize>> {
        // Check index is valid for search.
        if (radius as usize) > self.index.max_searchable_radius() { return None; };
        // Perform index search.
        let needle_index_values = needle.multi_index_values(self.index.bits_per_index());
        let candidate_indices = &self.index.candidate_indices(&needle_index_values);
        let mut indices: Vec<usize> = Vec::new();
        for c in candidate_indices {
            if self.bit_codes[*c].hamming_distance(&needle) <= radius { indices.push(*c); }
        }
        Some(indices)
    }
}


pub struct SearchResult {
    idx: usize,
    distance: u32,
}

impl SearchResult {
    pub fn by_distance(&self) -> u32 {
        self.distance
    }

    pub fn distance(&self) -> u32 { self.distance }

    pub fn idx(&self) -> usize { self.idx }
}

#[cfg(test)]
mod tests {
    use super::BitCodePool;
    use utils::random_string;

    #[test]
    fn index_search() {
        let mut bit_code_pool = BitCodePool::new(5, 256);
        for id in 0..1_000 {
            let string = random_string(3);
            bit_code_pool.add(&string, id);
        }
        bit_code_pool.index(8);
        let needle = bit_code_pool.get(0).unwrap();
        let hamming_radius = 31;
        let mut ids1 = bit_code_pool.search(needle, hamming_radius);
        let mut ids2 = bit_code_pool.search_with_index(needle, hamming_radius).unwrap();
        // Confirm results of unindexed and indexed search are the same.
        assert_eq!(ids1.len(), ids2.len());
        assert!(ids1.len() > 0);
        ids1.sort();
        ids2.sort();
        for i in 0..ids1.len() {
            assert_eq!(ids1[i], ids2[i]);
        }
    }

    #[test]
    fn new_bit_code_pool() {
        // Parameters.
        let num_bits = 256;
        let num_features = 500;
        let num_bit_codes: usize = 100;
        // Make a bit code pool.
        let mut bit_code_pool = BitCodePool::new(num_features, num_bits);
        for id in 0..(num_bit_codes as u64) {
            let string = random_string(10);
            bit_code_pool.add(&string, id);
        }
        // Test.
        assert_eq!(bit_code_pool.num_bits, num_bits);
        assert_eq!(bit_code_pool.bools.len(), num_bits);
        assert_eq!(bit_code_pool.features.len(), num_features);
        assert_eq!(bit_code_pool.bit_codes.len(), num_bit_codes);
    }

    #[test]
    fn resolve_entities() {
        // Make a bit code pool.
        let mut bit_code_pool = BitCodePool::new(100, 256);
        for id in 0..1_000 {
            let string = random_string(10);
            bit_code_pool.add(&string, id);
        }
        // Resolve the entities.
        let entity_sets = bit_code_pool.resolve_entities(10);
        // Each entity set should have at least 1 member.
        let mut num_entities = 0;
        for entity_set in &entity_sets {
            assert!(entity_set.len() >= 1);
            num_entities += entity_set.len();
        }
        // Number of resolved entities should equalnumber of bit codes in pool.
        assert_eq!(num_entities, bit_code_pool.len());
    }
}
