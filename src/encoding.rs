use bit_code::BitCode;
use fnv::FnvHasher;
use encoding_options::EncodingOptions;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use utils::FastHasher;


pub fn string_to_bit_code(string: &str, encoding_options: &EncodingOptions) -> BitCode {
    let nb = encoding_options.num_bits();
    let nd = encoding_options.num_features() as u64;
    // Get characters.
    let chars: Vec<char> = match encoding_options.downcase() {
        true => string.to_lowercase().chars().collect(),
        false => string.chars().collect(),
    };
    let nc = chars.len();
    // Iterate over string features to get frequencies of features. Values are stored sparsely in a hashmap.
    let mut features: HashMap<usize, f64, FastHasher> = HashMap::default();
    for l in encoding_options.ngram_lengths() {
        if l <= &nc {
            for pos in 0..(nc - l + 1) {
                let ngram = &chars[pos..(pos + l)];
                // Compute the feature value for the ngram via the hashing trick.
                let mut hasher = FnvHasher::default();
                ngram.hash(&mut hasher);
                let feature = hasher.finish() % nd;
                // Update the feature frequencies.
                let v = features.entry(feature as usize).or_insert(0.0);
                *v += 1.0;
            }
        }
    }
    // Compute bits via random projections.
    let mut bitcode = BitCode::new(nb);
    for b in 0..nb {
        bitcode.set(b, encoding_options.project(&features, b));
    }
    bitcode
}


#[cfg(test)]
mod tests {
    use super::string_to_bit_code;
    use encoding_options::EncodingOptions;
    use test::Bencher;
    use utils::random_string;


    /// Deriving bit code from a string on 2 occasions should yield the same bit code.
    #[test]
    fn test_string_to_bit_code() {
        let string = "Supercalifragilisticexpialidocious";
        let downcase = true;
        let ngram_lengths = vec![3, 4, 5, 6, 7, 8];
        let num_features = 500;
        let num_bits = 256;
        let encoding_options = EncodingOptions::new(downcase, ngram_lengths, num_bits, num_features);
        let bit_code_1 = string_to_bit_code(&string, &encoding_options);
        let bit_code_2 = string_to_bit_code(&string, &encoding_options);
        println!("{:?}", bit_code_1);
        println!("{:?}", bit_code_2);
        assert_eq!(bit_code_1.hamming_distance(&bit_code_2), 0);
    }

    #[bench]
    fn encode_string(b: &mut Bencher) {
        // Generate random string.
        let random_string = random_string(100);
        let encoding_options = EncodingOptions::default();
        // Benchmark time to encode the strings as bit codes.
        b.iter(|| {
            string_to_bit_code(&random_string, &encoding_options)
        });
    }
}
