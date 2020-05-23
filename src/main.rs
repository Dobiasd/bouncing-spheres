extern crate chrono;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::io::Read;
use std::time::SystemTime;

use pixel_canvas::Canvas;

use crate::animation::animation::{cam, make_world};
use crate::export::export::Exporter;

mod raytracer;
mod animation;
mod export;

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
    let mut world = make_world();

    let canvas = Canvas::new(
        config.resolution_x * config.display_scale_factor,
        config.resolution_y * config.display_scale_factor)
        .title("bouncing-spheres");

    let exporter = Exporter::new(config.export);

    let num_frames = 960;
    let mut frame_num = 0;
    let mut last_frame_start_time = SystemTime::now();
    canvas.render(move |_, image| {
        let t_real = frame_num as f64 / num_frames as f64;
        let t_real_previous_frame = ((frame_num as f64 - 1.0) / num_frames as f64).max(0.0);
        world = world.advance(t_real, t_real_previous_frame);
        let sky_factor = t_real;
        let cam_new = cam(image.width(), image.height(), t_real);
        let cam_old = cam(image.width(), image.height(), t_real_previous_frame);
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

        exporter.process_frame(&pixels, frame_num, num_frames);

        let frame_done_time = SystemTime::now();
        let elapsed = frame_done_time.duration_since(last_frame_start_time)
            .expect("Time is broken.");
        last_frame_start_time = frame_done_time;
        println!("Duration to render the frame: {} ms", elapsed.as_millis());

        if frame_num >= num_frames {
            exporter.combine_frames_to_video();
            std::process::exit(0);
        }
        frame_num += 1;
    });
}


fn main() {
    render(read_config());
}
