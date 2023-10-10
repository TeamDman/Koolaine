extern crate structopt;

use std::fs::File;
use std::io::{self, Read};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Path to the file to open
    #[structopt(parse(from_os_str))]
    file_path: std::path::PathBuf,
}
fn main() -> io::Result<()> {
    let opt = Opt::from_args();
    println!("Opening file: {:?}", opt.file_path);

    let metadata = std::fs::metadata(&opt.file_path)?;
    println!("Metadata: {:?}", metadata);

    // Display the file size in bytes
    let file_size = metadata.len();
    println!("File size: {} bytes", file_size);

    // Open the file
    let mut file = File::open(&opt.file_path)?;

    // Just to make sure the file handle is actually being used, we'll read one byte
    let mut buffer = [0; 1];
    let _ = file.read(&mut buffer)?;

    // File handle will be closed here as it goes out of scope
    println!("File handle is now closed.");
    Ok(())
}
