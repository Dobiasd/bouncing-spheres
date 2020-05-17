mod raytracer;

use std::ops::{Add, Div, Mul, Sub};
use std::prelude::v1::Vec;
use std::time::SystemTime;

use pixel_canvas::{Canvas, Color, input::MouseState};
use rand::prelude::*;
use raytracer::*;

fn main() {
    let pixel_scale = 4;
    let canvas = Canvas::new(1280, 720)
        .title("raytracer")
        .state(MouseState::new())
        .input(MouseState::handle_input);
    let mut rng = rand::thread_rng();
    let mut last_frame_start_time = SystemTime::now();
    canvas.render(move |mouse, image| {
        let width = image.width() as usize;
        let height = image.height() as usize;
        let pixels = raytracer::render(rng, width / pixel_scale, height / pixel_scale);
        for (y, row) in image.chunks_mut(width).enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                let c = pixels.get(x / pixel_scale, y / pixel_scale);
                *pixel = Color {
                    r: (c.r.max(0.0).min(1.0) * 255.0) as u8,
                    g: (c.g.max(0.0).min(1.0) * 255.0) as u8,
                    b: (c.b.max(0.0).min(1.0) * 255.0) as u8,
                }
            }
        }
        let frame_done_time = SystemTime::now();
        let elapsed = frame_done_time.duration_since(last_frame_start_time);
        last_frame_start_time = frame_done_time;
        println!("{}", elapsed.expect("wat").as_millis());
    });
}
