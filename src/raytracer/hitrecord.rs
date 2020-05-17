use crate::raytracer::material::Material;
use crate::raytracer::ray::Ray;
use crate::raytracer::vector3d::{dot, Vector3d};

pub struct HitRecord {
    pub p: Vector3d,
    pub t: f64,
    pub normal: Vector3d,
    pub front_face: bool,
    pub material: Material,
}

pub fn face_normal(r: &Ray, outward_normal: &Vector3d) -> (bool, Vector3d) {
    let front_face = dot(&r.direction, outward_normal) < 0.0;
    let normal = if front_face {
        *outward_normal
    } else {
        Vector3d { x: 0.0, y: 0.0, z: 0.0 } - &outward_normal
    };
    return (front_face, normal);
}
