use std::ops::{Add, Div, Mul, Sub};
use std::path::Path;
use std::prelude::v1::Vec;
use std::time::{SystemTime, UNIX_EPOCH};

use pixel_canvas::{Canvas, Color, input::MouseState};
use rand::prelude::*;

use raytracer::*;

use crate::raytracer::camera::Camera;
use crate::raytracer::floatcolor::FloatColor;
use crate::raytracer::material::Material;
use crate::raytracer::sphere::{HittableSpheres, Sphere};
use crate::raytracer::vector3d::Vector3d;

mod raytracer;

fn random_sphere(mut rng: ThreadRng) -> Sphere {
    let min = -10.0;
    let max = 10.0;
    let radius = rng.gen_range(0.5, 3.0);
    Sphere {
        center: Vector3d {
            x: rng.gen_range(min, max),
            y: radius + rng.gen_range(0.0, 2.0),
            z: rng.gen_range(min, max),
        },
        radius,
        material: Material {
            albedo: FloatColor {
                r: rng.gen_range(0.0, 1.0),
                g: rng.gen_range(0.0, 1.0),
                b: rng.gen_range(0.0, 1.0),
            },
            reflectiveness: rng.gen_range(0.0, 1.0),
            fuzz: rng.gen_range(0.0, 1.0),
        },
    }
}

fn make_world(mut rng: ThreadRng) -> HittableSpheres {
    let planet = Sphere {
        center: Vector3d { x: 0.0, y: -200.0, z: 0.0 },
        radius: 200.0,
        material: Material { albedo: FloatColor { r: 0.5, g: 0.7, b: 0.2 }, reflectiveness: 0.0, fuzz: 0.0 },
    };
    let objects = (0..32).map(|x| random_sphere(rng)).collect::<Vec<Sphere>>();
    HittableSpheres {
        spheres: [&objects[..], &vec![planet][..]].concat()
    }
}

fn cam(width: usize, height: usize, t: f64) -> Camera {
    let speed = 0.5;
    let dist = 12.5;
    let lookfrom = Vector3d {
        x: dist * (speed * t).sin(),
        y: 5.0 + 4.5 * (0.4 * t).cos(),
        z: dist * (speed * t).cos(),
    };
    let lookat = Vector3d {
        x: 1.3 * (0.2 * t).cos(),
        y: 1.3 * (0.34 * t).cos(),
        z: 1.3 * (0.41 * t).cos(),
    };
    let vup = Vector3d { x: 0.0, y: 1.0, z: 0.0 };
    let dist_to_focus = (lookfrom - &lookat).length();
    let aperture = 0.0;
    let aspect_ratio = width as f64 / height as f64;
    return Camera::new(&lookfrom, &lookat, &vup, 90.0, aspect_ratio, aperture, dist_to_focus);
}

// todo: spheres have dark border. Is this right?
fn main() {
    let pixel_scale = 2;
    let samples_per_pixel = 1024;
    let max_depth = 64;

    let canvas = Canvas::new(1280, 720)
        .title("raytracer")
        .state(MouseState::new())
        .input(MouseState::handle_input);
    let mut rng = rand::thread_rng();
    let start_time = SystemTime::now();
    let mut last_frame_start_time = start_time;
    let world = make_world(rng);
    canvas.render(move |mouse, image| {
        let t = start_time.elapsed().expect("wat").as_millis() as f64 / 1000.0;
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
                *pixel = Color { r, g, b }
            }
        }
        let frame_done_time = SystemTime::now();
        let elapsed = frame_done_time.duration_since(last_frame_start_time);
        last_frame_start_time = frame_done_time;

        let since_the_epoch = frame_done_time.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let time_format = format!("{:?}", since_the_epoch);
        let path_str = format!("./images/{}.png", time_format);
        let path = Path::new(&path_str);
        pixels.save_png(path);
        println!("{}", elapsed.expect("wat").as_millis());
    });
}
