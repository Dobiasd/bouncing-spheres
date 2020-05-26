use crate::animation::animation::world_time_from_real_time;
use crate::raytracer::hit::Hit;
use crate::raytracer::physics::{bounce, dim, friction, gravitate, move_positions, PhysicsSettings, solve_non_overlapping_constraint};
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

    pub fn advance(&self, t_real: f64, t_real_previous_frame: f64,
                   physics: &PhysicsSettings) -> World {
        let delta_t = world_time_from_real_time(t_real) -
            world_time_from_real_time(t_real_previous_frame);
        World {
            spheres:
            dim(
                &friction(
                    &solve_non_overlapping_constraint(
                        &bounce(
                            &gravitate(
                                &move_positions(
                                    &self.spheres, delta_t),
                                delta_t, physics.gravity_constant),
                            physics.bounciness,
                            physics.flash_strength,
                            physics.bounce_round_to_zero_threshold)
                    ),
                    delta_t, physics.friction),
                delta_t, physics.dim_factor, physics.dim_constant)
        }
    }
}
