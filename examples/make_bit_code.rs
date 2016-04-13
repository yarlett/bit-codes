extern crate bit_codes;

fn main() {
    let string = "Supercalifragilisticexpialidocious";
    let num_features = 500;
    let num_bits = 256;
    let random_projs = bit_codes::random_projections::get_random_projections(num_features, num_bits);
    let bit_code = bit_codes::encoders::string_to_bit_code_via_feature_vector(&string, &random_projs);
    println!("{:?}", bit_code);
}