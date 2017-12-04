use std::fs::File;
use std::io::prelude::*;


/// read content of file 'fname' and return as String
pub fn read_file(fname: &str) -> String {
    let mut f = File::open(fname)
        .expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    return contents.trim().to_string();
}
