use std::cell::RefCell;

use itertools::Itertools;

use crate::raytracer::hit::Hit;
use crate::raytracer::ray::Ray;
use crate::raytracer::sphere::Sphere;
use crate::raytracer::vector3d::{unit_vector, Vector3d};

pub struct World {
    pub spheres: Vec<Sphere>
}

impl World {
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut closest_so_far = t_max;
        let mut rec: Option<Hit> = None;
        for sphere in &self.spheres {
            match sphere.hit(ray, t_min, closest_so_far) {
                Some(temp_rec) => {
                    closest_so_far = temp_rec.t;
                    rec = Some(temp_rec);
                }
                None => {}
            }
        }
        rec
    }

    pub fn advance(&self, delta_t: f64) -> World {
        World {
            spheres:
            move_positions(&friction(&bounce(&gravitate(
                &self.spheres, delta_t))), delta_t)
        }
    }
}

fn gravitate(spheres: &Vec<Sphere>, delta_t: f64) -> Vec<Sphere> {
    let gravity_constant = 0.00000023;
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
                        (diff) * delta_t * gravity_constant * other.mass / dist.powf(2.0)
                    }
                }).fold(Vector3d { x: 0.0, y: 0.0, z: 0.0 },
                        |a: Vector3d, b: Vector3d| a + &b);
            Sphere {
                id: sphere.id,
                center: sphere.center,
                radius: sphere.radius,
                material: sphere.material,
                speed: sphere.speed + &acceleration,
                mass: sphere.mass,
            }
        }).collect()
}

fn bounce(spheres: &Vec<Sphere>) -> Vec<Sphere> {
    let bounciness = 0.53;
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
                    let move_dir_a = unit_vector(&(a.center - &b.center));
                    let move_dir_b = unit_vector(&(b.center - &a.center));
                    a.center = a.center + &(move_dir_a * move_dist * move_fraction_a) + &(move_dir_a * 0.00000001);
                    b.center = b.center + &(move_dir_b * move_dist * move_fraction_b) + &(move_dir_b * 0.00000001);
                    // todo: use bounciness
                    change = true;
                }
            }
        })
    }
    new_spheres.iter().map(|s| {
        let sphere = s.borrow();
        Sphere {
            id: sphere.id,
            center: sphere.center,
            radius: sphere.radius,
            material: sphere.material,
            speed: sphere.speed,
            mass: sphere.mass,
        }
    }).collect()
}

fn move_positions(spheres: &Vec<Sphere>, delta_t: f64) -> Vec<Sphere> {
    spheres.iter().map(|sphere| {
        Sphere {
            id: sphere.id,
            center: sphere.center + &(sphere.speed * delta_t),
            radius: sphere.radius,
            material: sphere.material,
            speed: sphere.speed,
            mass: sphere.mass,
        }
    }).collect()
}

fn friction(spheres: &Vec<Sphere>) -> Vec<Sphere> {
    let friction = 0.98;
    spheres.iter().map(|sphere| {
        Sphere {
            id: sphere.id,
            center: sphere.center,
            radius: sphere.radius,
            material: sphere.material,
            speed: sphere.speed * friction,
            mass: sphere.mass,
        }
    }).collect()
}

// todo: Can we create a new instance without repeating all fields?
