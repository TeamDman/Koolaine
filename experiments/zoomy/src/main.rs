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


#[derive(Debug)]
struct Region {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}
impl Region {
    fn capture(&self, screen: Screen) -> Result<RgbaImage> {
        screen.capture_area(self.x1, self.y1, (self.x2 - self.x1) as u32, (self.y2 - self.y1) as u32)
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
        print!("mouse_pos: {:?}\n", mouse_pos);

        let mut world_capture_region = Region {
            x1: mouse_pos.x - (CAPTURE_SIZE as i32) / 2,
            y1: mouse_pos.y - (CAPTURE_SIZE as i32) / 2,
            x2: mouse_pos.x + (CAPTURE_SIZE as i32) / 2,
            y2: mouse_pos.y + (CAPTURE_SIZE as i32) / 2,
        };

        // clear buffer
        for i in 0..buffer.len() {
            buffer[i] = 0x153399;
        }

        for screen in Screen::all().unwrap() {
            let mut screen_capture_region = Region {
                x1: world_capture_region.x1 - screen.display_info.x,
                y1: world_capture_region.y1 - screen.display_info.y,
                x2: world_capture_region.x2 - screen.display_info.x,
                y2: world_capture_region.y2 - screen.display_info.y,
            };
            // // Step 1: Calculate how much of the capture box is outside the monitor
            let left_overflow = std::cmp::max(0, screen.display_info.x - screen_capture_region.x1);
            let top_overflow = std::cmp::max(0, screen.display_info.y - screen_capture_region.y1);

            // // Step 2: Use the overflow to calculate the offset for writing into the buffer
            let buffer_x_offset = left_overflow as usize;
            let buffer_y_offset = top_overflow as usize;

            print!("{:?}\t", screen.display_info.id);
            print!("scr: {:?}\t", screen_capture_region);
            print!("leftover: {:?}\t", left_overflow);
            print!("topover: {:?}\t", top_overflow);
            print!("bxo: {:?}\t", buffer_x_offset);
            print!("byo: {:?}\t", buffer_y_offset);
            print!("\n");



            match screen_capture_region.capture(screen) {
                Ok(image) => {
                    for (x, y, pixel) in image.enumerate_pixels() {
                        // Apply the offset when calculating the index
                        let index = (y as usize + buffer_y_offset) * CAPTURE_SIZE
                            + (x as usize + buffer_x_offset);
                        if index < buffer.len() {
                            buffer[index] = ((pixel[0] as u32) << 16)
                                | ((pixel[1] as u32) << 8)
                                | ((pixel[2] as u32) << 0);
                        } else {
                            // not sure why we get here, but it happens
                            // println!("Index out of bounds: {}", index);
                        }
                    }
                }
                Err(e) => {
                    // region is outside of the screen, do nothing
                    // println!("Error: {}", e);
                }
            }
        }

        window
            .update_with_buffer(&buffer, CAPTURE_SIZE, CAPTURE_SIZE)
            .unwrap();

        sleep(Duration::from_millis(16));
    }
}
