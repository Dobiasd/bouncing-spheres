use rand::prelude::ThreadRng;
use rand::Rng;

use crate::raytracer::floatcolor::FloatColor;
use crate::raytracer::hitrecord::HitRecord;
use crate::raytracer::ray::Ray;
use crate::raytracer::vector3d::{dot, random_in_unit_sphere, random_unit_vector, reflect, unit_vector};

#[derive(Copy, Clone)]
pub struct Material {
    pub albedo: FloatColor,
    pub reflectiveness: f64,
    pub fuzz: f64,
}

impl Material {
    pub fn scatter(&self, mut rng: ThreadRng, ray: &Ray, rec: &HitRecord) -> Option<(Ray, FloatColor)> {
        return if rng.gen::<f64>() > self.reflectiveness {
            let scatter_direction = rec.normal + &random_unit_vector(rng);
            let scattered = Ray { origin: rec.p, direction: scatter_direction };
            Some((scattered, self.albedo))
        } else {
            let reflected = reflect(&unit_vector(&ray.direction), &rec.normal);

            let scattered = Ray {
                origin: rec.p,
                direction: reflected + &(random_in_unit_sphere(rng) * self.fuzz),
            };
            if dot(&scattered.direction, &rec.normal) > 0.0 {
                Some((scattered, self.albedo))
            } else {
                None
            }
        };
    }
}