use crate::raytracer::hitrecord::{face_normal, HitRecord};
use crate::raytracer::material::Material;
use crate::raytracer::ray::Ray;
use crate::raytracer::vector3d::{dot, Vector3d};

pub struct Sphere {
    pub center: Vector3d,
    pub radius: f64,
    pub material: Material
}

pub struct HittableSpheres {
    pub spheres: Vec<Sphere>
}

impl HittableSpheres {
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut rec: Option<HitRecord> = None;
        for sphere in &self.spheres {
            match hit_sphere(&sphere.center, sphere.radius, r, &sphere.material, t_min,closest_so_far) {
                Some(temp_rec) => {
                    closest_so_far = temp_rec.t;
                    rec = Some(temp_rec);
                }
                None => {}
            }
        }
        return rec;
    }
}

// todo: make method
fn hit_sphere(center: &Vector3d, radius: f64, r: &Ray, mat: &Material, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let oc = r.origin - center;
    let a = r.direction.length_squared();
    let half_b = dot(&oc, &r.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant > 0.0 {
        let root = discriminant.sqrt();
        let temp = (-half_b - root) / a;
        // todo: remove duplication of these two blocks
        if temp < t_max && temp > t_min {
            let p = r.at(temp);
            let outward_normal = (p - center) / radius;
            let (front_face, normal) = face_normal(r, &outward_normal);
            return Some(HitRecord {
                p,
                t: temp,
                normal,
                front_face,
                material: *mat,
            });
        }
        let temp2 = (-half_b + root) / a;
        if temp2 < t_max && temp2 > t_min {
            let p = r.at(temp2);
            let outward_normal = (p - center) / radius;
            let (front_face, normal) = face_normal(r, &outward_normal);
            return Some(HitRecord {
                p,
                t: temp2,
                normal,
                front_face,
                material: *mat,
            });
        }
    }
    return None;
}
