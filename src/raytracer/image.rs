use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use crate::raytracer::color::Color;

pub struct Image {
    pub data: Vec<Color>,
    pub width: usize,
    pub height: usize,
}

impl Image {
    pub fn get(&self, x: usize, y: usize) -> &Color {
        &self.data[y * self.width + x]
    }

    pub fn save_png(&self, path: &Path) {
        println!("Saving {}", path.display());
        let file = File::create(path)
            .expect(&format!("Unable to create file {}", path.display()));
        let mut encoder =
            png::Encoder::new(BufWriter::new(file),
                              self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()
            .expect(&format!("Unable to create file writer"));
        let png_data = (0..self.height).map(|y| {
            (0..self.width).map(|x| {
                let c = self.get(x, self.height - y - 1).to_canvas_color();
                vec![c.r, c.g, c.b]
            }).collect::<Vec<Vec<u8>>>().concat()
        }).collect::<Vec<Vec<u8>>>().concat();
        writer.write_image_data(&png_data)
            .expect(&format!("Unable to write file {}", path.display()));
    }
}
