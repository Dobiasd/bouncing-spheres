use rand::prelude::StdRng;
use rand::Rng;

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

pub struct CameraRange {
    pub cam_a: Camera,
    pub cam_b: Camera,
}

pub fn get_ray_camera_blend(rng: &mut StdRng,
                            horizontal_fraction: f64, vertical_fraction: f64,
                            cams: &CameraRange) -> Ray {
    let frame_time = rng.gen_range(0.0_f64, 1.0_f64);
    let rd = random_in_unit_disk(rng) *
        (frame_time * cams.cam_b.lens_radius + (1.0 - frame_time) * cams.cam_a.lens_radius);
    let ray_a = cams.cam_a.get_ray_rd(horizontal_fraction, vertical_fraction, rd, frame_time);
    let ray_b = cams.cam_b.get_ray_rd(horizontal_fraction, vertical_fraction, rd, frame_time);
    Ray {
        origin: (ray_b.origin * frame_time + &(ray_a.origin * (1.0 - frame_time))),
        direction: (ray_b.direction * frame_time + &(ray_a.direction * (1.0 - frame_time))),
        frame_time,
    }
}

impl Camera {
    pub fn get_ray_rd(&self, horizontal_fraction: f64, vertical_fraction: f64,
                      rd: Vector3d, frame_time: f64) -> Ray {
        let offset = self.u * rd.x + &(self.v * rd.y);
        Ray {
            origin: self.origin + &offset,
            direction: self.lower_left_corner +
                &(self.horizontal * horizontal_fraction) +
                &(self.vertical * vertical_fraction) - &self.origin - &offset,
            frame_time,
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
        let lower_left_corner =
            origin - &(horizontal / 2.0) - &(vertical / 2.0) - &(w * focus_dist);
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


