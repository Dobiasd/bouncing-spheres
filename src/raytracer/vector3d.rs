use core::fmt;
use std::f64::consts::PI;
use std::ops::{Add, Div, Mul, Sub};

use rand::prelude::StdRng;
use rand::Rng;

#[derive(Copy, Clone)]
pub struct Vector3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl fmt::Display for Vector3d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}, z: {}", self.x, self.y, self.z)
    }
}

impl Vector3d {
    #[inline(always)]
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}

impl Add<&Vector3d> for Vector3d {
    type Output = Vector3d;
    #[inline(always)]
    fn add(self, other: &Vector3d) -> Vector3d {
        Vector3d { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl Sub<&Vector3d> for Vector3d {
    type Output = Vector3d;
    #[inline(always)]
    fn sub(self, other: &Vector3d) -> Vector3d {
        Vector3d { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl Mul<f64> for Vector3d {
    type Output = Vector3d;
    #[inline(always)]
    fn mul(self, t: f64) -> Vector3d {
        Vector3d { x: self.x * t, y: self.y * t, z: self.z * t }
    }
}

impl Mul<&Vector3d> for Vector3d {
    type Output = Vector3d;
    #[inline(always)]
    fn mul(self, other: &Vector3d) -> Vector3d {
        Vector3d { x: self.x * other.x, y: self.y * other.y, z: self.z * other.z }
    }
}

impl Div<f64> for Vector3d {
    type Output = Vector3d;
    #[inline(always)]
    fn div(self, t: f64) -> Vector3d {
        Vector3d { x: self.x / t, y: self.y / t, z: self.z / t }
    }
}

#[inline(always)]
pub fn dot(a: &Vector3d, b: &Vector3d) -> f64 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

#[inline(always)]
pub fn cross(a: &Vector3d, b: &Vector3d) -> Vector3d {
    Vector3d {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    }
}

#[inline(always)]
pub fn unit_vector(v: &Vector3d) -> Vector3d {
    *v / v.length()
}

#[inline(always)]
pub fn random_unit_vector(rng: &mut StdRng) -> Vector3d {
    let a = rng.gen_range(0.0, 2.0 * PI);
    let z = rng.gen_range(-1.0_f64, 1.0_f64);
    let r = (1.0 - z * z).sqrt();
    Vector3d {
        x: r * a.cos(),
        y: r * a.sin(),
        z,
    }
}

#[inline(always)]
pub fn reflect(v: &Vector3d, n: &Vector3d) -> Vector3d {
    *v - &((*n * dot(v, n)) * 2.0_f64)
}

#[inline(always)]
pub fn random_in_unit_disk(rng: &mut StdRng) -> Vector3d {
    let mut p = Vector3d {
        x: rng.gen_range(-1.0, 1.0),
        y: rng.gen_range(-1.0, 1.0),
        z: 0.0,
    };
    while p.length_squared() >= 1.0 {
        p = Vector3d {
            x: rng.gen_range(-1.0, 1.0),
            y: rng.gen_range(-1.0, 1.0),
            z: 0.0,
        };
    }
    p
}


pub fn is_in_interval(x: f64, min: f64, max: f64) -> bool {
    x > min && x < max
}

pub fn zero_in(dist_to_zero: f64, x: f64) -> f64 {
    if is_in_interval(x, -dist_to_zero, dist_to_zero) {
        0.0
    } else {
        x
    }
}
