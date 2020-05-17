use std::ops::{Add, Div, Mul, Sub};
use std::prelude::v1::Vec;
use std::time::SystemTime;

use pixel_canvas::{Canvas, Color, input::MouseState};
use rand::prelude::*;
use crate::raytracer::vector3d;
use crate::raytracer::ray;


pub struct Camera {
    origin: vector3d::Vector3d,
    lower_left_corner: vector3d::Vector3d,
    horizontal: vector3d::Vector3d,
    vertical: vector3d::Vector3d,
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> ray::Ray {
        return ray::Ray { origin: self.origin, direction: self.lower_left_corner + &(self.horizontal * u) + &(self.vertical * v) - &self.origin };
    }
}

impl Camera {
    pub fn new(aspect_ratio: f64, viewport_height: f64, focal_length: f64) -> Camera {
        let viewport_width = aspect_ratio * viewport_height;
        let origin = vector3d::Vector3d { x: 0.0, y: 0.0, z: 0.0 };
        let horizontal = vector3d::Vector3d { x: viewport_width, y: 0.0, z: 0.0 };
        let vertical = vector3d::Vector3d { x: 0.0, y: viewport_height, z: 0.0 };
        return Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin - &(horizontal / 2.0) - &(vertical / 2.0) - &vector3d::Vector3d { x: 0.0, y: 0.0, z: focal_length },
        };
    }
}


