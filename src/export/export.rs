use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use log::info;

use crate::raytracer::image::Image;

pub struct Exporter {
    dir_path_str: Option<String>
}

impl Exporter {
    pub fn new(enabled: bool) -> Exporter {
        if enabled {
            let datetime: DateTime<Utc> = SystemTime::now().into();
            let dir_path_str = format!("./output/{}", datetime.format("%Y-%m-%d_%H-%M-%S"));
            fs::create_dir_all(Path::new(&dir_path_str))
                .expect(&format!("Can not create output directory: {}", dir_path_str));
            Exporter { dir_path_str: Some(dir_path_str) }
        } else {
            Exporter { dir_path_str: None }
        }
    }

    pub fn process_frame(&self, frame: &Image, frame_num: usize) {
        match &self.dir_path_str {
            Some(dir_path_str) => {
                let image_path = &Path::new(&dir_path_str)
                    .join(format!("{:08}.png", frame_num));
                info!("Saving {}", image_path.display());
                frame.save_png(image_path);
            }
            None => {}
        }
    }

    pub fn combine_frames_to_video(&self) {
        match &self.dir_path_str {
            Some(dir_path_str) => {
                let video_path = format!("{}.mp4", dir_path_str);
                info!("Saving {}", video_path);
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
            None => {}
        }
    }
}
