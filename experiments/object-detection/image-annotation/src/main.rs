use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::cmp::{min, max};
use std::path::Path;
use structopt::StructOpt;
use sdl2::image::LoadTexture;
#[derive(StructOpt)]
struct Opt {
    /// Path to the image to annotate
    image_path: String,
}

fn main() {
    let opt = Opt::from_args();
    let image_path = opt.image_path;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    let window = video_subsystem.window("image-annotation", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture(Path::new(&image_path)).unwrap();
    
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut drawing_box = None;
    let mut boxes = vec![];

    'running: loop {
        canvas.clear();
        canvas.copy(&texture, None, None).unwrap();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseButtonDown { x, y, .. } => {
                    println!("Mouse button down at x: {}, y: {}", x, y);
                    drawing_box = Some(Rect::new(x, y, 0, 0));
                },
                Event::MouseMotion { x, y, .. } => {
                    if let Some(rect) = drawing_box.as_mut() {
                        rect.set_width((x - rect.x()).abs() as u32);
                        rect.set_height((y - rect.y()).abs() as u32);
                    }
                },
                Event::MouseButtonUp { x, y, .. } => {
                    println!("Mouse button up at x: {}, y: {}", x, y);
                    if let Some(mut rect) = drawing_box.take() {
                        rect.set_width((x - rect.x()).abs() as u32);
                        rect.set_height((y - rect.y()).abs() as u32);
                        boxes.push(rect);
                    }
                },
                _ => {}
            }
        }

        // Render the boxes
        for box_rect in &boxes {
            println!("Drawing box at x: {}, y: {}, w: {}, h: {}", box_rect.x(), box_rect.y(), box_rect.width(), box_rect.height());
            canvas.set_draw_color(Color::RGB(0, 255, 0)); // Lime green
            canvas.fill_rect(*box_rect).unwrap();
        }

        if let Some(box_rect) = drawing_box {
            println!("Drawing temporary box at x: {}, y: {}, w: {}, h: {}", box_rect.x(), box_rect.y(), box_rect.width(), box_rect.height());
            canvas.set_draw_color(Color::RGB(0, 255, 0)); // Lime green
            canvas.fill_rect(box_rect).unwrap();
        }

        canvas.present();
    }
}