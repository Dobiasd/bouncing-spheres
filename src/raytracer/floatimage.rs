use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use crate::raytracer::floatcolor::FloatColor;

pub struct FloatImage {
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
    pub fn set(&mut self, x: usize, y: usize, c: FloatColor) {
        self.data[y * self.width + x] = c;
    }
    pub fn save_png(&self, path: &Path) {
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        let png_data = (0..self.height).map(|y| {
            (0..self.width).map(|x| {
                let c = self.get(x, self.height - y - 1);
                let r = (c.r.max(0.0).min(1.0) * 255.0) as u8;
                let g = (c.g.max(0.0).min(1.0) * 255.0) as u8;
                let b = (c.b.max(0.0).min(1.0) * 255.0) as u8;
                vec![r, g, b]
            }).collect::<Vec<Vec<u8>>>().concat()
        }).collect::<Vec<Vec<u8>>>().concat();
        writer.write_image_data(&png_data).unwrap();
    }
}