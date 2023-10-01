use std::time::Duration;
use tokio::net::windows::named_pipe::ClientOptions;
use tokio::time;
use windows_sys::Win32::Foundation::ERROR_PIPE_BUSY;

const PIPE_NAME: &str = r"\\.\pipe\testpipe";

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let client = loop {
        match ClientOptions::new().open(PIPE_NAME) {
            Ok(client) => break client,
            Err(e) if e.raw_os_error() == Some(ERROR_PIPE_BUSY as i32) => (),
            Err(e) => return Err(e),
        }

        time::sleep(Duration::from_millis(50)).await;
    };

    let mut buffer = [0u8; 512];

    // Reading and writing on the client.
    // The actual I/O operations are similar to what you'd do with a TcpStream.

    Ok(())
}
