use std::process::Command;

pub fn create_video(dir_path_str: &str) {
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
