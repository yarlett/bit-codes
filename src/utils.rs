use fnv::FnvHasher;
use rand::{thread_rng, Rng};
use std::hash::BuildHasherDefault;


// Custom hasher type using FNV algorithm (faster for small keys such as usize).
pub type FastHasher = BuildHasherDefault<FnvHasher>;


#[inline]
pub fn get_num_indexes(num_bits: usize, bits_per_index: usize) -> usize {
    ((num_bits - 1) / bits_per_index) + 1
}


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
