# Raytracer

A very simplistic raytracer implemented in Rust - based on [a nice tutorial by Peter Shirley](https://raytracing.github.io/books/RayTracingInOneWeekend.html), .
It can only do spheres, reflection and depth of field, but no refraction, fog, textures, etc.
Also there are no optimizations like bounding volume hierarchy, etc., just brute-force tracing all the rays. 

![example](example.png)

# Compile and run

```bash
RUSTFLAGS="-C target-cpu=native" cargo run --release --package raytracer --bin main
```

When done, you'll find an `output` directory with a bunch of image (`.png`) files and (if `ffmpeg` is available) a video (`.mp4`) in it.

# Render quality

The settings can be changed in `raytracer.toml`.

# todo

- Add bouncing to physics
- Light sources
- Motion blur
- Let spheres light up (and get non-reflective) when bouncing according to magnitude of change of speed
- Speed up camera during the animation, sigmoid(t)
- Slow down physics during the animation
