extern crate winapi;

use std::ptr::null_mut;
use winapi::shared::ntdef::LARGE_INTEGER;
use winapi::um::fileapi::CreateFileW;
use winapi::um::handleapi::CloseHandle;
use winapi::um::winnt::{FILE_SHARE_READ, GENERIC_READ, HANDLE};

// Define CfHydratePlaceholder FFI
extern "system" {
    fn CfHydratePlaceholder(
        FileHandle: HANDLE,
        StartingOffset: LARGE_INTEGER,
        Length: LARGE_INTEGER,
        HydrateFlags: u32,
        Overlapped: *mut std::ffi::c_void
    ) -> i32;
}

fn main() {
    // Initialize LARGE_INTEGERs
    let mut starting_offset = LARGE_INTEGER { QuadPart_mut: 0 };
    let mut length = LARGE_INTEGER { QuadPart_mut: -1 };
    
    let file_path = "C:\\Users\\TeamD\\OneDrive\\Documents\\onedrivetesting\\hello.txt";
    let file_path: Vec<u16> = file_path.encode_utf16().chain(Some(0)).collect();

    let file_handle: HANDLE = unsafe {
        CreateFileW(
            file_path.as_ptr(),
            GENERIC_READ,
            FILE_SHARE_READ,
            null_mut(),
            3,  // OPEN_EXISTING
            0,
            null_mut(),
        )
    };

    if file_handle == null_mut() {
        println!("Failed to open file.");
        return;
    }

    let hydrate_result = unsafe {
        CfHydratePlaceholder(file_handle, starting_offset, length, 0, null_mut())
    };

    if hydrate_result != 0 {
        println!("Failed to hydrate placeholder. Error code: {}", hydrate_result);
    }

    unsafe { CloseHandle(file_handle); }
}
