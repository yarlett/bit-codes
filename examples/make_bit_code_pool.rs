extern crate bit_codes;

fn main() {
    // Initialize bit code pool.
    let mut bit_code_pool = bit_codes::bit_code_pool::BitCodePool::new(1_000, 256);
    // Insert some bit codes into the pool.
    for id in 0..10_000 {
        let string = bit_codes::utils::random_string(100);
        bit_code_pool.add(&string, id);
    }
    // Index the bit pool using multi-index hashing.
    bit_code_pool.index();
    println!("{:?}", bit_code_pool.index_show());
}
