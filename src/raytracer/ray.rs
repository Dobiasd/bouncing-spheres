use crate::raytracer::vector3d::Vector3d;

pub struct Ray {
    pub origin: Vector3d,
    pub direction: Vector3d,
    pub frame_time: f64,
}

impl Ray {
    #[inline(always)]
    pub fn at(&self, t: f64) -> Vector3d {
        self.origin + &(self.direction * t)
    }
}
