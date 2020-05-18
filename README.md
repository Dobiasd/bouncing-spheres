# Raytracer

A very simplistic raytracer implemented in Rust - based on [a nice tutorial by Peter Shirley](https://raytracing.github.io/books/RayTracingInOneWeekend.html), .
It can only do spheres and reflection, but no refraction, lights, etc.

# Compile and run

```bash
RUSTFLAGS="-C target-cpu=native" cargo run --release --package raytracer --bin main
```

When done, you'll find an `output` directory with a bunch of image (`.png`) files and (if `ffmpeg` is available) a video (`.mp4`) in it.

# Render quality

The settings can be changed in `raytracer.toml`.
