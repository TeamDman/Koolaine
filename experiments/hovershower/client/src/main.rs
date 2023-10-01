use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use serde::Deserialize;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::windows::named_pipe::ClientOptions;
use tokio::sync::mpsc as async_mpsc;
use tokio::time;

#[derive(Debug, Deserialize)]
struct ElementDetails {
    name: String,
    boundingRect: Vec<i32>,
    controlType: String,
    className: String,
    automationId: String,
    value: Option<String>,
}

#[derive(Debug, Deserialize)]
struct InterestingElement {
    details: ElementDetails,
    depth: i32,
    relationship: String,
}

#[derive(Debug, Deserialize)]
struct ReceivedData {
    cursorPosition: Vec<i32>,
    elementDetails: ElementDetails,
    interestingElements: Vec<InterestingElement>,
}

const ERROR_PIPE_BUSY: i32 = 231;
const PIPE_NAME: &str = r"\\.\pipe\testpipe";

fn main() {
    let data: Arc<Mutex<Option<ReceivedData>>> = Arc::new(Mutex::new(None)); // Shared data between threads

    let data_clone = data.clone();
    // Tokio thread
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let client = loop {
                match ClientOptions::new().open(PIPE_NAME) {
                    Ok(client) => break client,
                    Err(e) if e.raw_os_error() == Some(ERROR_PIPE_BUSY) => (),
                    Err(e) => {
                        println!("Error opening client: {}", e);
                        return;
                    }
                }

                time::sleep(Duration::from_millis(100)).await;
            };

            let mut reader = BufReader::new(client); // Pass ownership
            let mut line = String::new();

            // Reading the incoming Fibonacci numbers
            loop {
                reader
                    .read_line(&mut line)
                    .await
                    .expect("Couldn't read line");

                let received_data: Result<ReceivedData, serde_json::Error> =
                    serde_json::from_str(&line);
                match received_data {
                    Ok(recv) => {
                        let mut data = data_clone.lock().unwrap();
                        println!("Cursor position: {:?}", recv.cursorPosition);
                        *data = Some(recv);
                    }
                    Err(e) => {
                        println!("Couldn't deserialize data: {}", e);
                        if (line == "") {
                            println!("Pipe closed");
                            break;
                        }
                    }
                }

                line.clear();
            }
        });
    });

    let data_clone = data.clone();
    // SDL2 thread
    thread::spawn(move || {
        let sdl_context = sdl2::init().expect("SDL initialization failed");
        let video_subsystem = sdl_context
            .video()
            .expect("Couldn't get SDL video subsystem");

        let window = video_subsystem
            .window("Screen Capture", 800, 600)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())
            .expect("Couldn't build SDL window");

        let mut canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())
            .expect("Couldn't build SDL canvas");
        let mut event_pump = sdl_context.event_pump().unwrap();

        'running: loop {
            canvas.clear();
            canvas.set_draw_color(Color::RGB(255, 0, 0));

            // Lock once and use throughout the loop
            let lock = data_clone.lock().unwrap();
            if let Some(ref data) = *lock {
                // Now `data` is a reference to the inner data, and the lock is held until the end of this block
                // println!("Received: {:?}", data);

                canvas.set_draw_color(Color::RGB(0, 0, 0));
                canvas.clear();
                canvas.set_draw_color(Color::RGB(255, 0, 0));
                canvas
                    .fill_rect(Rect::new(
                        data.cursorPosition[0],
                        data.cursorPosition[1],
                        10,
                        10,
                    ))
                    .unwrap();
            }
            // Drop the lock explicitly (optional, but makes it clear)
            drop(lock);

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }

            canvas.present();

            thread::sleep(Duration::from_millis(100));
        }
    })
    .join()
    .unwrap();
}
