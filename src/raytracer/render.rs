use rand::prelude::ThreadRng;
use rand::Rng;

use crate::raytracer::camera::Camera;
use crate::raytracer::floatcolor::FloatColor;
use crate::raytracer::floatimage::FloatImage;
use crate::raytracer::ray::Ray;
use crate::raytracer::sphere::{HittableSpheres, Sphere};
use crate::raytracer::vector3d::{random_in_hemisphere, unit_vector, Vector3d};

fn ray_color(mut rng: ThreadRng, r: &Ray, spheres: &HittableSpheres, depth: usize) -> FloatColor {
    if depth <= 0 {
        return FloatColor { r: 0.0, g: 0.0, b: 0.0 };
    }
    let t_min = 0.001;
    let t_max = 99999999.9;
    match spheres.hit(r, t_min, t_max) {
        Some(rec) => {
            let target = rec.p + &random_in_hemisphere(rng, &rec.normal);
            return ray_color(rng, &Ray { origin: rec.p, direction: target - &rec.p }, &spheres, depth - 1) * 0.5;
            return FloatColor {
                r: rec.normal.x + 1.0,
                g: rec.normal.y + 1.0,
                b: rec.normal.z + 1.0,
            } * 0.5;
        }
        None => {}
    }
    let unit_direction = unit_vector(&r.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    let col1 = FloatColor { r: 1.0, g: 1.0, b: 1.0 };
    let col2 = FloatColor { r: 0.5, g: 0.7, b: 1.0 };
    return col1 * (1.0 - t) + &(col2 * t);
}

pub fn render(mut rng: ThreadRng, width: usize, height: usize) -> FloatImage {
    let mut image = FloatImage::new(width, height);

    let world = HittableSpheres {
        spheres: vec![
            Sphere { center: Vector3d { x: 0.0, y: 0.0, z: -1.0 }, radius: 0.5 },
            Sphere { center: Vector3d { x: 0.0, y: -100.5, z: -1.0 }, radius: 100.0 }]
    };

    let cam = Camera::new(16.0 / 9.0, 2.0, 1.0);

    let samples_per_pixel = 100;
    let max_depth = 50;

    for y in 0..height {
        for x in 0..width {
            let mut pixel_color = FloatColor { r: 0.0, g: 0.0, b: 0.0 };
            for _ in 0..samples_per_pixel {
                let u = (x as f64 + rng.gen::<f64>()) / (width as f64 - 1.0);
                let v = (y as f64 + rng.gen::<f64>()) / (height as f64 - 1.0);
                let r = cam.get_ray(u, v);
                pixel_color = pixel_color + &ray_color(rng, &r, &world, max_depth);
            }
            image.set(x, y, pixel_color / samples_per_pixel as f64);
        }
    }
    return image;
}

