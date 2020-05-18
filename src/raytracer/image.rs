use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use crate::raytracer::color::Color;

pub struct Image {
    data: Vec<Color>,
    width: usize,
    height: usize,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        Image {
            data: vec![Color { r: 0.0, g: 0.0, b: 0.0 }; width * height],
            width,
            height,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> &Color {
        return &self.data[y * self.width + x];
    }

    pub fn set(&mut self, x: usize, y: usize, c: Color) {
        self.data[y * self.width + x] = c;
    }

    pub fn save_png(&self, path: &Path) {
        let file = File::create(path).unwrap();
        let mut encoder =
            png::Encoder::new(BufWriter::new(file),
                              self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        let png_data = (0..self.height).map(|y| {
            (0..self.width).map(|x| {
                let c = self.get(x, self.height - y - 1).to_canvas_color();
                vec![c.r, c.g, c.b]
            }).collect::<Vec<Vec<u8>>>().concat()
        }).collect::<Vec<Vec<u8>>>().concat();
        writer.write_image_data(&png_data).unwrap();
    }
}
