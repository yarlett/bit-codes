extern crate bit_codes;

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let path = Path::new("/Users/yarlett/Desktop/test_data/test_data.json");
    println!("{:}", path.display());

    let file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", path.display(), Error::description(&why)),
        Ok(file) => file,
    };

    let foo = BufReader::new(&file);
    for line in foo.lines() {
        let l = line.unwrap();
        // println!("{}", &l);
        let bc = bit_codes::encoders::string_to_bit_code(&l, 256);
        println!("{:?}", bc);
    }

}
