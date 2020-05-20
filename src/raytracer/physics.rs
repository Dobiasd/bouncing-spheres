use std::cell::RefCell;
use std::ops::Deref;

use itertools::Itertools;

use crate::raytracer::sphere::Sphere;
use crate::raytracer::vector3d::{dot, unit_vector, Vector3d, zero_in};

pub fn gravitate(spheres: &Vec<Sphere>, delta_t: f64) -> Vec<Sphere> {
    let gravity_constant = 0.73;
    spheres
        .iter()
        .map(|sphere| {
            let acceleration = spheres
                .iter()
                .map(|other| {
                    if other.id == sphere.id {
                        Vector3d { x: 0.0, y: 0.0, z: 0.0 }
                    } else {
                        let diff = other.center - &sphere.center;
                        let dist = diff.length();
                        unit_vector(&diff) * delta_t * gravity_constant * other.mass / dist.powf(2.0)
                    }
                }).fold(Vector3d { x: 0.0, y: 0.0, z: 0.0 },
                        |a: Vector3d, b: Vector3d| a + &b);
            Sphere {
                speed: sphere.speed + &acceleration,
                ..*sphere
            }
        }).collect()
}

pub fn bounce(spheres: &Vec<Sphere>) -> Vec<Sphere> {
    let bounciness = 0.46;
    let flash_strength = 0.007;
    let round_to_zero_threshold = 10.0; // Avoid infinite bouncing.
    let mut spheres_copy = spheres.to_vec();
    let new_spheres = spheres_copy.iter_mut()
        .map(|s| RefCell::new(s))
        .collect::<Vec<RefCell<&mut Sphere>>>();
    new_spheres.iter().combinations(2).for_each(|pair| {
        if let [a, b] = pair.as_slice() {
            let mut a = a.borrow_mut();
            let mut b = b.borrow_mut();
            let diff = b.center - &a.center;
            let dist = diff.length();
            let min_dist = a.radius + b.radius;
            if dist < min_dist {
                let dir_a_to_b = unit_vector(&(b.center - &a.center));
                let v_a_c_length = dot(&a.speed, &dir_a_to_b);
                let v_b_c_length = dot(&b.speed, &dir_a_to_b);
                let v_a_c = dir_a_to_b * v_a_c_length;
                let v_b_c = dir_a_to_b * v_b_c_length;
                let v_a_c_prime_length = (a.mass * v_a_c_length + b.mass * v_b_c_length - b.mass * (v_a_c_length - v_b_c_length) * bounciness) / (a.mass + b.mass);
                let v_b_c_prime_length = (b.mass * v_b_c_length + a.mass * v_a_c_length - a.mass * (v_b_c_length - v_a_c_length) * bounciness) / (a.mass + b.mass);
                let v_a_c_prime = dir_a_to_b * zero_in(round_to_zero_threshold, v_a_c_prime_length);
                let v_b_c_prime = dir_a_to_b * zero_in(round_to_zero_threshold, v_b_c_prime_length);
                let new_speed_a = a.speed - &v_a_c + &v_a_c_prime;
                let new_speed_b = b.speed - &v_b_c + &v_b_c_prime;
                a.extra_brightness = ((a.speed - &new_speed_a).length() * flash_strength).max(a.extra_brightness);
                b.extra_brightness = ((b.speed - &new_speed_b).length() * flash_strength).max(b.extra_brightness);
                a.speed = new_speed_a;
                b.speed = new_speed_b;
            }
        }
    });
    new_spheres.iter().map(|s| {
        s.borrow().deref().deref().clone()
    }).collect()
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

pub fn dim(spheres: &Vec<Sphere>, delta_t: f64) -> Vec<Sphere> {
    let dim_factor = 5.0;
    spheres.iter().map(|sphere| {
        Sphere {
            extra_brightness: sphere.extra_brightness -
                sphere.extra_brightness * dim_factor * delta_t,
            ..*sphere
        }
    }).collect()
}

pub fn friction(spheres: &Vec<Sphere>, delta_t: f64) -> Vec<Sphere> {
    let friction = 12.1;
    let deceleration_factor = delta_t * friction;
    // Something like the following would be a more realistic representation of air resistance:
    // delta_t * friction * sphere.radius.powf(2.0) / sphere.mass.powf(3.0)
    // But the other version looks nicer in the animation.
    spheres.iter().map(|sphere| {
        Sphere {
            speed: sphere.speed - &(sphere.speed * deceleration_factor),
            ..*sphere
        }
    }).collect()
}
