use bit_code::BitCode;
use bit_code_index::BitCodeIndex;


pub struct BitCodePool {
    bit_codes: Vec<BitCode>,       // Bit codes in the pool.
    bit_length: usize,             // Length of bit codes in the pool.
    ids: Vec<u64>,                 // Identifiers associated with bit codes (e.g. primary keys in database representation).
    index: Option<BitCodeIndex>,   // Optional multiindex to power sublinear time searching.
}


impl BitCodePool {
    pub fn new(bit_length: usize) -> Self {
        let bit_codes = Vec::new();
        let ids = Vec::new();
        BitCodePool {
            bit_codes: bit_codes,
            bit_length: bit_length,
            ids: ids,
            index: None,
        }
    }

    pub fn add(&mut self, bit_code: BitCode, id: u64) {
        self.bit_codes.push(bit_code);
        self.ids.push(id);
    }

    pub fn index(&mut self, mut bit_length: usize) {
        // Ensure index bit length is less than block size and divides block size exactly.
        if (bit_length == 0) || (bit_length > 64) || ((64 % bit_length) != 0) {
            bit_length = 8;
        }
        // Number of hash indexes needed by multi-index.
        let num_blocks = (self.bit_length) / 64;
        let num_indexes = num_blocks * (64 / bit_length);
        // Create the index.
        let index = BitCodeIndex::new(num_indexes);

        for bit_code in &self.bit_codes {
            let keys = bit_code.multi_index_keys(bit_length);
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
