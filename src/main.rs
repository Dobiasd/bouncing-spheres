extern crate chrono;
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate simple_logger;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use log::info;
use pixel_canvas::{Canvas, Image as CanvasImage};
use structopt::StructOpt;

use crate::animation::animation::{camera_range, make_world, num_frames, physics_settings, sky};
use crate::export::export::Exporter;
use crate::export::stopwatch::{measure, Stopwatch};
use crate::raytracer::image::Image;

mod raytracer;
mod animation;
mod export;

#[derive(StructOpt)]
struct Cli {
    #[structopt(default_value = "raytracer.toml", long)]
    config_path: String,
    #[structopt(short, long)]
    profile: String,
}

#[derive(Debug, Deserialize, Clone, Copy)]
struct Profile {
    resolution_x: usize,
    resolution_y: usize,
    samples_per_pixel: usize,
    max_depth: usize,
    display_scale_factor: usize,
    export: bool,
}

#[derive(Debug, Deserialize)]
struct Config {
    profiles: HashMap<String, Profile>,
}

fn load_config() -> Profile {
    let args = Cli::from_args();
    let mut config_file_content = String::new();
    File::open(&args.config_path).and_then(|mut f| {
        f.read_to_string(&mut config_file_content)
    }).expect(&format!("Unable to read config file: {}", &args.config_path));
    *toml::from_str::<Config>(&config_file_content)
        .expect(&format!("Unable to parse config file: {}", &args.config_path))
        .profiles.get(&args.profile)
        .expect(&format!("Unknown profile: {}", &args.profile))
}

fn render(profile: Profile) {
    let mut world = make_world();

    let canvas = Canvas::new(
        profile.resolution_x * profile.display_scale_factor,
        profile.resolution_y * profile.display_scale_factor)
        .title("bouncing-spheres");

    let exporter = Exporter::new(profile.export);
    let mut frame_num = 0;
    let mut frame_stopwatch = Stopwatch::new();

    canvas.render(move |_, image| {
        let t_real = frame_num as f64 / num_frames() as f64;
        let t_real_previous_frame = ((frame_num as f64 - 1.0) / num_frames() as f64).max(0.0);

        let (world_advanced, physics_duration) = measure(
            || world.advance(t_real, t_real_previous_frame, &physics_settings()));
        world = world_advanced;

        let cams = camera_range(t_real, t_real_previous_frame,
                                image.width() as f64 / image.height() as f64);

        let (pixels, render_duration) = measure(|| raytracer::render::render(
            profile.resolution_x, profile.resolution_y,
            profile.samples_per_pixel, profile.max_depth, &world,
            &cams, &sky(t_real)));

        plot_pixels(image, &pixels, profile.display_scale_factor);
        exporter.process_frame(&pixels, frame_num);
        info!("Time spent to render the current frame ({}/{}): {} ms (\
        {} ms physics + {} ms rendering + display)",
              frame_num + 1, num_frames(),
              frame_stopwatch.check_and_reset().as_millis(),
              physics_duration.as_millis(),
              render_duration.as_millis());

        frame_num += 1;
        if frame_num >= num_frames() {
            exporter.combine_frames_to_video();
            std::process::exit(0);
        }
    });
}

fn plot_pixels(image: &mut CanvasImage, pixels: &Image, scale_factor: usize) {
    let width = image.width();
    for (y, row) in image.chunks_mut(width).enumerate() {
        for (x, pixel) in row.iter_mut().enumerate() {
            *pixel = pixels.get(
                x / scale_factor,
                y / scale_factor,
            ).to_canvas_color()
        }
    }
}

fn main()
{
    simple_logger::init().unwrap();
    render(load_config());
}
