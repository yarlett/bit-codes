use std::hash::{Hasher, SipHasher};


use string_features::StringFeatures;


pub fn string_coder(string: &str, dim: usize) -> Vec<f64> {
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
    use std::hash::{Hasher, SipHasher};
    use super::string_coder;

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
    fn test_string_coder() {
        let result = string_coder("abcdef", 5);
        let expected: Vec<f64> = vec![-1.0, -2.0, -2.0, 1.0, -3.0];
        for i in 0..result.len() {
            assert_eq!(result[i], expected[i]);
        }
    }
}
