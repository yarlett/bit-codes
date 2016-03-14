struct StringFeatures<'a> {
    string: &'a str,
    position: usize,
}

impl Iterator for StringFeatures {
    type Item = &str;
    fn next (&mut self) -> Option<&str> {
        if self.position < self.string.chars.len() {
            let out = &self.string.chars.nth(self.position);
            self.position += 1;
            Some(out)
        } else {
            None()
        }
    }
}


fn foo(s: &str) -> StringFeatures {
    StringFeatures{ s: s, pos: 0 }
}


pub fn bar(s: &str) -> Vec<f64> {
    let mut v: Vec<f64> = Vec::new();
    v
}
