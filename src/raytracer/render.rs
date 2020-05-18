use rand::prelude::ThreadRng;
use rand::Rng;
use rayon::prelude::*;

use crate::raytracer::camera::Camera;
use crate::raytracer::color::Color;
use crate::raytracer::image::Image;
use crate::raytracer::ray::Ray;
use crate::raytracer::vector3d::unit_vector;
use crate::raytracer::world::World;

#[inline(always)]
fn ray_color(rng: ThreadRng, ray: &Ray, world: &World, depth: usize) -> Color {
    if depth <= 0 {
        return Color { r: 0.0, g: 0.0, b: 0.0 };
    }
    let t_min = 0.001;
    let t_max = 9999999999.9;
    match world.hit(ray, t_min, t_max) {
        Some(rec) => {
            return match rec.material.scatter(rng, &ray, &rec) {
                Some((scattered, attenuation)) => {
                    attenuation * &ray_color(rng, &scattered, world, depth - 1)
                }
                None => Color { r: 0.0, g: 0.0, b: 0.0 }
            };
        }
        None => {}
    }

    let blend = 0.5 * (unit_vector(&ray.direction).y + 1.0);
    let col1 = Color { r: 1.0, g: 1.0, b: 1.0 };
    let col2 = Color { r: 1.0, g: 0.1, b: 0.2 };
    return col1 * (1.0 - blend) + &(col2 * blend);
}

pub fn render(width: usize, height: usize,
              samples_per_pixel: usize, max_depth: usize,
              world: &World, cam: &Camera) -> Image {
    let mut image = Image::new(width, height);
    let image_rows: Vec<Image> = (0..height).into_par_iter().map(|y| {
        let mut rng = rand::thread_rng();
        let mut image_row = Image::new(width, 1);
        for x in 0..width {
            let mut pixel_color = Color { r: 0.0, g: 0.0, b: 0.0 };
            for _ in 0..samples_per_pixel {
                let u = (x as f64 + rng.gen::<f64>()) / (width as f64 - 1.0);
                let v = (y as f64 + rng.gen::<f64>()) / (height as f64 - 1.0);
                let ray = cam.get_ray(rng, u, v);
                pixel_color = pixel_color + &ray_color(rng, &ray, &world, max_depth);
            }
            image_row.set(x, 0, pixel_color.sqrt() / (samples_per_pixel as f64).sqrt());
        }
        image_row
    }).collect();
    for y in 0..height {
        for x in 0..width {
            image.set(x, y, *image_rows[y].get(x, 0));
        }
    }
    return image;
}
