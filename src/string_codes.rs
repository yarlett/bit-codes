pub struct StringFeatures {
    chars: Vec<char>,
    position: usize,
}


impl StringFeatures {
    pub fn new(string: &str) -> StringFeatures {
        let chars: Vec<char> = string.chars().collect();
        StringFeatures{ chars: chars, position: 0 }
    }
}


// impl<'a> Iterator for StringFeatures {
//     type Item = &'a[char];
//     fn next (&mut self) -> Option<&[char]> {
//         if self.position < self.chars.len() {
//             let out = &self.chars[self.position];
//             self.position += 1;
//             Some(out)
//         } else {
//             None()
//         }
//     }
// }
