extern crate flate2; // import the `flate2` package

use flate2::write::GzDecoder; // from `flate2` package we are using this encoder
use flate2::Decompress; // compression package
use std::env::args; // to accept the command line arguments
use std::fs::File; // since we are working with the files
use std::io::copy; // to copy the file
use std::io::BufReader; // to read the files
use std::time::Instant; // here it will show how much time taken to compress the files

fn main() {
    if args().len() != 3 {
        // if you give less than 3 arg the program will get panic
        eprintln!("Usage: 'source' 'target'"); // we use `eprintln` instead of `println` as eprintln is only for error and progress message.
        return;
    }

    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();
    let mut decoder = GzEncode::new(output, Decompress::de);
    let start = Instant::now();

    copy(&mut input, &mut encoder).unwrap();

    let output = encoder.finish();

    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );

    println!(
        "Target len: {:?}",
        output
            .expect("issue in the Target")
            .metadata()
            .unwrap()
            .len()
    );
    println!("Elapsed time: {:?}", start.elapsed());
}
