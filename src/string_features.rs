use std::hash::{Hash, Hasher};
use fnv::FnvHasher;


// String features are character ngrams of specified lengths. By exploding strings into a much larger number of sub-features in this way, non-identical but similar strings will end up overlapping in the sub-features they exhibit. Thus string similarity can be measured in terms of the overlapping sub-features.


// Returns a vector of character ngrams contained in the input string.
pub fn get_string_features(string: &str, ngram_lengths: &Vec<usize>) -> Vec<(u64, f64)> {
    let mut features: Vec<(u64, f64)> = Vec::new();
    let chars: Vec<char> = string.to_lowercase().chars().collect();
    let n = chars.len();
    for l in ngram_lengths {
        if l <= &n {
            for pos in 0..(n - l + 1) {
                // Get the character ngram.
                let ngram = &chars[pos..(pos + l)];
                // Compute and store the hash value and weight for the character ngram.
                let mut hasher = FnvHasher::default();
                ngram.hash(&mut hasher);
                let hash_value = hasher.finish();
                let weight = ((ngram.len() + 1) as f64).ln();
                features.push((hash_value, weight));
            }
        }
    }
    features
}


#[cfg(test)]
mod tests {
    use fnv::FnvHasher;
    use std::hash::Hasher;
    use super::get_string_features;
    use test::Bencher;
    use utils::random_string;

    /// Hashing a string on 2 occasions should give the same result.
    #[test]
    fn rehash_gives_same() {
        let string = "A random string.";
        let mut hasher = FnvHasher::default();
        hasher.write(string.as_bytes());
        let h1 = hasher.finish();
        let mut hasher = FnvHasher::default();
        hasher.write(string.as_bytes());
        let h2 = hasher.finish();
        assert_eq!(h1, h2);
    }

    #[bench]
    fn string_features_iterate(b: &mut Bencher) {
        // Generate random string.
        let random_string = random_string(100);
        // Benchmark iterating over the hash values of the features of the string.
        b.iter(|| {
            let string_features = get_string_features(&random_string, &vec![2, 3, 4, 5, 6, 7]);
            let mut sum: u64 = 0;
            for (hash_value, _) in string_features { sum = sum ^ hash_value };
            sum
        })
    }
}
