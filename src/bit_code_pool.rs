use bit_code::BitCode;


pub struct BitCodePool {
    bit_codes: Vec<BitCode>,
}


impl BitCodePool {
    pub fn new() -> BitCodePool {
        let bit_codes = Vec::new();
        BitCodePool { bit_codes: bit_codes }
    }

    pub fn push(&mut self, bit_code: BitCode) {
        self.bit_codes.push(bit_code);
    }

    /*
    Returns the indices of bit codes with Hamming distance <= radius from the needle.
    */
    pub fn search(&self, needle: &BitCode, radius: usize) -> Vec<usize> {
        let mut indices: Vec<usize> = Vec::new();
        for i in 0..self.bit_codes.len() {
            let d = self.bit_codes[i].hamming_distance(&needle);
            if d <= radius {
                indices.push(i);
            }
        }
        indices
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
            let mut bit_code_pool = BitCodePool::new();
            for _ in 0..1_000 {
                let bit_code = BitCode::from_bit_string(&random_bit_string(256));
                bit_code_pool.push(bit_code);
            }
            bit_code_pool
        });
    }

    #[bench]
    fn search_bit_code_pool_1000_by_256(b: &mut Bencher) {
        // Create a bit pool of 1000 256 bit codes.
        let mut bit_code_pool = BitCodePool::new();
        for _ in 0..1_000 {
            let bit_string = random_bit_string(256);
            let bit_code = BitCode::from_bit_string(&bit_string);
            bit_code_pool.push(bit_code);
        }
        //Select a needle to look for in the haystack.
        let needle = &bit_code_pool.bit_codes[0];
        // Benchmark the time to search for the needle.
        b.iter(|| { bit_code_pool.search(needle, 10) });
    }
}
