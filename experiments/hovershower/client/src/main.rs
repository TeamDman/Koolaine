use scrap::{Capturer, Display, Frame};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use serde::Deserialize;
use std::io::ErrorKind::WouldBlock;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::windows::named_pipe::ClientOptions;

use std::ptr;
use winapi::shared::windef::{HDC, HWND};
use winapi::um::wingdi::{BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, SelectObject, SRCCOPY};
use winapi::um::winuser::{GetDC, GetDesktopWindow, ReleaseDC};
use winapi::ctypes::c_void;

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

                tokio::time::sleep(Duration::from_millis(100)).await;
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
                        if line == "" {
                            println!("Pipe closed");
                            break;
                        }
                    }
                }

                line.clear();
            }
        });
    });

    let frame_data: Arc<Mutex<Option<(usize, usize, Vec<u8>)>>> = Arc::new(Mutex::new(None));
    let frame_data_clone = frame_data.clone();
    thread::spawn(move || {
        loop {
            unsafe {
                let hwnd: HWND = GetDesktopWindow();
                let hdc: HDC = GetDC(hwnd);
                let mem_dc: HDC = CreateCompatibleDC(hdc);
                
                let width: i32 = 800;  // Set these values to the width and height of the area you want to capture
                let height: i32 = 600;

                let hbitmap = CreateCompatibleBitmap(hdc, width, height);
                let old_bitmap = SelectObject(mem_dc, hbitmap as *mut c_void);

                BitBlt(mem_dc, 0, 0, width, height, hdc, 0, 0, SRCCOPY);

                // At this point, `hbitmap` contains the captured screen data.
                // You can save it to a file or process it further.

                // Cleanup
                SelectObject(mem_dc, old_bitmap);
                DeleteObject(hbitmap as *mut c_void);
                DeleteDC(mem_dc);
                ReleaseDC(hwnd, hdc);
            }
            // let displays = Display::all().unwrap();
            // for (index, display) in displays.into_iter().enumerate() {
            //     let mut capturer = Capturer::new(display).expect("Couldn't begin capture");
            //     let (w, h) = (capturer.width(), capturer.height());

            //     // Local scope to end mutable borrow quickly
            //     {
            //         let frame = loop {
            //             if let Ok(frame) = capturer.frame() {
            //                 break frame;
            //             }
            //             if let Err(err) = capturer.frame() {
            //                 if err.kind() == WouldBlock {
            //                     thread::sleep(Duration::from_millis(100));
            //                     continue;
            //                 } else {
            //                     panic!("Error capturing frame: {}", err);
            //                 }
            //             }
            //         };
            //         let frame_vec = frame.to_vec(); // Convert the frame to Vec<u8>

            //         let mut frame_data = frame_data_clone.lock().unwrap();
            //         *frame_data = Some((w, h, frame_vec));
            //     } // End of local scope, mutable borrow ends here
            // }
        }
    });

    let data_clone = data.clone();
    let frame_data_clone = frame_data.clone();
    // SDL2 thread
    thread::spawn(move || {
        let sdl_context = sdl2::init().expect("SDL initialization failed");
        let video_subsystem = sdl_context
            .video()
            .expect("Couldn't get SDL video subsystem");

        let window = video_subsystem
            .window("Screeby Jeebie", 800, 600)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())
            .expect("Couldn't build SDL window");

        let mut canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())
            .expect("Couldn't build SDL canvas");
        let texture_creator = canvas.texture_creator();

        let mut event_pump = sdl_context.event_pump().unwrap();

        'running: loop {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();

            let lock = frame_data_clone.lock().unwrap();
            if let Some(ref frame) = *lock {
                let (w, h, frame) = frame; // rgba image
                let mut texture = texture_creator
                    .create_texture_static(
                        sdl2::pixels::PixelFormatEnum::RGBA8888, // Corresponds to the RgbaImage format
                        (*w).try_into().unwrap(),
                        (*h).try_into().unwrap(),
                    )
                    .expect("Failed to create texture");

                let _ = texture.update(None, frame, w * 4);
                canvas.copy(&texture, None, None).unwrap();
            }

            let lock = data_clone.lock().unwrap();
            if let Some(ref data) = *lock {
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
