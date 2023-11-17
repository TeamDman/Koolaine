use std::time::Duration;
use clap::arg_enum;
use structopt::StructOpt;
use tokio::{net::windows::named_pipe::ClientOptions, io::{BufReader, AsyncReadExt}};

const PIPE_NAME: &str = r"\\.\pipe\testpipe";
const ERROR_PIPE_BUSY: i32 = 231;

async fn be_client() {
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

    let mut reader = BufReader::new(client);
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

async fn be_server() {

}

arg_enum! {
    #[derive(Debug)]
    enum Mode {
        Server,
        Client
    }
}

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(possible_values = &Mode::variants(), case_insensitive = true)]
    mode: Mode,
    pipe_name: String
}


#[tokio::main]
async fn main() {
    let opt = Args::from_args();
    println!("{:?}", opt);
}
