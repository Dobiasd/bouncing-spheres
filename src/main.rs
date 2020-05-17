use std::ops::{Add, Mul};
use std::prelude::v1::Vec;
use std::time::SystemTime;

use pixel_canvas::{Canvas, Color, input::MouseState};
use rand::prelude::*;

#[derive(Copy, Clone)]
struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Copy, Clone)]
struct Position {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Copy, Clone)]
struct FloatColor {
    r: f64,
    g: f64,
    b: f64,
}

struct FloatImage {
    data: Vec<FloatColor>,
    width: usize,
    height: usize,
}

impl FloatImage {
    pub fn new(width: usize, height: usize) -> FloatImage {
        FloatImage {
            data: vec![FloatColor { r: 0.0, g: 0.0, b: 0.0 }; width * height],
            width,
            height,
        }
    }
    pub fn get(&self, x: usize, y: usize) -> &FloatColor {
        return &self.data[y * self.width + x];
    }
}

fn render(width: usize, height: usize) -> FloatImage {
    let image = FloatImage::new(width, height);
    return image;
}

fn main() {
    let pixel_scale = 10;
    let canvas = Canvas::new(1280, 720)
        .title("raytracer")
        .state(MouseState::new())
        .input(MouseState::handle_input);
    let mut rng = rand::thread_rng();
    canvas.render(move |mouse, image| {
        let width = image.width() as usize;
        let height = image.height() as usize;
        let pixels = render(width / pixel_scale, height / pixel_scale);
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
    });
}
