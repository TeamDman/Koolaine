extern crate minifb;
extern crate rsautogui;
extern crate winit;

use anyhow::Result;
use image::RgbaImage;
use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use mouse_rs::Mouse;
use screenshots::Screen;
use std::thread::sleep;
use std::time::Duration;

const CAPTURE_SIZE: usize = 100;

struct Region {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}
impl Region {
    fn capture(&self, screen: Screen) -> Result<RgbaImage> {
        screen.capture_area(self.x, self.y, self.width, self.height)
    }
}
fn main() {
    let mut buffer: Vec<u32> = vec![0; CAPTURE_SIZE * CAPTURE_SIZE];

    let mut window = Window::new(
        "Zoom Window",
        CAPTURE_SIZE,
        CAPTURE_SIZE,
        WindowOptions {
            resize: true,
            scale: Scale::X4,
            scale_mode: ScaleMode::Stretch,
            ..Default::default()
        },
    )
    .unwrap();

    let mouse = Mouse::new();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mouse_pos = mouse.get_position().unwrap();
        let capture_region_world = Region {
            x: mouse_pos.x - 50,
            y: mouse_pos.y - 50,
            width: 100,
            height: 100,
        };
        println!("mouse_pos: {:?}", mouse_pos);

        // clear buffer
        for i in 0..buffer.len() {
            buffer[i] = 0;
        }

        for screen in Screen::all().unwrap() {
            let capture_region_screen = Region {
                x: capture_region_world.x - screen.display_info.x,
                y: capture_region_world.y - screen.display_info.y,
                width: capture_region_world.width,
                height: capture_region_world.height,
            };

            // Step 1: Calculate how much of the capture box is outside the monitor
            let left_overflow = std::cmp::max(0, screen.display_info.x - capture_region_screen.x);
            let top_overflow = std::cmp::max(0, screen.display_info.y - capture_region_screen.y);

            // Step 2: Use the overflow to calculate the offset for writing into the buffer
            let buffer_x_offset = left_overflow as usize;
            let buffer_y_offset = top_overflow as usize;

            match capture_region_screen.capture(screen) {
                Ok(image) => {
                    for (x, y, pixel) in image.enumerate_pixels() {
                        // Apply the offset when calculating the index
                        let index = (y as usize + buffer_y_offset) * CAPTURE_SIZE
                            + (x as usize + buffer_x_offset);

                        buffer[index] = ((pixel[0] as u32) << 16)
                            | ((pixel[1] as u32) << 8)
                            | ((pixel[2] as u32) << 0);
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }

        window
            .update_with_buffer(&buffer, CAPTURE_SIZE, CAPTURE_SIZE)
            .unwrap();

        sleep(Duration::from_millis(16));
    }
}
