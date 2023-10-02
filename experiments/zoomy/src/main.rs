extern crate minifb;
extern crate rsautogui;
extern crate winit;

use anyhow::Result;
use image::{ImageBuffer, Rgba, RgbaImage};
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
        screen.capture_area(
            self.x1,
            self.y1,
            (self.x2 - self.x1) as u32,
            (self.y2 - self.y1) as u32,
        )
    }
}
fn main() {
    let mut img_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::new(CAPTURE_SIZE as u32, CAPTURE_SIZE as u32);

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
        if window.is_key_down(Key::Space) {
            // println!("pausing!");
            window.update();
            sleep(Duration::from_millis(100));
            continue;
        }
        let mouse_pos = mouse.get_position().unwrap();
        print!("mouse_pos: {:?}\n", mouse_pos);

        let mut world_capture_region = Region {
            x1: mouse_pos.x - (CAPTURE_SIZE as i32) / 2,
            y1: mouse_pos.y - (CAPTURE_SIZE as i32) / 2,
            x2: mouse_pos.x + (CAPTURE_SIZE as i32) / 2,
            y2: mouse_pos.y + (CAPTURE_SIZE as i32) / 2,
        };

        // clear buffer
        for x in 0..CAPTURE_SIZE as u32 {
            for y in 0..CAPTURE_SIZE as u32 {
                img_buffer.put_pixel(x, y, Rgba([21, 51, 153, 255]));
            }
        }

        for screen in Screen::all().unwrap() {
            let mut screen_capture_region = Region {
                x1: world_capture_region.x1 - screen.display_info.x,
                y1: world_capture_region.y1 - screen.display_info.y,
                x2: world_capture_region.x2 - screen.display_info.x,
                y2: world_capture_region.y2 - screen.display_info.y,
            };
            let mut screen_capture_region_inbounds = Region {
                x1: 0.max(screen_capture_region.x1),
                y1: 0.max(screen_capture_region.y1),
                x2: (screen.display_info.width as i32).min(screen_capture_region.x2),
                y2: (screen.display_info.height as i32).min(screen_capture_region.y2),
            };

            print!(
                "ID {:<10} sx {:<10} sy {:<10} ",
                screen.display_info.id, screen.display_info.x, screen.display_info.y,
            );

            match screen_capture_region.capture(screen) {
                Ok(image) => {
                    let x_offset = screen_capture_region_inbounds.x1 - screen_capture_region.x1;
                    let y_offset = screen_capture_region_inbounds.y1 - screen_capture_region.y1;

                    for (x, y, pixel) in image.enumerate_pixels() {
                        let mut x_img = x as i32 + x_offset;
                        let mut y_img = y as i32 + y_offset;
                        fn clamp<T: Ord>(value: T, min: T, max: T) -> T {
                            std::cmp::max(min, std::cmp::min(value, max))
                        }
                        x_img = clamp(x_img, 0, CAPTURE_SIZE as i32 - 1);
                        y_img = clamp(y_img, 0, CAPTURE_SIZE as i32 - 1);
                        img_buffer.put_pixel(
                            x_img as u32,
                            y_img as u32,
                            Rgba([pixel[0], pixel[1], pixel[2], 255]),
                        );
                    }
                }
                Err(e) => {
                    print!("Error: {} ", e);
                }
            }

            print!("\n");
        }
        let flat_buffer: Vec<u32> = img_buffer
            .pixels()
            .map(|p| {
                let Rgba([r, g, b, _]) = *p;
                ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
            })
            .collect();
        window
            .update_with_buffer(&flat_buffer, CAPTURE_SIZE, CAPTURE_SIZE)
            .unwrap();

        sleep(Duration::from_millis(16));
    }
}
