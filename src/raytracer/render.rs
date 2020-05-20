use rand::{Rng, SeedableRng};
use rand::prelude::StdRng;
use rayon::prelude::*;

use crate::raytracer::camera::{Camera, get_ray_camera_blend};
use crate::raytracer::color::Color;
use crate::raytracer::image::Image;
use crate::raytracer::ray::Ray;
use crate::raytracer::vector3d::unit_vector;
use crate::raytracer::world::World;

#[inline(always)]
fn ray_color(rng: &mut StdRng, ray: &Ray, world: &World, depth: usize, sky_factor: f64) -> Color {
    if depth <= 0 {
        return Color { r: 0.0, g: 0.0, b: 0.0 };
    }
    let t_min = 0.001;
    let t_max = 9999999999.9;
    match world.hit(ray, t_min, t_max) {
        Some(rec) => {
            return match rec.material.scatter(rng, &ray, &rec) {
                Some((scattered, attenuation)) => {
                    attenuation * &ray_color(rng, &scattered, world, depth - 1, sky_factor)
                }
                None => Color { r: 0.0, g: 0.0, b: 0.0 }
            };
        }
        None => {}
    }
    let day1 = Color { r: 1.0, g: 1.0, b: 1.0 };
    let day2 = Color { r: 0.5, g: 0.7, b: 1.0 };
    let night1 = Color { r: 0.8, g: 0.8, b: 0.8 };
    let night2 = Color { r: 0.4, g: 0.1, b: 0.2 };
    let col1 = (night1 * sky_factor) + &(day1 * (1.0 - sky_factor));
    let col2 = (night2 * sky_factor) + &(day2 * (1.0 - sky_factor));
    let blend = 0.5 * (unit_vector(&ray.direction).y + 1.0);
    col1 * (1.0 - blend) + &(col2 * blend)
}

pub fn render(width: usize, height: usize,
              samples_per_pixel: usize, max_depth: usize,
              world: &World, cam: &Camera, cam_old: &Camera, sky_factor: f64) -> Image {
    Image {
        data: (0..height).into_par_iter().map(|y| {
            let mut rng: StdRng = SeedableRng::seed_from_u64(y as u64);
            (0..width).map(|x| {
                (0..samples_per_pixel).map(|_| {
                    let s = (x as f64 + rng.gen::<f64>()) / (width as f64 - 1.0);
                    let t = (y as f64 + rng.gen::<f64>()) / (height as f64 - 1.0);
                    let ray = get_ray_camera_blend(&mut rng, s, t, cam_old, cam);
                    ray_color(&mut rng, &ray, &world, max_depth, sky_factor)
                }).fold(Color { r: 0.0, g: 0.0, b: 0.0 },
                        |a: Color, b: Color| a + &b)
            }).collect()
        }).collect::<Vec<Vec<Color>>>()
            .iter()
            .flatten()
            .map(|c| *c / samples_per_pixel as f64)
            .map(|c| c.sqrt_gamma_correct())
            .collect(),
        width,
        height,
    }
}
