use std::ops::{Add, Div, Mul};

#[derive(Copy, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn sqrt(&self) -> Color {
        Color {
            r: self.r.sqrt(),
            g: self.g.sqrt(),
            b: self.b.sqrt(),
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, f: f64) -> Color {
        return Color { r: self.r * f, g: self.g * f, b: self.b * f };
    }
}

impl Mul<&Color> for Color {
    type Output = Color;
    fn mul(self, other: &Color) -> Color {
        return Color { r: self.r * other.r, g: self.g * other.g, b: self.b * other.b };
    }
}

impl Div<f64> for Color {
    type Output = Color;
    fn div(self, f: f64) -> Color {
        return Color { r: self.r / f, g: self.g / f, b: self.b / f };
    }
}

impl Add<&Color> for Color {
    type Output = Color;
    fn add(self, other: &Color) -> Color {
        return Color { r: self.r + other.r, g: self.g + other.g, b: self.b + other.b };
    }
}