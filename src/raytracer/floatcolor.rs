use std::ops::{Add, Div, Mul, Sub};
use std::prelude::v1::Vec;
use std::time::SystemTime;

#[derive(Copy, Clone)]
pub struct FloatColor {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Mul<f64> for FloatColor {
    type Output = FloatColor;
    fn mul(self, f: f64) -> FloatColor {
        return FloatColor { r: self.r * f, g: self.g * f, b: self.b * f };
    }
}

impl Div<f64> for FloatColor {
    type Output = FloatColor;
    fn div(self, f: f64) -> FloatColor {
        return FloatColor { r: self.r / f, g: self.g / f, b: self.b / f };
    }
}

impl Add<&FloatColor> for FloatColor {
    type Output = FloatColor;
    fn add(self, other: &FloatColor) -> FloatColor {
        return FloatColor { r: self.r + other.r, g: self.g + other.g, b: self.b + other.b };
    }
}

