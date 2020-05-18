use crate::raytracer::sphere::Sphere;
use crate::raytracer::ray::Ray;
use crate::raytracer::hit::Hit;

pub struct World {
    pub spheres: Vec<Sphere>
}

impl World {
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut closest_so_far = t_max;
        let mut rec: Option<Hit> = None;
        for sphere in &self.spheres {
            match sphere.hit(r, t_min, closest_so_far) {
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
