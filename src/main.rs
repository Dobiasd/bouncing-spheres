extern crate chrono;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::time::SystemTime;

use chrono::DateTime;
use chrono::offset::Utc;
use pixel_canvas::Canvas;
use rand::prelude::StdRng;
use rand::SeedableRng;

use crate::animation::animation::{cam, make_world};
use crate::video::video::create_video;

mod raytracer;
mod animation;
mod video;

#[derive(Debug, Deserialize)]
struct Config {
    resolution_x: usize,
    resolution_y: usize,
    samples_per_pixel: usize,
    max_depth: usize,
    display_scale_factor: usize,
    export: bool,
}

fn read_config() -> Config {
    let config_path = "raytracer.toml";
    let mut config_file_content = String::new();
    File::open(config_path).and_then(|mut f| {
        f.read_to_string(&mut config_file_content)
    }).expect(&format!("Unable to read config file: {}", config_path));
    toml::from_str(&config_file_content)
        .expect(&format!("Unable to parse config file: {}", config_path))
}

fn render(config: Config) {
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
    let mut last_frame_start_time = SystemTime::now();
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

        let frame_done_time = SystemTime::now();
        let elapsed = frame_done_time.duration_since(last_frame_start_time)
            .expect("Time is broken.");
        last_frame_start_time = frame_done_time;
        println!("Duration to render the frame: {} ms", elapsed.as_millis());

        if frame_num >= num_frames {
            create_video(&dir_path_str);
            std::process::exit(0);
        }
        frame_num += 1;
    });
}


fn main() {
    render(read_config());
}
