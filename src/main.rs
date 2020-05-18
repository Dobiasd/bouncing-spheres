extern crate chrono;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::fs;
use std::fs::File;
use std::io::Read;
use std::ops::{Add, Div, Mul};
use std::path::Path;
use std::prelude::v1::Vec;
use std::process::Command;
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
        x: 7.3 * (0.21 * t).cos(),
        y: 2.3 * (0.34 * t).cos(),
        z: 7.3 * (0.41 * t).cos(),
    };
    let up_direction = Vector3d { x: 0.0, y: 1.0, z: 0.0 };
    let dist_to_focus = (position - &looks_at).length();
    let aperture = 0.28;
    let aspect_ratio = width as f64 / height as f64;
    Camera::new(&position, &looks_at, &up_direction, 90.0, aspect_ratio, aperture, dist_to_focus)
}


fn create_video(dir_path_str: &str) {
    let video_path = format!("{}.mp4", dir_path_str);
    println!("Saving {}", video_path);
    let output = Command::new("ffmpeg")
        .arg("-i")
        .arg(format!("{}/%08d.png", dir_path_str))
        .arg("-c:v")
        .arg("libx264")
        .arg("-preset")
        .arg("slow")
        .arg("-profile:v")
        .arg("high")
        .arg("-crf")
        .arg("18")
        .arg("-coder")
        .arg("1")
        .arg("-pix_fmt")
        .arg("yuv420p")
        .arg("-movflags")
        .arg("+faststart")
        .arg("-g")
        .arg("60")
        .arg("-bf")
        .arg("2")
        .arg("-y")
        .arg(video_path)
        .output()
        .expect("failed to execute ffmpeg");
    println!("{}", output.status);
    println!("{}", std::str::from_utf8(&output.stderr[..]).expect("wat"));
    println!("{}", std::str::from_utf8(&output.stdout[..]).expect("wat"));
}

#[derive(Debug, Deserialize)]
struct Config {
    resolution_x: usize,
    resolution_y: usize,
    samples_per_pixel: usize,
    max_depth: usize,
    display_scale_factor: usize,
    speed: f64,
    num_frames: usize,
}

fn init() -> Config {
    let config_path = "raytracer.toml";
    let mut config_file_content = String::new();
    File::open(config_path).and_then(|mut f| {
        f.read_to_string(&mut config_file_content)
    }).expect(&format!("Unable to read config file: {}", config_path));
    toml::from_str(&config_file_content).expect(&format!("Unable to parse config file: {}", config_path))
}

fn main() {
    let config = init();

    let rng = rand::thread_rng();
    let world = make_world(rng);

    let window_width = config.resolution_x * config.display_scale_factor;
    let window_height = config.resolution_y * config.display_scale_factor;
    let canvas = Canvas::new(window_width, window_height).title("raytracer");

    let start_time = SystemTime::now();
    let datetime: DateTime<Utc> = start_time.into();

    let dir_path_str = format!("./output/{}", datetime.format("%Y-%m-%d_%H-%M-%S"));
    fs::create_dir_all(Path::new(&dir_path_str)).expect("wat");

    let mut t = 0.0;
    let mut frame_num = 0;
    canvas.render(move |_, image| {
        t += config.speed / 60.0;
        let sky_factor = 0.3 + 0.7 * t.mul(0.2).cos();
        let pixels = raytracer::render::render(
            image.width() / config.display_scale_factor,
            image.height() / config.display_scale_factor,
            config.samples_per_pixel, config.max_depth, &world,
            &cam(image.width(), image.height(), t),
            sky_factor);
        let width = image.width();
        for (y, row) in image.chunks_mut(width).enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                *pixel = pixels.get(
                    x / config.display_scale_factor,
                    y / config.display_scale_factor,
                ).to_canvas_color()
            }
        }
        pixels.save_png(&Path::new(&dir_path_str).join(format!("{:08}.png", frame_num)));
        frame_num += 1;
        if frame_num >= config.num_frames {
            create_video(&dir_path_str);
            std::process::exit(0);
        }
    });
}
