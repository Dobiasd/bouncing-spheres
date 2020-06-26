use rand::prelude::StdRng;
use rand::Rng;
use rand::SeedableRng;

use crate::raytracer::camera::{Camera, CameraRange};
use crate::raytracer::color::{blend_colors, Color};
use crate::raytracer::material::Material;
use crate::raytracer::physics::PhysicsSettings;
use crate::raytracer::render::Sky;
use crate::raytracer::sphere::Sphere;
use crate::raytracer::vector3d::Vector3d;
use crate::raytracer::world::World;

fn random_sphere(rng: &mut StdRng) -> Sphere {
    let radius = rng.gen_range(0.2_f64, 2.0_f64);

    let min = -14.0_f64;
    let max = 14.0_f64;

    let center = Vector3d {
        x: rng.gen_range(min, max),
        y: rng.gen_range(min, max),
        z: rng.gen_range(min, max),
    };
    Sphere {
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
        speed: Vector3d::null(),
        mass: radius.powf(3.0),
        extra_brightness: 0.0,
        center_old: center,
    }
}

pub fn make_world() -> World {
    let mut rng: StdRng = SeedableRng::seed_from_u64(42);

    let number_of_spheres = 16;
    World {
        spheres: (0..number_of_spheres).map(move |_| random_sphere(&mut rng)).collect()
    }
}

pub fn camera_range(t_real: f64, t_real_previous_frame: f64, aspect_ratio: f64) -> CameraRange {
    CameraRange {
        cam_a: cam(t_real, aspect_ratio),
        cam_b: cam(t_real_previous_frame, aspect_ratio),
    }
}

pub fn cam(_: f64, aspect_ratio: f64) -> Camera {
    let position = Vector3d {
        x: 0.0,
        y: 0.0,
        z: 21.0,
    };
    let looks_at = Vector3d {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let up_direction = Vector3d { x: 0.0, y: 1.0, z: 0.0 };
    let dist_to_looks_at = (position - &looks_at).length();
    let dist_to_focus = dist_to_looks_at;
    let aperture = 0.0;
    let vertical_field_of_view = 80.0;

    Camera::new(&position, &looks_at, &up_direction, vertical_field_of_view,
                aspect_ratio, aperture, dist_to_focus)
}

pub fn num_frames() -> usize { 480 }

pub fn sky(t_real: f64) -> Sky {
    let sky_factor = t_real;
    let day1 = Color { r: 0.6, g: 0.6, b: 0.6 };
    let day2 = Color { r: 0.3, g: 0.4, b: 0.5 };
    let night1 = Color { r: 0.6, g: 0.5, b: 0.5 };
    let night2 = Color { r: 0.4, g: 0.1, b: 0.15 };
    Sky {
        col1: blend_colors(&night1, &day1, sky_factor),
        col2: blend_colors(&night2, &day2, sky_factor),
    }
}

pub fn physics_settings() -> PhysicsSettings {
    PhysicsSettings {
        gravity_constant: 0.0,
        bounciness: 0.0,
        bounce_round_to_zero_threshold: 0.0,
        flash_strength: 0.006,
        dim_factor: 5.0,
        dim_constant: 1.32,
        friction: 12.1,
    }
}

pub fn world_time_from_real_time(t_real: f64) -> f64 {
    t_real
}
