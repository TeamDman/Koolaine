extern crate winapi;

use std::io;
use std::os::windows::ffi::OsStrExt;
use std::ptr;
use structopt::StructOpt;
use winapi::um::fileapi::{CreateFileW, OPEN_EXISTING, ReadFile};
use winapi::um::winnt::{FILE_SHARE_READ, GENERIC_READ, HANDLE};
use winapi::um::handleapi::CloseHandle;
use winapi::um::errhandlingapi::GetLastError;


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

    // Convert PathBuf to a null-terminated wide string
    let v: Vec<u16> = opt.file_path.as_os_str().encode_wide().chain(Some(0)).collect();

    // Call CreateFileW
    let handle: HANDLE = unsafe {
        CreateFileW(
            v.as_ptr(),
            GENERIC_READ,
            FILE_SHARE_READ,
            ptr::null_mut(),
            OPEN_EXISTING,
            0,
            ptr::null_mut(),
        )
    };

    if handle == ptr::null_mut() {
        println!("Failed to open file with CreateFileW");
        return Ok(());
    }

    println!("Successfully opened file with CreateFileW");

    // Read the file content
    let mut buffer = vec![0u8; file_size as usize];
    let mut bytes_read: u32 = 0;

    let success = unsafe {
        ReadFile(
            handle,
            buffer.as_mut_ptr() as *mut _,
            file_size as u32,
            &mut bytes_read,
            ptr::null_mut(),
        )
    };

    // Close the handle
    unsafe {
        CloseHandle(handle);
    }

    if success == 0 {
        let error_code = unsafe { GetLastError() };
        println!("Failed to read file. Error code: {}", error_code);
    } else {
        let content = String::from_utf8_lossy(&buffer);
        println!("Read {} bytes. Content:\n{}", bytes_read, content);
    }

    Ok(())
}
