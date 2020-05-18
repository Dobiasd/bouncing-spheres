use rand::prelude::ThreadRng;
use rand::Rng;

use crate::raytracer::camera::Camera;
use crate::raytracer::color::Color;
use crate::raytracer::image::Image;
use crate::raytracer::ray::Ray;
use crate::raytracer::vector3d::unit_vector;
use crate::raytracer::world::World;

fn ray_color(rng: ThreadRng, r: &Ray, world: &World, depth: usize) -> Color {
    if depth <= 0 {
        return Color { r: 0.0, g: 0.0, b: 0.0 };
    }
    let t_min = 0.001;
    let t_max = 99999999.9;
    match world.hit(r, t_min, t_max) {
        Some(rec) => {
            return match rec.material.scatter(rng, &r, &rec) {
                Some((scattered, attenuation)) => {
                    attenuation * &ray_color(rng, &scattered, world, depth - 1)
                }
                None => Color { r: 0.0, g: 0.0, b: 0.0 }
            };
        }
        None => {}
    }
    // todo: make nice evening background
    let unit_direction = unit_vector(&r.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    let col1 = Color { r: 1.0, g: 1.0, b: 1.0 };
    let col2 = Color { r: 0.5, g: 0.7, b: 1.0 };
    //let col1 = Color { r: 0.6, g: 0.5, b: 0.2 };
    //let col2 = Color { r: 0.05, g: 0.05, b: 0.1 };
    return col1 * (1.0 - t) + &(col2 * t);
}

pub fn render(mut rng: ThreadRng, width: usize, height: usize,
              samples_per_pixel: usize, max_depth: usize,
              world: &World, cam: &Camera) -> Image {
    let mut image = Image::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let mut pixel_color = Color { r: 0.0, g: 0.0, b: 0.0 };
            for _ in 0..samples_per_pixel {
                let u = (x as f64 + rng.gen::<f64>()) / (width as f64 - 1.0);
                let v = (y as f64 + rng.gen::<f64>()) / (height as f64 - 1.0);
                let r = cam.get_ray(rng, u, v);
                pixel_color = pixel_color + &ray_color(rng, &r, &world, max_depth);
            }
            image.set(x, y, pixel_color.sqrt() / (samples_per_pixel as f64).sqrt());
        }
    }
    return image;
}
