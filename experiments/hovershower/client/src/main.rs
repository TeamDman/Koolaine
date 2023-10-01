use std::time::Duration;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::windows::named_pipe::ClientOptions;
use tokio::time;

const ERROR_PIPE_BUSY: i32 = 231;
const PIPE_NAME: &str = r"\\.\pipe\testpipe";

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let client = loop {
        match ClientOptions::new().open(PIPE_NAME) {
            Ok(client) => break client,
            Err(e) if e.raw_os_error() == Some(ERROR_PIPE_BUSY) => (),
            Err(e) => return Err(e),
        }

        time::sleep(Duration::from_millis(50)).await;
    };

    let mut reader = BufReader::new(client); // Pass ownership
    let mut line = String::new();

    // Reading the incoming Fibonacci numbers
    loop {
        reader.read_line(&mut line).await?;
        println!("Received from server: {}", line.trim());
        line.clear();
    }
}
