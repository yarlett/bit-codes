use std::hash::{Hasher, SipHasher};


use bit_code::BitCode;
use string_features::StringFeatures;


/// Encodes an input string as a BitCode.
pub fn string_to_bit_code(string: &str, random_projections: &Vec<Vec<f64>>) -> BitCode {
    // Infer dimensions from random projections.
    let nd = random_projections[0].len();
    let nb = random_projections.len();
    // Convert string to feature vector.
    let feature_vector: Vec<f64> = string_to_feature_vector(string, nd);
    // Convert the feature vector to a Vec<bool> using random projections.
    let mut bit_vector: Vec<bool> = vec![false; nb];
    for (i, projection_vector) in random_projections.iter().enumerate() {
        let mut dotprod: f64 = 0.0;
        for j in 0..nd {
            dotprod += feature_vector[j] * projection_vector[j];
        }
        if dotprod > 0.0 {
            bit_vector[i] = true;
        }
    }
    // Create and return the BitCode.
    BitCode::from_bit_vector(bit_vector)
}


/// Encodes an input string as a feature vector using the hashing trick.
pub fn string_to_feature_vector(string: &str, dim: usize) -> Vec<f64> {
    let mut vector: Vec<f64> = vec![0.0; dim];
    for feature in StringFeatures::new(&string) {
        let mut hasher = SipHasher::new();
        let bytes = feature.as_bytes();
        hasher.write(bytes);
        let hash_value = hasher.finish();
        let vector_bin = (hash_value as usize) % (dim as usize);
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
    use random_projections;
    use std::hash::{Hasher, SipHasher};
    use super::{string_to_bit_code, string_to_feature_vector};
    use test::Bencher;
    use utils::random_string;

    #[test]
    fn rehash_gives_same() {
        let bytes: Vec<u8> = vec![12, 23, 34];
        let mut hasher = SipHasher::new();
        hasher.write(&bytes);
        let h1 = hasher.finish();
        let mut hasher = SipHasher::new();
        hasher.write(&bytes);
        let h2 = hasher.finish();
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_string_to_bit_code() {
        let s = "Supercalifragilisticexpialidocious";
        let nd = 500;
        let nb = 256;
        let rps = random_projections::get_random_projections(nd, nb);
        let bc1 = string_to_bit_code(s, &rps);
        let bc2 = string_to_bit_code(s, &rps);
        assert_eq!(bc1.hamming_distance(&bc2), 0);
    }

    #[test]
    fn test_string_to_feature_vector() {
        let result = string_to_feature_vector("abcdef", 5);
        let expected: Vec<f64> = vec![-1.0, -2.0, -2.0, 1.0, -3.0];
        for i in 0..result.len() {
            assert_eq!(result[i], expected[i]);
        }
    }

    #[bench]
    fn encode_string(b: &mut Bencher) {
        // Generate random string.
        let random_string = random_string(100);
        // Generate random projections vectors.
        let rps = random_projections::get_random_projections(500, 256);
        // Benchmark time to encode the strings as bit codes.
        b.iter(|| {
            string_to_bit_code(&random_string, &rps);
        });
    }
}
