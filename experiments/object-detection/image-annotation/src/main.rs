use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::image::LoadTexture;
use std::path::Path;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "image-annotation")]
struct Opt {
    /// Path to the image
    #[structopt(short, long)]
    image: String,
}

fn main() {
    let opt = Opt::from_args();
    let image_path = opt.image;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("image annotation", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture(Path::new(&image_path)).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut permanent_boxes = Vec::new();
    let mut current_box = None;

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
                    current_box = Some((x, y, x, y));
                },
                Event::MouseMotion { x, y, .. } => {
                    if let Some((x1, y1, _, _)) = current_box {
                        current_box = Some((x1, y1, x, y));
                    }
                },
                Event::MouseButtonUp { .. } => {
                    if let Some(box_coords) = current_box {
                        permanent_boxes.push(box_coords);
                    }
                    current_box = None;
                },
                _ => {}
            }
        }

        for &(x1, y1, x2, y2) in &permanent_boxes {
            let min_x = x1.min(x2);
            let min_y = y1.min(y2);
            let max_x = x1.max(x2);
            let max_y = y1.max(y2);
            canvas.set_draw_color(Color::RGB(0, 255, 0));
            canvas.fill_rect(Rect::new(min_x, min_y, (max_x - min_x) as u32, (max_y - min_y) as u32)).unwrap();
        }

        if let Some((x1, y1, x2, y2)) = current_box {
            let min_x = x1.min(x2);
            let min_y = y1.min(y2);
            let max_x = x1.max(x2);
            let max_y = y1.max(y2);
            canvas.set_draw_color(Color::RGB(0, 255, 0));
            canvas.fill_rect(Rect::new(min_x, min_y, (max_x - min_x) as u32, (max_y - min_y) as u32)).unwrap();
            println!("Current box: x: {}, y: {}, width: {}, height: {}", min_x, min_y, max_x - min_x, max_y - min_y);
        }

        canvas.present();
    }
}