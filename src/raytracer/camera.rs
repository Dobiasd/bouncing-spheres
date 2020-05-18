use rand::prelude::ThreadRng;

use crate::raytracer::ray::Ray;
use crate::raytracer::vector3d::{cross, random_in_unit_disk, unit_vector, Vector3d};

pub struct Camera {
    origin: Vector3d,
    lower_left_corner: Vector3d,
    horizontal: Vector3d,
    vertical: Vector3d,
    u: Vector3d,
    v: Vector3d,
    //w: Vector3d,
    lens_radius: f64,
}

impl Camera {
    pub fn get_ray(&self,  rng: ThreadRng, s: f64, t: f64) -> Ray {
        let rd = random_in_unit_disk(rng) * self.lens_radius;
        let offset = self.u * rd.x + &(self.v * rd.y);
        return Ray {
            origin: self.origin + &offset,
            direction: self.lower_left_corner + &(self.horizontal * s) + &(self.vertical * t) - &self.origin - &offset,
        };
    }
}

impl Camera {
    pub fn new(
        lookfrom: &Vector3d,
        lookat: &Vector3d,
        vup: &Vector3d,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(&((*lookfrom) - lookat));
        let u = unit_vector(&cross(&vup, &w));
        let v = cross(&w, &u);

        let origin = *lookfrom;

        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = origin - &(horizontal / 2.0) - &(vertical / 2.0) - &(w * focus_dist);

        let lens_radius = aperture / 2.0;

        return Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            //w,
            lens_radius,
        };
    }
}


