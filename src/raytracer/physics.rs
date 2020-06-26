use std::cell::RefCell;
use std::ops::Deref;

use itertools::Itertools;

use crate::raytracer::sphere::Sphere;
use crate::raytracer::vector3d::unit_vector;

pub struct PhysicsSettings {
    pub gravity_constant: f64,
    pub bounciness: f64,

    // Avoid infinite bouncing
    pub bounce_round_to_zero_threshold: f64,

    pub flash_strength: f64,
    pub dim_factor: f64,
    pub dim_constant: f64,
    pub friction: f64,
}

pub fn gravitate(spheres: &Vec<Sphere>, _: f64, _: f64) -> Vec<Sphere> {
    spheres
        .iter()
        .map(|sphere| {
            let acceleration = sphere.center * -0.6;
            Sphere {
                speed: sphere.speed + &acceleration,
                ..*sphere
            }
        }).collect()
}

pub fn bounce(spheres: &Vec<Sphere>, _: f64, _: f64,
              _: f64) -> Vec<Sphere> {
    spheres.to_vec()
}

pub fn solve_non_overlapping_constraint(spheres: &Vec<Sphere>) -> Vec<Sphere> {
    let mut change = true;
    let mut spheres_copy = spheres.to_vec();
    let new_spheres = spheres_copy.iter_mut()
        .map(|s| RefCell::new(s))
        .collect::<Vec<RefCell<&mut Sphere>>>();
    while change {
        change = false;
        new_spheres.iter().combinations(2).for_each(|pair| {
            if let [a, b] = pair.as_slice() {
                let mut a = a.borrow_mut();
                let mut b = b.borrow_mut();
                let diff = b.center - &a.center;
                let dist = diff.length();
                let min_dist = a.radius + b.radius;
                if dist < min_dist {
                    let move_fraction_a = b.mass / (b.mass + a.mass);
                    let move_fraction_b = 1.0 - move_fraction_a;
                    let move_dist = min_dist - dist;
                    let dir_b_to_a = unit_vector(&(a.center - &b.center));
                    let dir_a_to_b = unit_vector(&(b.center - &a.center));
                    a.center = a.center + &(dir_b_to_a * move_dist * move_fraction_a) + &(dir_b_to_a * 0.00000001);
                    b.center = b.center + &(dir_a_to_b * move_dist * move_fraction_b) + &(dir_a_to_b * 0.00000001);
                    change = true;
                }
            }
        })
    }
    new_spheres.iter().map(|s| {
        s.borrow().deref().deref().clone()
    }).collect()
}

pub fn move_positions(spheres: &Vec<Sphere>, delta_t: f64) -> Vec<Sphere> {
    spheres.iter().map(|sphere| {
        Sphere {
            center_old: sphere.center,
            center: sphere.center + &(sphere.speed * delta_t),
            ..*sphere
        }
    }).collect()
}

pub fn dim(spheres: &Vec<Sphere>, delta_t: f64,
           dim_factor: f64, dim_constant: f64) -> Vec<Sphere> {
    spheres.iter().map(|sphere| {
        Sphere {
            extra_brightness: (sphere.extra_brightness -
                (sphere.extra_brightness * dim_factor * delta_t +
                    dim_constant * delta_t)).max(0.0),
            ..*sphere
        }
    }).collect()
}

pub fn friction(spheres: &Vec<Sphere>, _: f64, _: f64) -> Vec<Sphere> {
    spheres.to_vec()
}
