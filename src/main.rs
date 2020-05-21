extern crate chrono;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::f64::consts::PI;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::ops::{Add, Div, Mul, Sub};
use std::path::Path;
use std::process::Command;
use std::time::SystemTime;

use chrono::DateTime;
use chrono::offset::Utc;
use pixel_canvas::Canvas;
use rand::prelude::StdRng;
use rand::Rng;
use rand::SeedableRng;
use uuid::Uuid;

use crate::raytracer::camera::Camera;
use crate::raytracer::color::Color;
use crate::raytracer::material::Material;
use crate::raytracer::sphere::Sphere;
use crate::raytracer::vector3d::Vector3d;
use crate::raytracer::world::World;

mod raytracer;

fn random_sphere(rng: &mut StdRng) -> Sphere {
    let min = -5.0;
    let max = 5.0;
    let max_start_y = 123.0;
    let radius = 0.4 + 1.7 *
        rng.gen_range(-10.0_f64, 0.9_f64).tanh().add(1.0).div(2.0);
    let center = Vector3d {
        x: rng.gen_range(min, max),
        y: rng.gen_range(radius, max_start_y),
        z: rng.gen_range(min, max),
    };
    Sphere {
        id: Uuid::new_v4(),
        center,
        radius,
        material: Material {
            albedo: Color {
                r: rng.gen_range(0.0, 1.0),
                g: rng.gen_range(0.0, 1.0),
                b: rng.gen_range(0.0, 1.0),
            },
            reflectiveness: 1.0 + 0.0 * rng.gen_range(0.0, 1.0),
            reflection_fuzz: 0.0 + 0.0 * rng.gen_range(0.0, 1.0),
        },
        speed: Vector3d { x: 0.0, y: 0.0, z: 0.0 },
        mass: radius.powf(3.0),
        extra_brightness: 0.0,
        center_old: center,
    }
}

fn make_world(rng: &mut StdRng) -> World {
    let radius_planet = 6371.0;
    let center = Vector3d { x: 0.0, y: -radius_planet, z: 0.0 };
    let planet = Sphere {
        id: Uuid::new_v4(),
        center,
        radius: radius_planet,
        material: Material {
            albedo: Color { r: 0.5, g: 0.5, b: 0.5 },
            reflectiveness: 0.8,
            reflection_fuzz: 0.1,
        },
        speed: Vector3d { x: 0.0, y: 0.0, z: 0.0 },
        mass: radius_planet.powf(3.0),
        extra_brightness: 0.0,
        center_old: center,
    };

    let number_of_spheres = 80;
    World {
        spheres: (0..number_of_spheres).map(move |_| random_sphere(rng))
            .chain(std::iter::once(planet))
            .collect()
    }
}

fn cam(width: usize, height: usize, t_world: f64) -> Camera {
    let t_cam = t_world.mul(5.0).sub(2.3).tanh().add(1.0).div(2.0);
    let position = Vector3d {
        x: 15.0 * (7.1 * t_cam).sin(),
        y: 0.1 + 8.1 * t_cam.mul(-1.0).add(1.0),
        z: 15.0 * (5.9 * (t_cam - 0.05)).cos(),
    };
    let looks_at = Vector3d {
        x: 0.3 * (7.1 * t_cam).sin(),
        y: position.y.sqrt().div(4.0),
        z: 0.3 * (8.1 * t_cam).cos(),
    };
    let v_rotation = t_world.mul(40.0).sub(20.0).tanh().add(1.0).mul(PI);
    let up_direction = Vector3d { x: 0.0, y: v_rotation.cos(), z: v_rotation.sin() };
    let dist_to_looks_at = (position - &looks_at).length();
    let dist_to_focus = (dist_to_looks_at + 0.1 * (0.74 * t_cam).sin()).max(3.5);
    let max_aperture = 0.17;
    let aperture = max_aperture - t_world.powf(5.0) * max_aperture;
    let aspect_ratio = width as f64 / height as f64;
    let vertical_field_of_view = 80.0;

    Camera::new(&position, &looks_at, &up_direction, vertical_field_of_view,
                aspect_ratio, aperture, dist_to_focus)
}


fn create_video(dir_path_str: &str) {
    let video_path = format!("{}.mp4", dir_path_str);
    println!("Saving {}", video_path);
    Command::new("ffmpeg")
        .arg("-i")
        .arg(format!("{}/%08d.png", dir_path_str))
        .arg("-c:v")
        .arg("libx264")
        .arg("-preset")
        .arg("veryslow")
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
}

#[derive(Debug, Deserialize)]
struct Config {
    resolution_x: usize,
    resolution_y: usize,
    samples_per_pixel: usize,
    max_depth: usize,
    display_scale_factor: usize,
    export: bool,
}

fn init() -> Config {
    let config_path = "raytracer.toml";
    let mut config_file_content = String::new();
    File::open(config_path).and_then(|mut f| {
        f.read_to_string(&mut config_file_content)
    }).expect(&format!("Unable to read config file: {}", config_path));
    toml::from_str(&config_file_content)
        .expect(&format!("Unable to parse config file: {}", config_path))
}

fn main() {
    let config = init();

    let mut rng: StdRng = SeedableRng::seed_from_u64(42);
    let mut world = make_world(&mut rng);

    let window_width = config.resolution_x * config.display_scale_factor;
    let window_height = config.resolution_y * config.display_scale_factor;
    let canvas = Canvas::new(window_width, window_height).title("bouncing-spheres");

    let datetime: DateTime<Utc> = SystemTime::now().into();
    let dir_path_str = format!("./output/{}", datetime.format("%Y-%m-%d_%H-%M-%S"));
    if config.export {
        fs::create_dir_all(Path::new(&dir_path_str))
            .expect(&format!("Can not create output directory: {}", dir_path_str));
    }

    let num_frames = 960;
    let mut frame_num = 0;
    let mut t_world = 0.0;
    let mut t_world_old = 0.0;
    canvas.render(move |_, image| {
        let t_real = frame_num as f64 / num_frames as f64;
        let time_old = (frame_num - 1) as f64 / num_frames as f64;
        t_world_old = t_world;
        t_world = t_real - (50.0 * (t_real - 0.417)).tanh() / 50.0 - 0.02;
        world = world.advance(t_world - t_world_old);
        let sky_factor = t_real;
        let cam_new = cam(image.width(), image.height(), t_real);
        let cam_old = cam(image.width(), image.height(), time_old);
        let pixels = raytracer::render::render(
            image.width() / config.display_scale_factor,
            image.height() / config.display_scale_factor,
            config.samples_per_pixel, config.max_depth, &world,
            &cam_new, &cam_old,
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
        if config.export {
            pixels.save_png(&Path::new(&dir_path_str)
                .join(format!("{:08}.png", frame_num)));
        } else {
            println!("Frame {} of {}", frame_num, num_frames)
        }
        if frame_num >= num_frames {
            create_video(&dir_path_str);
            std::process::exit(0);
        }
        frame_num += 1;
    });
}
