# Raytracer

A very simple raytracer based on [a nice tutorial by Peter Shirley](https://raytracing.github.io/books/RayTracingInOneWeekend.html), implemented in Rust.

# Compile and run

```bash
RUSTFLAGS="-C target-cpu=native" cargo run --release --package raytracer --bin main
```

You'll find an `image` directory with a bunch of `.png` files in it.

# Generate video from images

```bash
ffmpeg -i ./images/[DIR_NAME]/%08d.png -c:v libx264 -preset slow -profile:v high -crf 18 -coder 1 -pix_fmt yuv420p -movflags +faststart -g 60 -bf 2 ./images/video.mp4
```

# Render quality

You can change the settings in  `raytracer.toml`.
