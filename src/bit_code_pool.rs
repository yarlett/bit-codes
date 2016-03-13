use bit_code::BitCode;


pub struct BitCodePool {
    codes: Vec<BitCode>,
}


impl BitCodePool {
    pub fn new() -> BitCodePool {
        let codes = Vec::new();
        BitCodePool { codes: codes }
    }

    pub fn push(&mut self, bit_code: BitCode) {
        self.codes.push(bit_code);
    }

    pub fn search(&self, needle: &BitCode, radius: usize) -> Vec<usize> {
        let mut keys: Vec<usize> = Vec::new();
        for i in 0..self.codes.len() {
            let d = self.codes[i].hamming_distance(&needle);
            if d <= radius {
                keys.push(i);
            }
        }
        keys
    }
}


#[cfg(test)]
mod tests {
    use bit_code::BitCode;
    use super::BitCodePool;
    use test::Bencher;
    use utils::random_bit_string;

    #[bench]
    fn new_bit_code_pool(b: &mut Bencher) {
        // Benchmark time to create a bit pool of 1M 256-bit codes.
        b.iter(|| {
            let mut bcp = BitCodePool::new();
            for _ in 0..1_000_000 {
                let bit_string = random_bit_string(256);
                let bit_code = BitCode::from_str(&bit_string);
                bcp.push(bit_code);
            }
        });
    }

    #[bench]
    fn search_bit_code_pool(b: &mut Bencher) {
        // Create a bit pool of 1M 256 bit codes.
        let mut bcp = BitCodePool::new();
        for _ in 0..1_000_000 {
            let bit_string = random_bit_string(256);
            let bit_code = BitCode::from_str(&bit_string);
            bcp.push(bit_code);
        }
        // Benchmark the time to search for a pattern.
        let needle = &bcp.codes[0];
        b.iter(|| { bcp.search(needle, 10); });
    }
}
