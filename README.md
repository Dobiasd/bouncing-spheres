# bouncing-spheres

A very simplistic raytracer - implemented in Rust - based on [a nice tutorial by Peter Shirley](https://raytracing.github.io/books/RayTracingInOneWeekend.html).

https://youtu.be/NPh1-T89xjE

![example](images/example.jpg)

Features:
- Spheres as the only type of object
- Lambertian surfaces
- Reflections
- Depth of field
- Motion blur
- Multi-core rendering
- Rudimentary physics (gravity and elastic collisions)
- Some hardcoded initial conditions and camera movements
- Display and export as images and video

Things it does not have:
- Refraction
- Fog (and other volumes)
- Textures
- Optimizations like bounding volume hierarchies
- Any interesting objects
- Support for some scene-definition language
- All the countless other cool things good and actually useful raytracers can do

Basically, all it can do is waste a few CPU hours to produce an output video like the following ([YouTube link](https://youtu.be/NPh1-T89xjE)):

[![bouncing spheres - a very simplistic raytracer - implemented in Rust](http://img.youtube.com/vi/NPh1-T89xjE/0.jpg)](http://www.youtube.com/watch?v=NPh1-T89xjE "bouncing spheres - a very simplistic raytracer - implemented in Rust")

# Compile and run

```bash
RUSTFLAGS="-C target-cpu=native" cargo run --release --package bouncing-spheres --bin main
```

When done, you'll find an `output` directory with a bunch of images (`.png`) files and (if `ffmpeg` is available) a video (`.mp4`) in it.

# Render quality

The settings can be changed in `raytracer.toml`.
