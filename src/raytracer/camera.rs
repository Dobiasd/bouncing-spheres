use rand::prelude::ThreadRng;

use crate::raytracer::ray::Ray;
use crate::raytracer::vector3d::{cross, random_in_unit_disk, unit_vector, Vector3d};

pub struct Camera {
    origin: Vector3d,
    lower_left_corner: Vector3d,
    horizontal: Vector3d,
    vertical: Vector3d,
    lens_radius: f64,
    u: Vector3d,
    v: Vector3d,
    #[allow(dead_code)]
    w: Vector3d,
}

impl Camera {
    pub fn get_ray(&self, rng: ThreadRng, s: f64, t: f64) -> Ray {
        let rd = random_in_unit_disk(rng) * self.lens_radius;
        let offset = self.u * rd.x + &(self.v * rd.y);
        Ray {
            origin: self.origin + &offset,
            direction: self.lower_left_corner + &(self.horizontal * s) + &(self.vertical * t) - &self.origin - &offset,
        }
    }
}

impl Camera {
    pub fn new(
        position: &Vector3d,
        looks_at: &Vector3d,
        up_direction: &Vector3d,
        vertical_field_of_view: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = vertical_field_of_view.to_radians();
        let viewport_height = 2.0 * (theta / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(&((*position) - looks_at));
        let u = unit_vector(&cross(&up_direction, &w));
        let v = cross(&w, &u);

        let origin = *position;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = origin - &(horizontal / 2.0) - &(vertical / 2.0) - &(w * focus_dist);
        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            lens_radius,
            u,
            v,
            w,
        }
    }
}


