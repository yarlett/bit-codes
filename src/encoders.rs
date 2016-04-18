use bit_code::BitCode;
use random_projections::RandomProjections;
use string_features::StringFeatures;


/// Encodes an input string as a BitCode.
pub fn string_to_bit_code(string: &str, random_projections: &RandomProjections) -> Option<BitCode> {
    // Get feature vector from string.
    let feature_vector: Vec<f64> = string_to_feature_vector(
        string,
        random_projections.dim_in(),
    );
    // Compute bools from feature vector via random projections.
    let bools = random_projections.project(feature_vector);
    match bools {
        Some(b) => {
            let mut bit_code = BitCode::new(b.len());
            for i in 0..b.len() { bit_code.set(i, b[i]); };
            return Some(bit_code);
        },
        None => return None,
    }
}


/// Encodes an input string as a feature vector using the hashing trick.
pub fn string_to_feature_vector(string: &str, dim: usize) -> Vec<f64> {
    let mut vector: Vec<f64> = vec![0.0; dim];
    for hash_value in StringFeatures::default(&string) {
        let vector_bin = hash_value % dim;
        if (hash_value as i64) > 0 {
            vector[vector_bin] += 1.0;
        } else {
            vector[vector_bin] -= 1.0;
        }
    }
    vector
}


#[cfg(test)]
mod tests {
    use random_projections::{RandomProjections};
    use super::{string_to_bit_code, string_to_feature_vector};
    use test::Bencher;
    use utils::random_string;


    /// Deriving bit code from a string on 2 occasions should yield the same bit code.
    #[test]
    fn test_string_to_bit_code() {
        let string = "Supercalifragilisticexpialidocious";
        let num_features = 500;
        let num_bits = 256;
        let random_projs = RandomProjections::new(num_features, num_bits);
        let bit_code_1 = string_to_bit_code(&string, &random_projs).unwrap();
        let bit_code_2 = string_to_bit_code(&string, &random_projs).unwrap();
        println!("{:?}", bit_code_1);
        println!("{:?}", bit_code_2);
        assert_eq!(bit_code_1.hamming_distance(&bit_code_2), 0);
    }

    #[test]
    fn test_string_to_feature_vector() {
        let result = string_to_feature_vector("abcdef", 5);
        let expected: Vec<f64> = vec![3.0, 2.0, 0.0, 4.0, 4.0];
        for i in 0..result.len() {
            assert_eq!(result[i], expected[i]);
        }
    }

    #[bench]
    fn encode_string(b: &mut Bencher) {
        // Generate random string.
        let random_string = random_string(100);
        let random_projs = RandomProjections::new(500, 256);
        // Benchmark time to encode the strings as bit codes.
        b.iter(|| {
            string_to_bit_code(&random_string, &random_projs)
        });
    }
}
