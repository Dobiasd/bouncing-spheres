use core::fmt;
use std::f64::consts::PI;
use std::ops::{Add, Div, Mul, Sub};

use rand::prelude::ThreadRng;
use rand::Rng;

#[derive(Copy, Clone)]
pub struct Vector3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// Similarly, implement `Display` for `Point2D`
impl fmt::Display for Vector3d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}, z: {}", self.x, self.y, self.z)
    }
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

impl Mul<&Vector3d> for Vector3d {
    type Output = Vector3d;
    fn mul(self, other: &Vector3d) -> Vector3d {
        return Vector3d { x: self.x * other.x, y: self.y * other.y, z: self.z * other.z };
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

pub fn cross(a: &Vector3d, b: &Vector3d) -> Vector3d {
    return Vector3d {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    };
}

pub fn unit_vector(v: &Vector3d) -> Vector3d {
    return *v / v.length();
}

pub fn random_vector3d_0_to_1(mut rng: ThreadRng) -> Vector3d {
    return Vector3d {
        x: rng.gen::<f64>(),
        y: rng.gen::<f64>(),
        z: rng.gen::<f64>(),
    };
}

pub fn random_vector3d(mut rng: ThreadRng, min: f64, max: f64) -> Vector3d {
    return Vector3d {
        x: rng.gen_range(min, max),
        y: rng.gen_range(min, max),
        z: rng.gen_range(min, max),
    };
}

pub fn random_unit_vector(mut rng: ThreadRng) -> Vector3d {
    let a = rng.gen_range(0.0, 2.0 * PI);
    let z = rng.gen_range(-1.0 as f64, 1.0 as f64);
    let r = (1.0 - z * z).sqrt();
    return Vector3d {
        x: r * a.cos(),
        y: r * a.sin(),
        z,
    };
}

pub fn random_in_unit_sphere(mut rng: ThreadRng) -> Vector3d {
    let mut p = random_vector3d(rng, -1.0, 1.0);
    while p.length_squared() >= 1.0 {
        p = random_vector3d(rng, -1.0, 1.0);
    }
    return p;
}

pub fn random_in_hemisphere(mut rng: ThreadRng, normal: &Vector3d) -> Vector3d {
    let in_unit_sphere = random_in_unit_sphere(rng);
    return if dot(&in_unit_sphere, normal) > 0.0 {
        in_unit_sphere
    } else {
        in_unit_sphere * -1.0
    };
}

pub fn reflect(v: &Vector3d, n: &Vector3d) -> Vector3d {
    return *v - &(((*n * dot(v, n))) * 2.0 as f64);
}

pub fn random_in_unit_disk(mut rng: ThreadRng) -> Vector3d {
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
    return p;
}