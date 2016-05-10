use std::hash::{Hasher, SipHasher};
use std::hash::Hash;


// TODO: Look into string kernels here to see if there's a more general framework that can be used to iterate over the features of strings.


/// Structure that can be iterated over to generate the sub-features of a string.
///
/// The sub-features are character ngrams up to a maximum length. By exploding strings into a much larger number of sub-features in this way, non-identical but similar strings will end up overlapping in the sub-features they exhibit. Thus string similarity can be measured in terms of the overlapping sub-features.
pub struct StringFeatures {
    chars: Vec<char>,
    position: usize,
    length_cur: usize,
    length_min: usize,
    length_max: usize,
}


impl StringFeatures {
    pub fn new(string: &str, ignore_case: bool, length_min: usize, length_max: usize) -> StringFeatures {
        let chars: Vec<char>;
        if ignore_case { chars = string.to_lowercase().chars().collect(); }
        else { chars = string.chars().collect(); }
        StringFeatures{
            chars: chars,
            position: 0,
            length_min: length_min,
            length_max: length_max,
            length_cur: length_min,
        }
    }

    pub fn default(string: &str) -> StringFeatures {
        let chars: Vec<char> = string.to_lowercase().chars().collect();
        StringFeatures{
            chars: chars,
            position: 0,
            length_min: 2,
            length_max: 10,
            length_cur: 2,
        }
    }
}


impl Iterator for StringFeatures {
    type Item = usize;
    fn next (&mut self) -> Option<usize> {
        let n = self.chars.len();
        // Move to next position if 1) the current substring would exceed the bounds of the current string, or 2) the length of the current substring is greater than the maximum length.
        if ((self.position + self.length_cur) > n) || (self.length_cur > self.length_max) {
            self.position += 1;
            self.length_cur = self.length_min;
        }
        // Terminate the iteration when the position falls of the right of the string.
        if self.position == (n - self.length_min + 1) { return None }
        // Get the sub-string of characters.
        let subchars = &self.chars[self.position..(self.position + self.length_cur)];
        // Compute the hash value of the sub-string.
        let mut hasher = SipHasher::new();
        subchars.hash(&mut hasher);
        let hash_value = hasher.finish() as usize;
        // Move on to the next state and return the current substring.
        self.length_cur += 1;
        return Some(hash_value)
    }
}


#[cfg(test)]
mod tests {
    use std::hash::{Hasher, SipHasher};
    use super::StringFeatures;
    use test::Bencher;
    use utils::random_string;

    /// Hashing a string on 2 occasions should give the same result.
    #[test]
    fn rehash_gives_same() {
        let string = "A random string.";
        let mut hasher = SipHasher::new();
        hasher.write(string.as_bytes());
        let h1 = hasher.finish();
        hasher = SipHasher::new();
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
            let string_features = StringFeatures::default(&random_string);
            let mut sum: usize = 0;
            for hash_value in string_features { sum = sum ^ hash_value };
            sum
        })
    }
}
