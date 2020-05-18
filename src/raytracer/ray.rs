use crate::raytracer::vector3d::Vector3d;

pub struct Ray {
    pub origin: Vector3d,
    pub direction: Vector3d,
}

impl Ray {
    #[inline(always)]
    pub fn at(&self, t: f64) -> Vector3d {
        return self.origin + &(self.direction * t);
    }
}
