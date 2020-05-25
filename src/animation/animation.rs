use std::f64::consts::PI;
use std::ops::{Add, Div, Mul, Sub};

use rand::prelude::StdRng;
use rand::Rng;
use rand::SeedableRng;
use uuid::Uuid;

use crate::raytracer::camera::{Camera, CameraRange};
use crate::raytracer::color::{blend_colors, Color};
use crate::raytracer::material::Material;
use crate::raytracer::physics::PhysicsSettings;
use crate::raytracer::render::Sky;
use crate::raytracer::sphere::Sphere;
use crate::raytracer::vector3d::Vector3d;
use crate::raytracer::world::World;

fn random_sphere(rng: &mut StdRng) -> Sphere {
    let min = -5.0;
    let max = 5.0;
    let max_start_y = 123.0;
    let radius = 0.4 + 1.7 *
        rng.gen_range(-10.0_f64, 0.9_f64).tanh().add(1.0).div(2.0);
    let center = Vector3d {
        x: rng.gen_range(min, max),
        y: rng.gen_range(radius, max_start_y),
        z: rng.gen_range(min, max),
    };
    Sphere {
        id: Uuid::new_v4(),
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
    let radius_planet = 6371.0;
    let center = Vector3d { x: 0.0, y: -radius_planet, z: 0.0 };
    let planet = Sphere {
        id: Uuid::new_v4(),
        center,
        radius: radius_planet,
        material: Material {
            albedo: Color { r: 0.5, g: 0.5, b: 0.5 },
            reflectiveness: 0.75,
            reflection_fuzz: 0.08,
        },
        speed: Vector3d::null(),
        mass: radius_planet.powf(3.0),
        extra_brightness: 0.0,
        center_old: center,
    };

    let number_of_spheres = 80;
    World {
        spheres: (0..number_of_spheres).map(move |_| random_sphere(&mut rng))
            .chain(std::iter::once(planet))
            .collect()
    }
}

pub fn camera_range(t_real: f64, t_real_previous_frame: f64, aspect_ratio: f64) -> CameraRange {
    CameraRange {
        cam_a: cam(t_real, aspect_ratio),
        cam_b: cam(t_real_previous_frame, aspect_ratio),
    }
}

pub fn cam(t_real: f64, aspect_ratio: f64) -> Camera {
    let t_cam = t_real.mul(5.0).sub(2.3).tanh().add(1.0).div(2.0);
    let position = Vector3d {
        x: 15.0 * (7.1 * t_cam).sin(),
        y: 0.1 + 8.1 * t_cam.mul(-1.0).add(1.0),
        z: 15.0 * (5.9 * (t_cam - 0.05)).cos(),
    };
    let looks_at = Vector3d {
        x: 0.3 * (7.1 * t_cam).sin(),
        y: position.y.sqrt().div(4.0),
        z: 0.3 * (8.1 * t_cam).cos(),
    };
    let v_rotation = t_real.mul(40.0).sub(20.0).tanh().add(1.0).mul(PI);
    let up_direction = Vector3d { x: 0.0, y: v_rotation.cos(), z: v_rotation.sin() };
    let dist_to_looks_at = (position - &looks_at).length();
    let dist_to_focus = (dist_to_looks_at + 0.1 * (0.74 * t_cam).sin()).max(3.5);
    let max_aperture = 0.17;
    let aperture = max_aperture - t_real.powf(5.0) * max_aperture;
    let vertical_field_of_view = 80.0;

    Camera::new(&position, &looks_at, &up_direction, vertical_field_of_view,
                aspect_ratio, aperture, dist_to_focus)
}

pub fn num_frames() -> usize {
    960
}

pub fn sky(t_real: f64) -> Sky {
    let sky_factor = t_real;
    let day1 = Color { r: 1.0, g: 1.0, b: 1.0 };
    let day2 = Color { r: 0.5, g: 0.7, b: 1.0 };
    let night1 = Color { r: 0.8, g: 0.6, b: 0.7 };
    let night2 = Color { r: 0.4, g: 0.1, b: 0.15 };
    Sky {
        col1: blend_colors(&night1, &day1, sky_factor),
        col2: blend_colors(&night2, &day2, sky_factor),
    }
}

pub fn physics_settings() -> PhysicsSettings {
    PhysicsSettings {
        gravity_constant: 0.73,
        bounciness: 0.46,
        bounce_round_to_zero_threshold: 10.0,
        flash_strength: 0.006,
        dim_factor: 5.0,
        dim_constant: 1.32,
        friction: 12.1,
    }
}