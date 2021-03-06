extern crate bit_codes;

fn main() {
    // Parameters.
    let encoding_options = bit_codes::encoding_options::EncodingOptions::default();
    let num_items = 10_000;
    let string_length = 25;
    // Initialize bit code pool.
    let mut bit_code_pool = bit_codes::bit_code_pool::BitCodePool::new(encoding_options);
    // Insert some bit codes into the pool.
    for id in 0..num_items {
        let string = bit_codes::utils::random_string(string_length);
        bit_code_pool.add(&string, id);
    }
}
