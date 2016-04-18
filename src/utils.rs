use rand::{thread_rng, Rng};


#[inline]
pub fn num_blocks_needed(num_bits: usize) -> usize {
    let mut num_blocks = ((num_bits - 1) / 64) + 1;
    if num_blocks == 0 { num_blocks = 1 };
    num_blocks
}


pub fn random_bit_string(n: usize) -> String {
    let mut rng = thread_rng();
    let mut s = String::new();
    for _ in 0..n {
        if rng.gen() {
            s.push('1');
        } else {
            s.push('0');
        }
    }
    s
}


pub fn random_string(n: usize) -> String {
    thread_rng()
        .gen_ascii_chars()
        .take(n)
        .collect::<String>()
}
