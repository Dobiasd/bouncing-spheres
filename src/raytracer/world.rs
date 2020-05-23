use crate::raytracer::hit::Hit;
use crate::raytracer::physics::{bounce, dim, friction, gravitate, move_positions, solve_non_overlapping_constraint};
use crate::raytracer::ray::Ray;
use crate::raytracer::sphere::Sphere;

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

    pub fn advance(&self, t_real: f64, t_real_previous_frame: f64) -> World {
        let delta_t = world_time_from_real_time(t_real) -
            world_time_from_real_time(t_real_previous_frame);
        println!("delta_t {}", delta_t);
        World {
            spheres:
            dim(
                &friction(
                    &solve_non_overlapping_constraint(
                        &bounce(
                            &gravitate(
                                &move_positions(
                                    &self.spheres, delta_t),
                                delta_t)
                        )
                    ),
                    delta_t),
                delta_t)
        }
    }
}

fn world_time_from_real_time(t_real: f64) -> f64 {
    t_real - (50.0 * (t_real - 0.417)).tanh() / 50.0 - 0.02
}
