extern crate bit_codes;
extern crate rand;
extern crate time;

use rand::distributions::IndependentSample;

fn main() {
    // Parameters.
    let bits_per_index = 10;
    let ngram_lengths = vec![3, 4, 5, 6, 7, 8];
    let num_bits = 256;
    let num_features = 1_000;
    let num_items = 1_000_000;
    let num_needles = 1_000;
    let string_length = 5;
    // Create random strings.
    let t1 = time::precise_time_s();
    let mut strings: Vec<String> = Vec::new();
    for _ in 0..num_items { strings.push(bit_codes::utils::random_string(string_length)); }
    let t2 = time::precise_time_s();
    let t_s = format!("{:.*}", 3, t2 - t1);
    println!("{:} random strings created in {:}s.", num_items, t_s);
    // Create bit code pool from random strings.
    let t1 = time::precise_time_s();
    let mut bit_code_pool = bit_codes::bit_code_pool::BitCodePool::new(num_features, num_bits, ngram_lengths);
    for i in 0..strings.len() { bit_code_pool.add(&strings[i], i as u64); }
    let t2 = time::precise_time_s();
    let t_s = format!("{:.*}", 3, t2 - t1);
    println!("{:} bit codes inserted into pool in {:}s.", num_items, t_s);
    // Create index on the bit code pool.
    let t1 = time::precise_time_s();
    bit_code_pool.index(bits_per_index);
    let max_radius = bit_code_pool.index_max_searchable_radius() as u32;
    let t2 = time::precise_time_s();
    let t_s = format!("{:.*}", 3, t2 - t1);
    println!("Bit code index created in {:}s (max searchable radius is {:}).", t_s, max_radius);
    // Search unindexed patterns.
    let mut rng = rand::thread_rng();
    let between = rand::distributions::Range::new(0, bit_code_pool.len());
    let t1 = time::precise_time_s();
    for _ in 0..num_needles {
        let i = between.ind_sample(&mut rng);
        let _ = bit_code_pool.search(bit_code_pool.get(i).unwrap(), max_radius);
    }
    let t2 = time::precise_time_s();
    let t_s = format!("{:.*}", 3, t2 - t1);
    println!("Unindexed search for {:} items took {:}s.", num_needles, t_s);
    // Search indexed patterns.
    let t1 = time::precise_time_s();
    for _ in 0..num_needles {
        let i = between.ind_sample(&mut rng);
        let _ = bit_code_pool.search_with_index(bit_code_pool.get(i).unwrap(), max_radius);
    }
    let t2 = time::precise_time_s();
    let t_s = format!("{:.*}", 3, t2 - t1);
    println!("Indexed search for {:} items took {:}s.", num_needles, t_s);
}
