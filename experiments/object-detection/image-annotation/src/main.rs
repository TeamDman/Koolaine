extern crate sdl2;
extern crate structopt;

use structopt::StructOpt;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::path::Path;
use crate::sdl2::image::LoadTexture;
use std::cmp::{min, max};


#[derive(StructOpt, Debug)]
#[structopt(name = "image-annotation")]
struct Opt {
    /// Image path
    #[structopt(short, long, parse(from_os_str))]
    image_path: std::path::PathBuf,
}

enum DrawState {
    Idle,
    Drawing { start_x: i32, start_y: i32 },
}
fn main() {
    let opt = Opt::from_args();
    let image_path = opt.image_path;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Image Annotation", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let texture = texture_creator.load_texture(Path::new(&image_path)).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut boxes: Vec<Rect> = Vec::new(); // A list to hold the coordinates for our boxes
    let mut draw_state = DrawState::Idle;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseButtonDown { x, y, .. } => {
                    draw_state = DrawState::Drawing { start_x: x, start_y: y };
                },
                Event::MouseMotion { x, y, .. } => {
                    if let DrawState::Drawing { start_x, start_y } = draw_state {
                        let new_box = Rect::new(
                            min(start_x, x),
                            min(start_y, y),
                            (max(start_x, x) - min(start_x, x)) as u32,
                            (max(start_y, y) - min(start_y, y)) as u32,
                        );
                        if let Some(last_box) = boxes.last_mut() {
                            *last_box = new_box;
                        }
                    }
                },
                Event::MouseButtonUp { .. } => {
                    if let DrawState::Drawing { .. } = draw_state {
                        draw_state = DrawState::Idle;
                    }
                },
                _ => {}
            }
        }

        canvas.copy(&texture, None, None).expect("Render failed");

        // Draw all the boxes
        canvas.set_draw_color(Color::RGB(0, 255, 0)); // Lime green
        for box_rect in &boxes {
            canvas.draw_rect(*box_rect).unwrap(); // Use draw_rect instead of fill_rect for thick lines
        }

        canvas.present();
    }
}