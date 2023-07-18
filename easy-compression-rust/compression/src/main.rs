extern crate flate2; //import the flate2 package

use flate2::write::GzEncoder; // from flate2 package we are using this encoder
use flate2::Compression; // compression package
use std::env::args; // to accept the command line arguments
use std::fs::File; // since we are working with the files
use std::io::copy; // to copy the file
use std::io::BufReader; // to read the files
use std::time::Instant; // here it will show how much time taken to compress the files 

fn main() {
    if args().len() != 3 { 
        eprintln!("Usage: 'source' 'target' ");
        return;
    }
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().net(2).unwrap()).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
    let output = encoder

}
