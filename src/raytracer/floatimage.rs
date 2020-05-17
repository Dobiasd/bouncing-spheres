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
}
