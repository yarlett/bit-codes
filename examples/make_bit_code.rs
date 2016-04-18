extern crate bit_codes;

fn main() {
    let string = "Supercalifragilisticexpialidocious";
    let num_features = 500;
    let num_bits = 256;
    let random_projs = bit_codes::random_projections::RandomProjections::new(num_features, num_bits);
    let bit_code = bit_codes::encoders::string_to_bit_code(&string, &random_projs).unwrap();
    println!("{:?}", bit_code);
}
