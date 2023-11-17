use std::time::Duration;

use tokio::sync::mpsc::{Receiver, Sender};
use tokio::task;
use tokio::time::sleep;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
    net::windows::named_pipe::{ClientOptions, ServerOptions},
};

#[derive(Debug)]
enum Status {
    Online,
    Offline,
}
impl Default for Status {
    fn default() -> Self {
        Status::Offline
    }
}
#[derive(Debug)]
enum Side {
    Reader,
    Writer,
}

#[derive(Debug, Default)]
struct ConnectionInfo {
    read_from: String,
    write_to: String,
    read_status: Status,
    send_status: Status,
}

async fn print_status(connections: &[ConnectionInfo]) {
    for conn in connections {
        println!(
            "STATUS \"{}\": {:?} | \"{}\": {:?}",
            conn.read_from, conn.read_status, conn.write_to, conn.send_status
        );
    }
}

fn spawn_reader(
    i: usize,
    pipe_name: String,
    buffer_tx: Sender<Vec<u8>>,
    status_tx: Sender<(usize, Side, Status)>,
) {
    println!("Spawning reader thread for pipe \"{pipe_name}\"");
    task::spawn(async move {
        loop {
            // println!("Attempting to connect to pipe \"{pipe_name}\"");
            match ClientOptions::new().open(pipe_name.clone()) {
                Ok(client) => {
                    // connection successful
                    println!("Connected to pipe \"{pipe_name}\"");
                    
                    // push status update to channel
                    // println!("Pushing status update to channel for reader {i}");
                    match status_tx.send((i, Side::Reader, Status::Online)).await {
                        Ok(()) => {
                            // println!("Successfully published status update for reader {i}");
                        },
                        Err(e) => {
                            eprintln!("Error sending status update for reader {i}: {e}");
                            return;
                        }
                    };

                    // read from pipe loop
                    let mut reader = BufReader::new(client);
                    let mut data = [0; 1024];
                    loop {
                        // println!("Reading from pipe \"{pipe_name}\"");
                        let read = match reader.read(&mut data).await {
                            Ok(0) => {
                                println!("Pipe \"{pipe_name}\"closed");
                                break;
                            }
                            Ok(n) => {
                                println!("Received {n} bytes from pipe \"{pipe_name}\"");
                                n
                            },
                            Err(e) => {
                                println!("Error reading from pipe \"{pipe_name}\": {}", e);
                                break;
                            }
                        };

                        // println!("Pushing data to channel for pipe \"{pipe_name}\"");
                        let data = data[..read].to_vec();
                        match buffer_tx.send(data).await {
                            Ok(()) => {
                                // println!("Successfully published data to channel for pipe \"{pipe_name}\"");
                            },
                            Err(e) => {
                                eprintln!("Error sending data to pipe \"{pipe_name}\": {}", e);
                                continue;
                            }
                        };
                    }
                }
                Err(e) => {
                    // we expect others to create the pipe
                    // eprintln!("Error opening reader client for connection {i} pipe \"{pipe_name}\", might not be created by foreign process yet: {e}");
                    sleep(Duration::from_millis(100)).await;
                    continue;
                }
            }
        }
    });
}

fn spawn_writer(
    i: usize,
    pipe_name: String,
    mut buffer_rx: Receiver<Vec<u8>>,
    status_tx: Sender<(usize, Side, Status)>,
) {
    println!("Spawning writer thread for pipe \"{pipe_name}\"");
    task::spawn(async move {
        println!("Creating pipe \"{pipe_name}\"");
        match ServerOptions::new().create(pipe_name.clone()) {
            Ok(mut server) => {
                // println!("Successfully created pipe \"{pipe_name}\"");

                // println!("Pushing status update to channel for writer {i}");
                match status_tx.send((i, Side::Writer, Status::Online)).await {
                    Ok(()) => {
                        // println!("Successfully published status update for writer {i}");
                    },
                    Err(e) => {
                        eprintln!("Error sending status to for writer {i}: {e}");
                        eprintln!("Ending writer thread for pipe \"{pipe_name}\"");
                        return;
                    }
                };
                loop {
                    // println!("Waiting for data from channel for pipe \"{pipe_name}\"");
                    let content = match buffer_rx.recv().await {
                        Some(content) => {
                            // println!("Received {} bytes from channel for pipe \"{pipe_name}\"", content.len());
                            content
                        },
                        None => {
                            println!("Received termination for data channel for pipe \"{pipe_name}\"");
                            println!("Ending writer thread for pipe \"{pipe_name}\"");
                            return;
                        }
                    };
                    println!("Relaying {} bytes to pipe \"{pipe_name}\"", content.len());
                    match server.write(&content).await {
                        Ok(n) => {
                            println!("Successfully relayed {n} bytes to pipe \"{pipe_name}\"");
                        },
                        Err(e) => {
                            eprintln!("Error writing to pipe \"{pipe_name}\": {}", e);
                            println!("Ending writer thread for pipe \"{pipe_name}\"");
                            return;
                        }
                    };
                }
            }
            Err(e) => {
                eprintln!("Error creating server for pipe \"{pipe_name}\": {}", e);
                println!("Ending writer thread for pipe \"{pipe_name}\"");
                return;
            }
        }
    });
}

#[tokio::main]
async fn main() {
    let mut connections = vec![
        ConnectionInfo {
            read_from: String::from("\\\\.\\pipe\\bruh"),
            write_to: String::from("\\\\.\\pipe\\testpipe"),
            ..ConnectionInfo::default()
        },
        // ConnectionInfo {
        //     read_from: String::from("\\\\.\\pipe\\microphone_out"),
        //     write_to: String::from("\\\\.\\pipe\\transcriber_in"),
        //     ..ConnectionInfo::default()
        // },
        // ConnectionInfo {
        //     read_from: String::from("\\\\.\\pipe\\transcriber_out"),
        //     write_to: String::from("\\\\.\\pipe\\tts_in"),
        //     ..ConnectionInfo::default()
        // },
    ];
    let (status_tx, mut status_rx) = tokio::sync::mpsc::channel::<(usize, Side, Status)>(32);
    // for each connection
    // spawn the reader
    //  - connection retry loop, push status
    //  - read pipe loop, push data to channel
    // spawn the writer
    //  - create pipe, push status
    //  - read channel loop, push data to pipe
    for (i, conn) in connections.iter().enumerate() {
        // the reader will read from the pipe and write to the buffer
        // the writer will read from the buffer and write to the pipe
        let (buffer_tx, buffer_rx) = tokio::sync::mpsc::channel::<Vec<u8>>(32);
        // buffer_tx.send("value".as_bytes().to_vec()).await.unwrap();
        spawn_reader(i, conn.read_from.clone(), buffer_tx, status_tx.clone());
        spawn_writer(i, conn.write_to.clone(), buffer_rx, status_tx.clone());
    }

    // receive changes to connection status
    loop {
        match status_rx.recv().await {
            Some((i, Side::Writer, status)) => {
                connections[i].send_status = status;
                print_status(&connections).await;
            }
            Some((i, Side::Reader, status)) => {
                connections[i].read_status = status;
                print_status(&connections).await;
            }
            None => (),
        };
    }
}
