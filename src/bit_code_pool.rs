use bit_code::BitCode;
use bit_code_index::BitCodeIndex;
use encoders::string_to_bit_code_no_allocation;
use random_projections::RandomProjections;
use utils::num_blocks_needed;


#[derive(Debug)]
pub struct BitCodePool {
    bit_codes: Vec<BitCode>,                // Bit codes in the pool.
    bools: Vec<bool>,                       // Bool storage to compute bit codes.
    features: Vec<f64>,                     // Feature storage to compute bit codes.
    ids: Vec<u64>,                          // Identifiers associated with bit codes (e.g. primary keys in database representation).
    index: BitCodeIndex,                    // Optional multi-index to enable sublinear-time searching.
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
        let bit_code = string_to_bit_code_no_allocation(&string, &self.random_projections, &mut self.features, &mut self.bools);
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

    pub fn len(&self) -> usize {
        self.bit_codes.len()
    }

    // TODO: Figure out a way to expire the index when new bit codes added?
    // Set multi-index on the bit codes currently in the pool.
    pub fn index(&mut self) {
        let bits_per_index = 8;
        // Number of indexes.
        let mut num_indexes = 0;
        if self.bit_codes.len() > 0 {
            num_indexes = self.bit_codes[0].multi_index_values(bits_per_index).len();
        }
        // Construct index.
        self.index.init(num_indexes);
        for (i, bit_code) in self.bit_codes.iter().enumerate() {
            let index_values = bit_code.multi_index_values(bits_per_index);
            self.index.add(&index_values, i);
        }
    }

    pub fn index_show(self) {
        println!("{:?}", self.index);
    }

    /// Returns the ids of bit codes with Hamming distance <= radius from the needle.
    pub fn search(&self, needle: &BitCode, radius: u32) -> Vec<u64> {
        let mut ids: Vec<u64> = Vec::new();
        for i in 0..self.bit_codes.len() {
            let d = self.bit_codes[i].hamming_distance(&needle);
            if d <= radius {
                ids.push(self.ids[i]);
            }
        }
        ids
    }

    pub fn search_with_index(&self, needle: &BitCode, radius: u32) -> Option<Vec<u64>> {
        // Check index is valid for search.
        if (radius as usize) > self.index.max_searchable_radius() { return None };
        let bits_per_index = 8;
        let needle_index_values = needle.multi_index_values(bits_per_index);
        let cands = &self.index.candidates(&needle_index_values);
        let mut ids: Vec<u64> = Vec::new();
        for c in cands {
            let d = self.bit_codes[*c].hamming_distance(&needle);
            if d <= radius {
                ids.push(self.ids[*c]);
            }
        }
        Some(ids)
    }
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
        bit_code_pool.index();
        let needle = bit_code_pool.get(0).unwrap();
        let hamming_radius = 31;
        let mut ids1 = bit_code_pool.search(needle, hamming_radius);
        let mut ids2 = bit_code_pool.search_with_index(needle, hamming_radius).unwrap();
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
}
