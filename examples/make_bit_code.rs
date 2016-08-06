extern crate bit_codes;

fn main() {
    let string = "Supercalifragilisticexpialidocious";
    let downcase = true;
    let ngram_lengths = vec![3, 4, 5, 6];
    let num_bits = 256;
    let num_features = 500;
    let encoding_options = bit_codes::encoding_options::EncodingOptions::new(
        downcase,
        ngram_lengths,
        num_bits,
        num_features,
    );
    let bit_code = bit_codes::encoding::string_to_bit_code(&string, &encoding_options);
    println!("{:?}", bit_code);
}
