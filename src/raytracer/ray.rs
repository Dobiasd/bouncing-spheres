use std::ops::{Add, Div, Mul, Sub};
use std::prelude::v1::Vec;
use std::time::SystemTime;

use crate::raytracer::vector3d::Vector3d;

pub struct Ray {
    pub origin: Vector3d,
    pub direction: Vector3d,
}

impl Ray {
    pub fn at(&self, t: f64) -> Vector3d {
        return self.origin + &(self.direction * t);
    }
}

