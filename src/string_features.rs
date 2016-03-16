pub struct StringFeatures {
    chars: Vec<char>,
    position: usize,
    length_cur: usize,
    length_max: usize,
}


impl StringFeatures {
    pub fn new(string: &str) -> StringFeatures {
        StringFeatures{
            chars: string.chars().collect(),
            position: 0,
            length_cur: 1,
            length_max: 6,
        }
    }
}


impl Iterator for StringFeatures {
    type Item = String;
    fn next (&mut self) -> Option<String> {
        // Move to next position if 1) the current substring would exceed the bounds of the current string, or 2) the length of the current substring is greater than the maximum length.
        if ((self.position + self.length_cur) > self.chars.len()) || (self.length_cur > self.length_max) {
            self.position += 1;
            self.length_cur = 1;
        }
        // Terminate the iteration when the position falls of the right of the string.
        if self.position == self.chars.len() { return None }
        // Get the currently defined substring.
        let mut substring = String::with_capacity(self.length_cur);
        for c in &self.chars[self.position..self.position+self.length_cur] {
            substring.push(c.clone());
        }
        // Move on to the next state and return the current substring.
        self.length_cur += 1;
        return Some(substring)
    }
}


#[cfg(test)]
mod tests {
    use super::StringFeatures;

    #[test]
    fn string_features_iterate() {
        let sf = StringFeatures::new("abcdef");
        let mut n = 0;
        for (i, f) in sf.enumerate() {
            println!("{:}: {:?}", i, f);
            n += 1;
        }
        assert_eq!(n, 21);
    }
}
