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

    pub fn search(&self, needle: &BitCode, radius: u32) -> Vec<u64> {
        let mut keys: Vec<u64> = Vec::new();
        for code in &self.codes {
            let d = code.hamming_distance(&needle);
            if d <= radius {
                keys.push(code.key);
            }
        }
        keys
    }
}


#[cfg(test)]
mod tests {
    use bit_code::BitCode;
    use super::BitCodePool;
    use utils::random_bit_string;

    #[test]
    fn new_bit_code_pool() {
        let mut bcp = BitCodePool::new();
        for _ in 0..1000 {
            let bit_string = random_bit_string(256);
            let bit_code = BitCode::from_str(&bit_string, 0);
            bcp.push(bit_code);
        }

        for i in 0..bcp.codes.len() {
            let needle = &bcp.codes[i];
            bcp.search(needle, 5);
        }
    }
}
