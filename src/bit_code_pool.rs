use bit_code::BitCode;
use bit_code_index::BitCodeIndex;


pub struct BitCodePool {
    bit_codes: Vec<BitCode>,       // Bit codes in the pool.
    bit_length: usize,             // Length of the bit codes.
    ids: Vec<u64>,                 // Identifiers associated with bit codes (e.g. primary keys in database representation).
    index: Option<BitCodeIndex>,   // Optional multiindex to power sublinear time searching.
}


impl BitCodePool {
    pub fn new(bit_length: usize) -> Self {
        // TODO: Ensure bit_length is an exact multiple of 64?
        // TODO: Store IDs in BitCodes themselves?
        // TODO: Configure a BitCodePool with random projections?
        BitCodePool {
            bit_codes: Vec::new(),
            bit_length: bit_length,
            ids: Vec::new(),
            index: None,
        }
    }

    // TODO: Ensure that all bit codes in the pool have the same bit_length? Do this by taking strings in here and mapping them to a common bit code format using random projections.
    pub fn add(&mut self, bit_code: BitCode, id: u64) {
        self.bit_codes.push(bit_code);
        self.ids.push(id);
    }

    // Set multi-index on the bit codes currently in the pool.
    // TODO: Figure out a way to expire the index when new bit codes added?
    pub fn index(&mut self, mut bits_per_index: usize) {
        // Number of hash indexes needed by multi-index.
        let num_blocks = (self.bit_length) / 64;
        let num_indexes = num_blocks * (64 / bits_per_index);
        // Create the index.
        let index = BitCodeIndex::new(num_indexes);
        for bit_code in &self.bit_codes {
            let keys = bit_code.multi_index_values(bits_per_index);
        }
        self.index = Some(index);
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

    pub fn search_with_index(&self, needle: &BitCode, radius: u32) -> Vec<u64> {
        let mut ids: Vec<u64> = Vec::new();
        ids
    }
}


#[cfg(test)]
mod tests {
    use bit_code::BitCode;
    use super::BitCodePool;
    use test::Bencher;
    use utils::random_bit_string;

    #[bench]
    fn new_bit_code_pool_1000_by_256(b: &mut Bencher) {
        // Benchmark time to create a bit pool of 1000 256-bit codes.
        b.iter(|| {
            let mut bit_code_pool = BitCodePool::new(256);
            for id in 0..1_000 {
                let bit_code = BitCode::from_string(&random_bit_string(256));
                bit_code_pool.add(bit_code, id);
            }
            bit_code_pool
        });
    }

    #[bench]
    fn search_bit_code_pool_1000_by_256(b: &mut Bencher) {
        // Create a bit pool of 1000 256 bit codes.
        let mut bit_code_pool = BitCodePool::new(256);
        for id in 0..1_000 {
            let bit_string = random_bit_string(256);
            let bit_code = BitCode::from_string(&bit_string);
            bit_code_pool.add(bit_code, id);
        }
        //Select a needle to look for in the haystack.
        let needle = &bit_code_pool.bit_codes[0];
        // Benchmark the time to search for the needle.
        b.iter(|| { bit_code_pool.search(needle, 10) });
    }
}
