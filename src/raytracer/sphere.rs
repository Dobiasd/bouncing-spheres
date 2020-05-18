use crate::raytracer::hit::{face_normal, Hit};
use crate::raytracer::material::Material;
use crate::raytracer::ray::Ray;
use crate::raytracer::vector3d::{dot, Vector3d};

pub struct Sphere {
    pub center: Vector3d,
    pub radius: f64,
    pub material: Material,
}

fn is_in_interval(x: f64, min: f64, max: f64) -> bool {
    x > min && x < max
}

impl Sphere {
    #[inline(always)]
    fn calculate_hit(&self, ray: &Ray, t: f64) -> Hit {
        let p = ray.at(t);
        let outward_normal = (p - &self.center) / self.radius;
        let (front_face, normal) = face_normal(ray, &outward_normal);
        return Hit {
            position: p,
            t,
            normal,
            front_face,
            material: &self.material,
        };
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
        return None;
    }
}
