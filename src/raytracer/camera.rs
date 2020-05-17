use crate::raytracer::ray::Ray;
use crate::raytracer::vector3d::Vector3d;

pub struct Camera {
    origin: Vector3d,
    lower_left_corner: Vector3d,
    horizontal: Vector3d,
    vertical: Vector3d,
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        return Ray {
            origin: self.origin,
            direction: self.lower_left_corner + &(self.horizontal * u) + &(self.vertical * v) - &self.origin,
        };
    }
}

impl Camera {
    pub fn new(aspect_ratio: f64, viewport_height: f64, focal_length: f64) -> Camera {
        let viewport_width = aspect_ratio * viewport_height;
        let origin = Vector3d { x: 0.0, y: 0.0, z: 0.0 };
        let horizontal = Vector3d { x: viewport_width, y: 0.0, z: 0.0 };
        let vertical = Vector3d { x: 0.0, y: viewport_height, z: 0.0 };
        return Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin - &(horizontal / 2.0) - &(vertical / 2.0) - &Vector3d { x: 0.0, y: 0.0, z: focal_length },
        };
    }
}


