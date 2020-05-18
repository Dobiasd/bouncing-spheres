extern crate chrono;

use std::fs;
use std::path::Path;
use std::prelude::v1::Vec;
use std::time::SystemTime;

use chrono::DateTime;
use chrono::offset::Utc;
use pixel_canvas::Canvas;
use pixel_canvas::Color as CanvasColor;
use rand::prelude::*;

use crate::raytracer::camera::Camera;
use crate::raytracer::color::Color;
use crate::raytracer::material::Material;
use crate::raytracer::sphere::Sphere;
use crate::raytracer::world::World;
use crate::raytracer::vector3d::Vector3d;

mod raytracer;

fn random_sphere(mut rng: ThreadRng) -> Sphere {
    let min = -10.0;
    let max = 10.0;
    let radius = rng.gen_range(0.5, 1.5);
    Sphere {
        center: Vector3d {
            x: rng.gen_range(min, max),
            y: radius + rng.gen_range(0.0, 2.0),
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
            fuzz: (rng.gen_range(-3.0, 1.0) as f64).max(0.0),
        },
    }
}

fn make_world(rng: ThreadRng) -> World {
    let planet = Sphere {
        center: Vector3d { x: 0.0, y: -300.0, z: 0.0 },
        radius: 300.0,
        material: Material { albedo: Color { r: 0.5, g: 0.7, b: 0.2 }, reflectiveness: 0.0, fuzz: 0.0 },
    };
    let objects = (0..32).map(|_| random_sphere(rng)).collect::<Vec<Sphere>>();
    World {
        spheres: [&objects[..], &vec![planet][..]].concat()
    }
}

fn cam(width: usize, height: usize, t: f64) -> Camera {
    let speed = 0.5;
    let dist = 12.5;
    let position = Vector3d {
        x: dist * (speed * t).sin(),
        y: 5.0 + 4.5 * (0.4 * t).cos(),
        z: dist * (speed * t).cos(),
    };
    let looks_at = Vector3d {
        x: 1.3 * (0.2 * t).cos(),
        y: 1.3 * (0.34 * t).cos(),
        z: 1.3 * (0.41 * t).cos(),
    };
    let vup = Vector3d { x: 0.0, y: 1.0, z: 0.0 };
    let dist_to_focus = (position - &looks_at).length();
    let aperture = 0.0;
    let aspect_ratio = width as f64 / height as f64;
    return Camera::new(&position, &looks_at, &vup, 90.0, aspect_ratio, aperture, dist_to_focus);
}

// todo: spheres have dark border. Is this right?
// todo: multi core
fn main() {
    //let pixel_scale = 2;
    //let samples_per_pixel = 1024;
    //let max_depth = 64;

    let pixel_scale = 8;
    let samples_per_pixel = 32;
    let max_depth = 16;
    let t_step = 0.2;

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
            rng, width / pixel_scale, height / pixel_scale,
            samples_per_pixel, max_depth, &world, &cam(width, height, t));
        for (y, row) in image.chunks_mut(width).enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                let c = pixels.get(x / pixel_scale, y / pixel_scale);
                let r = (c.r.max(0.0).min(1.0) * 255.0) as u8;
                let g = (c.g.max(0.0).min(1.0) * 255.0) as u8;
                let b = (c.b.max(0.0).min(1.0) * 255.0) as u8;
                *pixel = CanvasColor { r, g, b }
            }
        }
        pixels.save_png(&Path::new(&dir_path_str).join(format!("{:08}.png", frame_num)));
        frame_num += 1;
    });
}
