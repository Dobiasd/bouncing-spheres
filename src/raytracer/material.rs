use rand::prelude::StdRng;
use rand::Rng;

use crate::raytracer::color::Color;
use crate::raytracer::hit::Hit;
use crate::raytracer::ray::Ray;
use crate::raytracer::vector3d::{dot, random_unit_vector, reflect, unit_vector};

#[derive(Copy, Clone)]
pub struct Material {
    pub albedo: Color,
    pub reflectiveness: f64,
    pub reflection_fuzz: f64,
}

impl Material {
    #[inline(always)]
    pub fn scatter(&self, rng: &mut StdRng, ray: &Ray, rec: &Hit) -> Option<(Ray, Color)> {
        if rng.gen::<f64>() > self.reflectiveness {
            let scatter_direction = rec.normal + &random_unit_vector(rng);
            let scattered = Ray { origin: rec.position, direction: scatter_direction };
            Some((scattered, self.albedo))
        } else {
            let reflected = reflect(&unit_vector(&ray.direction), &rec.normal);
            let scattered = Ray {
                origin: rec.position,
                direction: reflected + &(random_unit_vector(rng) * self.reflection_fuzz),
            };
            if dot(&scattered.direction, &rec.normal) > 0.0 {
                Some((scattered, self.albedo))
            } else {
                None
            }
        }
    }
}