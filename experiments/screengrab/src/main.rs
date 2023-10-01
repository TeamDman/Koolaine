extern crate scrap;
extern crate image;

use scrap::{Capturer, Display};
use std::io::ErrorKind::WouldBlock;
use std::{env, thread, time};
use image::{ColorType, ImageEncoder};
use image::codecs::png::PngEncoder;
use std::fs::File;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // Get the directory from the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <output_directory>", args[0]);
        return;
    }
    let output_dir = Path::new(&args[1]);

    // Create the directory if it doesn't exist
    if !output_dir.exists() {
        std::fs::create_dir_all(output_dir).unwrap();
    }

    // Get all displays
    let displays = Display::all().unwrap();

    // Capture and save screenshots for each display
    for (index, display) in displays.into_iter().enumerate() {  // into_iter() consumes the vector, no need to clone Display
        let mut capturer = Capturer::new(display).expect("Couldn't begin capture");
        let (w, h) = (capturer.width(), capturer.height());

        // Wait until a frame is available
        let frame = loop {
            if let Ok(frame) = capturer.frame() {
                break frame;
            }

            if let Err(err) = capturer.frame() {
                if err.kind() == WouldBlock {
                    // Keep spinning loop if frames are not yet available
                    thread::sleep(time::Duration::from_millis(100));
                    continue;
                } else {
                    panic!("Error capturing frame: {}", err);
                }
            }
        };

        // Convert frame to image
        let img = image::RgbaImage::from_raw(w as u32, h as u32, frame.to_vec()).unwrap();

        // Get current Unix timestamp
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        // Generate the output path
        let output_path = output_dir.join(format!("screenshot_{}_{}.png", timestamp, index));

        // Save the image
        let fout = &mut File::create(output_path).expect("Couldn't create file");
        let encoder = PngEncoder::new(fout);
        encoder
            .write_image(
                &img,
                img.width(),
                img.height(),
                ColorType::Rgba8,
            )
            .expect("Couldn't write image");
    }
}
