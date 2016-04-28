extern crate bit_codes;
extern crate time;

fn main() {
    // Parameters.
    let num_bits = 256;
    let num_features = 100;
    let num_items = 10_000;
    let radius = 50;
    let string_length = 5;
    // Create random strings.
    let mut strings: Vec<String> = Vec::new();
    for _ in 0..num_items {
        strings.push(bit_codes::utils::random_string(string_length));
    }
    // Create bit code pool from random strings.
    let mut bit_code_pool = bit_codes::bit_code_pool::BitCodePool::new(num_features, num_bits);
    for i in 0..strings.len() { bit_code_pool.add(&strings[i], i as u64); }
    // Resolve entities in bit code pool.
    let t1 = time::precise_time_s();
    let entity_sets = bit_code_pool.resolve_entities(radius);
    let t2 = time::precise_time_s();
    let t_s = format!("{:.*}", 3, t2 - t1);
    for entity_set in &entity_sets {
        if entity_set.len() > 1 {
            for i in entity_set { println!("{:?}", strings[*i]); }
            println!("");
        }
    }
    println!("Resolved entities into {:} entity sets of bit code pool in {:}s.", entity_sets.len(), t_s);
}
