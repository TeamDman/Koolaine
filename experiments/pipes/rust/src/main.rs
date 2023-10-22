use std::time::Duration;

use tokio::{net::windows::named_pipe::ClientOptions, io::{BufReader, AsyncBufReadExt, AsyncReadExt}};

const PIPE_NAME: &str = r"\\.\pipe\testpipe";
const ERROR_PIPE_BUSY: i32 = 231;

#[tokio::main]
async fn main() {
    let client = loop {
        match ClientOptions::new().open(PIPE_NAME) {
            Ok(client) => break client,
            Err(e) if e.raw_os_error() == Some(ERROR_PIPE_BUSY) => (),
            Err(e) => {
                println!("Error opening client: {}", e); 
                // we expect others to create the pipe
                return;
            }
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    };

    let mut reader = BufReader::new(client); // Pass ownership
    let mut data = [0; 1024];

    loop {
        let read = match reader.read(&mut data).await {
            Ok(0) => {
                println!("Pipe closed");
                break;
            }
            Ok(n) => n,
            Err(e) => {
                println!("Error reading from pipe: {}", e);
                break;
            }
        };
        let message = String::from_utf8_lossy(&data[..read]);
        println!("Received: {}", message);
    }
    
}
