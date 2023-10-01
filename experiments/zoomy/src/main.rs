extern crate minifb;
extern crate rsautogui;

use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use rsautogui::screen;
use std::thread::sleep;
use std::time::Duration;

const CAPTURE_SIZE: usize = 100;
const ZOOM_FACTOR: usize = 4;

fn main() {
    let mut buffer: Vec<u32> = vec![0; CAPTURE_SIZE * CAPTURE_SIZE];

    let mut window = Window::new(
        "Zoom Window",
        100,
        100,
        WindowOptions {
            resize: true,
            scale: Scale::X4,
            scale_mode: ScaleMode::Stretch,
            ..Default::default()
        },
    )
    .unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let (x, y) = rsautogui::mouse::position(); // Replace this with your actual mouse position fetching code

        // Capture the frame centered around mouse cursor
        let x_u16 = x as u16;
        let y_u16 = y as u16;
        let capture_size_u16 = CAPTURE_SIZE as u16;

        let (screen_width, screen_height) = screen::size();

        let x_left = (x_u16 as i32 - (capture_size_u16 / 2) as i32).max(0) as u16;
        let y_top = (y_u16 as i32 - (capture_size_u16 / 2) as i32).max(0) as u16;

        let x_right = (x_left as u32 + capture_size_u16 as u32).min(screen_width as u32) as u16;
        let y_bottom = (y_top as u32 + capture_size_u16 as u32).min(screen_height as u32) as u16;

        let adjusted_width = (x_right - x_left).max(1);
        let adjusted_height = (y_bottom - y_top).max(1);

        let region = screen::screenshot(x_left, y_top, adjusted_width, adjusted_height);

        // Convert the frame data to a buffer compatible with minifb
        if let image::DynamicImage::ImageRgba8(image) = region {
            for (x, y, pixel) in image.enumerate_pixels() {
                let index = y as usize * CAPTURE_SIZE + x as usize;
                buffer[index] = ((pixel[0] as u32) << 16) | // R
                    ((pixel[1] as u32) << 8) | // G
                    ((pixel[2] as u32) << 0);
                //   | // B
                // (pixel[3] as u32); // A
            }
        }

        // Update the window
        window
            .update_with_buffer(&buffer, CAPTURE_SIZE, CAPTURE_SIZE)
            .unwrap();

        // Sleep to prevent high CPU usage
        sleep(Duration::from_millis(16));
    }
}
