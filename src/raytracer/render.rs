use rand::{Rng, SeedableRng};
use rand::prelude::StdRng;
use rayon::prelude::*;

use crate::raytracer::camera::{CameraRange, get_ray_camera_blend};
use crate::raytracer::color::{blend_colors, Color};
use crate::raytracer::image::Image;
use crate::raytracer::ray::Ray;
use crate::raytracer::vector3d::unit_vector;
use crate::raytracer::world::World;

pub struct Sky {
    pub col1: Color,
    pub col2: Color,
}

#[inline(always)]
fn ray_color(rng: &mut StdRng, ray: &Ray, world: &World,
             depth: usize, sky: &Sky) -> Color {
    if depth <= 0 {
        return Color::black();
    }
    let t_min = 0.001;
    let t_max = 9999999999.9;
    match world.hit(ray, t_min, t_max) {
        Some(rec) => {
            return match rec.material.scatter(rng, &ray, &rec) {
                Some((scattered, attenuation)) => {
                    attenuation * &ray_color(rng, &scattered,
                                             world, depth - 1, sky)
                }
                None => Color::black()
            };
        }
        None => {}
    }

    blend_colors(&sky.col2, &sky.col1, 0.5 * (unit_vector(&ray.direction).y + 1.0))
}

pub fn render(width: usize, height: usize,
              samples_per_pixel: usize, max_depth: usize,
              world: &World, cams: &CameraRange, sky: &Sky) -> Image {
    Image {
        data: (0..height).into_par_iter().map(|y| {
            let mut rng: StdRng = SeedableRng::seed_from_u64(y as u64);
            (0..width).map(|x| {
                (0..samples_per_pixel).map(|_| {
                    let horizontal_fraction = (x as f64 + rng.gen::<f64>()) / (width as f64 - 1.0);
                    let vertical_fraction = (y as f64 + rng.gen::<f64>()) / (height as f64 - 1.0);
                    let ray = get_ray_camera_blend(
                        &mut rng, horizontal_fraction, vertical_fraction, cams);
                    ray_color(&mut rng, &ray, &world, max_depth, sky)
                }).fold(Color::black(),
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
