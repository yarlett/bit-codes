use bit_code::BitCode;


pub struct BitCodePool {
    codes: Vec<BitCode>,
}


impl BitCodePool {
    pub fn new() -> BitCodePool {
        let codes = Vec::new();
        BitCodePool { codes: codes }
    }
}
