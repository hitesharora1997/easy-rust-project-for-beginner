use std::fs::File; // when working with the files
use std::io; // when working with the io operation
use zip::ZipArchive;

fn main() {
    std::process::exit(real_main());
}

fn real_main() -> i32 {
    let args: Vec<_> = std::env::args().collect();

    if args.len() > 2 {
        println!("Usage:\n\t{} [filename]", &args[0]);
        return -1;
    };

    let fname = std::path::Path::new(&*args[1]);
    let file = File::open(&fname).unwrap(); // we open up the file to read

    let mut archive = ZipArchive::new(file).unwrap(); // Here we are gonna use the zip reader to read the data from the files

    // To go over all the content

    for i in 0..archive.len() {
        // go till archive len
        let file = archive.by_index(i).unwrap(); // by index means going through all the files in the compressed files one by one

        let outpath = match file.enclosed_name() {
            // enclosed_name is to change the absolute path of the file to relative path.
            Some(path) => path.to_owned(), // We are clowning all the data from the Zip file. In this case we are just clowning the path.
            None => continue,
        };
    }
    return 1;
}
