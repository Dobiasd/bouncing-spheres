use std::ops::{Add, Div, Mul};

use pixel_canvas::Color as CanvasColor;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn sqrt_gamma_correct(&self) -> Color {
        Color {
            r: self.r.sqrt(),
            g: self.g.sqrt(),
            b: self.b.sqrt(),
        }
    }

    pub fn to_canvas_color(&self) -> CanvasColor {
        let r = (self.r.max(0.0).min(1.0) * 255.0) as u8;
        let g = (self.g.max(0.0).min(1.0) * 255.0) as u8;
        let b = (self.b.max(0.0).min(1.0) * 255.0) as u8;
        CanvasColor { r, g, b }
    }
}

impl Mul<f64> for Color {
    type Output = Color;
    #[inline(always)]
    fn mul(self, f: f64) -> Color {
        Color { r: self.r * f, g: self.g * f, b: self.b * f }
    }
}

impl Mul<&Color> for Color {
    type Output = Color;
    #[inline(always)]
    fn mul(self, other: &Color) -> Color {
        Color { r: self.r * other.r, g: self.g * other.g, b: self.b * other.b }
    }
}

impl Div<f64> for Color {
    type Output = Color;
    #[inline(always)]
    fn div(self, f: f64) -> Color {
        Color { r: self.r / f, g: self.g / f, b: self.b / f }
    }
}

impl Add<&Color> for Color {
    type Output = Color;
    #[inline(always)]
    fn add(self, other: &Color) -> Color {
        Color { r: self.r + other.r, g: self.g + other.g, b: self.b + other.b }
    }
}

pub fn black() -> Color {
    Color { r: 0.0, g: 0.0, b: 0.0 }
}

pub fn blend_colors(col1: &Color, col2: &Color, factor_col1: f64) -> Color {
    *col1 * factor_col1 + &(*col2 * (1.0 - factor_col1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blend() {
        let result = blend_colors(
            &Color { r: 1.0, g: 0.0, b: 0.0 },
            &Color { r: 0.0, g: 1.0, b: 0.0 },
            0.3
        );
        assert_eq!(result, Color { r: 0.3, g: 0.7, b: 0.0 });
    }
}