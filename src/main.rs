extern crate chrono;

use std::fs;
use std::ops::{Add, Div};
use std::path::Path;
use std::prelude::v1::Vec;
use std::time::SystemTime;

use chrono::DateTime;
use chrono::offset::Utc;
use pixel_canvas::Canvas;
use rand::prelude::*;

use crate::raytracer::camera::Camera;
use crate::raytracer::color::Color;
use crate::raytracer::material::Material;
use crate::raytracer::sphere::Sphere;
use crate::raytracer::vector3d::Vector3d;
use crate::raytracer::world::World;

mod raytracer;

fn random_sphere(mut rng: ThreadRng) -> Sphere {
    let min = -24.0;
    let max = 24.0;
    let radius = 0.4 + 1.8 * rng.gen_range(-12.0 as f64, 2.0 as f64).tanh().add(1.0).div(2.0);
    Sphere {
        center: Vector3d {
            x: rng.gen_range(min, max),
            y: radius,
            z: rng.gen_range(min, max),
        },
        radius,
        material: Material {
            albedo: Color {
                r: rng.gen_range(0.0, 1.0),
                g: rng.gen_range(0.0, 1.0),
                b: rng.gen_range(0.0, 1.0),
            },
            reflectiveness: (rng.gen_range(0.0, 3.0) as f64).min(1.0),
            reflection_fuzz: (rng.gen_range(-3.0, 1.0) as f64).max(0.0),
        },
    }
}

fn make_world(rng: ThreadRng) -> World {
    let radius_planet = 6371.0;
    let planet = Sphere {
        center: Vector3d { x: 0.0, y: -radius_planet, z: 0.0 },
        radius: radius_planet,
        material: Material { albedo: Color { r: 0.5, g: 0.5, b: 0.5 }, reflectiveness: 1.0, reflection_fuzz: 0.0 },
    };
    let mut objects = (0..200).map(|_| random_sphere(rng)).collect::<Vec<Sphere>>();
    objects.extend(vec![planet]);
    World {
        spheres: objects
    }
}

fn cam(width: usize, height: usize, t: f64) -> Camera {
    let dist = 12.5;
    let position = Vector3d {
        x: dist * (0.11 * t).sin(),
        y: 5.0 + 4.999 * (0.4 * t).cos(),
        z: dist * (0.17 * t).cos(),
    };
    let looks_at = Vector3d {
        x: 5.3 * (0.21 * t).cos(),
        y: 2.3 * (0.34 * t).cos(),
        z: 5.3 * (0.41 * t).cos(),
    };
    let up_direction = Vector3d { x: 0.0, y: 1.0, z: 0.0 };
    let dist_to_focus = (position - &looks_at).length();
    let aperture = 0.0;
    let aspect_ratio = width as f64 / height as f64;
    return Camera::new(&position, &looks_at, &up_direction, 90.0, aspect_ratio, aperture, dist_to_focus);
}

fn main() {
    //let pixel_scale = 1;
    //let samples_per_pixel = 1024;
    //let max_depth = 64;

    let pixel_scale = 4;
    let samples_per_pixel = 4;
    let max_depth = 4;

    let speed = 2.3;
    let t_step = speed * 1.0 / 60.0;

    let rng = rand::thread_rng();
    let world = make_world(rng);

    let canvas = Canvas::new(1280, 720)
        .title("raytracer");

    let start_time = SystemTime::now();
    let datetime: DateTime<Utc> = start_time.into();

    let dir_path_str = format!("./images/{}/", datetime.format("%Y-%m-%dT%TZ"));
    fs::create_dir_all(Path::new(&dir_path_str)).expect("wat");

    let mut t = 0.0;
    let mut frame_num = 0;
    canvas.render(move |_, image| {
        t += t_step;
        let width = image.width() as usize;
        let height = image.height() as usize;
        let pixels = raytracer::render::render(
            width / pixel_scale, height / pixel_scale,
            samples_per_pixel, max_depth, &world, &cam(width, height, t));
        for (y, row) in image.chunks_mut(width).enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                *pixel = pixels.get(x / pixel_scale, y / pixel_scale).to_canvas_color()
            }
        }
        pixels.save_png(&Path::new(&dir_path_str).join(format!("{:08}.png", frame_num)));
        frame_num += 1;
    });
}
