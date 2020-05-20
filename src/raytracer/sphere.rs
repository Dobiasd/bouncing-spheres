use uuid::Uuid;

use crate::raytracer::hit::{face_normal, Hit};
use crate::raytracer::material::Material;
use crate::raytracer::ray::Ray;
use crate::raytracer::vector3d::{dot, is_in_interval, Vector3d};

#[derive(Copy, Clone)]
pub struct Sphere {
    pub id: Uuid,
    pub center: Vector3d,
    pub radius: f64,
    pub material: Material,
    pub speed: Vector3d,
    pub mass: f64,
    pub extra_brightness: f64,
}

impl Sphere {
    #[inline(always)]
    fn calculate_hit(&self, ray: &Ray, t: f64) -> Hit {
        let p = ray.at(t);
        let outward_normal = (p - &self.center) / self.radius;
        let (front_face, normal) = face_normal(ray, &outward_normal);
        Hit {
            position: p,
            t,
            normal,
            front_face,
            material: Material {
                albedo: self.material.albedo + &(self.material.albedo * self.extra_brightness),
                reflectiveness: self.material.reflectiveness - self.extra_brightness,
                reflection_fuzz: self.material.reflection_fuzz + self.extra_brightness,
            },
        }
    }

    #[inline(always)]
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let ray_origin_to_center = ray.origin - &self.center;
        let day_direction_squared_length = ray.direction.length_squared();
        let half_b = dot(&ray_origin_to_center, &ray.direction);
        let c = ray_origin_to_center.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - day_direction_squared_length * c;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let t_front = (-half_b - root) / day_direction_squared_length;
            if is_in_interval(t_front, t_min, t_max) {
                return Some(self.calculate_hit(ray, t_front));
            }
            let t_back = (-half_b + root) / day_direction_squared_length;
            if is_in_interval(t_back, t_min, t_max) {
                return Some(self.calculate_hit(ray, t_back));
            }
        }
        None
    }
}
