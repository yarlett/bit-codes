extern crate bit_codes;

fn main() {
    // Parameters.
    let bits_per_index = 8;
    let num_bits = 256;
    let num_features = 1_000;
    let num_items = 10_000;
    let string_length = 25;
    // Initialize bit code pool.
    let mut bit_code_pool = bit_codes::bit_code_pool::BitCodePool::new(num_features, num_bits);
    // Insert some bit codes into the pool.
    for id in 0..num_items {
        let string = bit_codes::utils::random_string(string_length);
        bit_code_pool.add(&string, id);
    }
    // Index the bit pool using multi-index hashing.
    bit_code_pool.index(bits_per_index);
    println!("{:?}", bit_code_pool.index_show());
}
