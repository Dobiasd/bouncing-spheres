
use std::ops::{Add, Div, Mul, Sub};
use std::prelude::v1::Vec;
use std::time::SystemTime;

use pixel_canvas::{Canvas, Color, input::MouseState};
use rand::prelude::*;

#[derive(Copy, Clone)]
pub struct Vector3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3d {
    pub fn length_squared(&self) -> f64 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }
    pub fn length(&self) -> f64 {
        return self.length_squared().sqrt();
    }
}

impl Add<&Vector3d> for Vector3d {
    type Output = Vector3d;
    fn add(self, other: &Vector3d) -> Vector3d {
        Vector3d { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl Sub<&Vector3d> for Vector3d {
    type Output = Vector3d;
    fn sub(self, other: &Vector3d) -> Vector3d {
        Vector3d { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl Mul<f64> for Vector3d {
    type Output = Vector3d;
    fn mul(self, t: f64) -> Vector3d {
        return Vector3d { x: self.x * t, y: self.y * t, z: self.z * t };
    }
}

impl Div<f64> for Vector3d {
    type Output = Vector3d;
    fn div(self, t: f64) -> Vector3d {
        return Vector3d { x: self.x / t, y: self.y / t, z: self.z / t };
    }
}

pub fn dot(a: &Vector3d, b: &Vector3d) -> f64 {
    return a.x * b.x + a.y * b.y + a.z * b.z;
}

fn cross(a: &Vector3d, b: &Vector3d) -> Vector3d {
    return Vector3d {
        x: a.y * b.z - a.z - b.y,
        y: a.z * b.x - a.x - b.z,
        z: a.x * b.y - a.y - b.x,
    };
}

fn unit_vector(v: &Vector3d) -> Vector3d {
    return *v / v.length();
}

