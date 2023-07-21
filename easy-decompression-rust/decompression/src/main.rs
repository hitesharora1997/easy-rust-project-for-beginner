use std::fs::File; // when working with the files
use std::io; // when working with the io operation

fn main() {
    println!("Hello, world!");
}

fn real_main() {
    let mut file = File::open("input.txt").unwrap();
}
