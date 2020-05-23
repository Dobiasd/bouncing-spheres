use crate::raytracer::material::Material;
use crate::raytracer::ray::Ray;
use crate::raytracer::vector3d::{dot, null_vector, Vector3d};

pub struct Hit {
    pub position: Vector3d,
    pub t: f64,
    pub normal: Vector3d,
    pub front_face: bool,
    pub material: Material,
}

#[inline(always)]
pub fn face_normal(r: &Ray, outward_normal: &Vector3d) -> (bool, Vector3d) {
    let front_face = dot(&r.direction, outward_normal) < 0.0;
    let normal = if front_face {
        *outward_normal
    } else {
        null_vector() - &outward_normal
    };
    (front_face, normal)
}
