use std::fs::{self, File}; // when working with the files
                           // use std::io; // when working with the io operation
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
            None => continue,              // if we got none we were will continue.
        };

        // Check comments in the files
        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment {} ", i, comment);
            }
        }

        if (file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\" ", i, outpath.display());
            fs::create_dir_all(outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
        }
    }

    return 1;
}
