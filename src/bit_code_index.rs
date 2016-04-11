pub struct BitCodeIndex {
    num_indexes: usize,
}


impl BitCodeIndex {
    pub fn new(num_indexes: usize) -> Self {
        BitCodeIndex{ num_indexes: num_indexes }
    }
}
