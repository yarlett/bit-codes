use bit_code::BitCode;
use fnv::FnvHasher;
use random_projections::RandomProjections;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use utils::FastHasher;


/// Encodes an input string as a BitCode.
pub fn string_to_bit_code(string: &str, random_projections: &RandomProjections) -> BitCode {
    // Make allocations.
    let mut features: Vec<f64> = vec![0.0; random_projections.dim_in()];
    let mut bools: Vec<bool> = vec![false; random_projections.dim_out()];
    // Make and return bit code.
    string_to_bit_code_no_allocation(string, random_projections, &mut features, &mut bools)
}


/// Encodes an input string as a BitCode without unnecessary memory allocation.
pub fn string_to_bit_code_no_allocation(string: &str, random_projections: &RandomProjections, features: &mut Vec<f64>, bools: &mut Vec<bool>) -> BitCode {
    // Set feature vector.
    random_projections.set_feature_vector(string, features);
    // Set bools via random projections of the features.
    random_projections.set_bool_vector(features, bools);
    // Make and return the bit code.
    BitCode::from_bools(bools)
}


pub fn string_to_bit_code_fast_encoder(
    string: &str,
    downcase: bool,
    ngram_lengths: &Vec<usize>,
    random_projections: &RandomProjections
    ) -> BitCode {
    let nb = random_projections.dim_out();
    let nd = random_projections.dim_in();
    // Get characters.
    let chars: Vec<char> = match downcase {
        true => string.to_lowercase().chars().collect(),
        false => string.chars().collect(),
    };
    let nc = chars.len();
    // Iterate over string features to get frequencies of features.
    // Values are stored sparsely in a hashmap.
    let mut features: HashMap<usize, f64, FastHasher> = HashMap::default();
    for l in ngram_lengths {
        if l <= &nc {
            for pos in 0..(nc - l + 1) {
                // Get the character ngram.
                let ngram = &chars[pos..(pos + l)];
                // Compute the feature value for the ngram via the hashing trick.
                let mut hasher = FnvHasher::default();
                ngram.hash(&mut hasher);
                let feature = (hasher.finish() as usize) % nd;
                //
                let v = features.entry(feature).or_insert(0.0);
                *v += 1.0;
            }
        }
    }
    // Compute bits via random projections.
    let mut bitcode = BitCode::new(nb);
    for i in 0..nb {
        let mut acc: f64 = 0.0;
        for (f, w) in features.iter() {
            acc += random_projections.vectors[i][*f as usize] * w;
        }
        if acc > 0.0 { bitcode.set(i, true); } else { bitcode.set(i, false); };
    }
    bitcode
}


#[cfg(test)]
mod tests {
    use random_projections::{RandomProjections};
    use super::string_to_bit_code;
    use test::Bencher;
    use utils::random_string;


    /// Deriving bit code from a string on 2 occasions should yield the same bit code.
    #[test]
    fn test_string_to_bit_code() {
        let string = "Supercalifragilisticexpialidocious";
        let ngram_lengths = vec![3, 4, 5, 6, 7, 8];
        let num_features = 500;
        let num_bits = 256;
        let random_projs = RandomProjections::new(num_features, num_bits, ngram_lengths);
        let bit_code_1 = string_to_bit_code(&string, &random_projs);
        let bit_code_2 = string_to_bit_code(&string, &random_projs);
        println!("{:?}", bit_code_1);
        println!("{:?}", bit_code_2);
        assert_eq!(bit_code_1.hamming_distance(&bit_code_2), 0);
    }

    #[bench]
    fn encode_string(b: &mut Bencher) {
        // Generate random string.
        let ngram_lengths = vec![3, 4, 5, 6, 7, 8];
        let random_string = random_string(100);
        let random_projs = RandomProjections::new(500, 256, ngram_lengths);
        // Benchmark time to encode the strings as bit codes.
        b.iter(|| {
            string_to_bit_code(&random_string, &random_projs)
        });
    }
}
