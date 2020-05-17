pub mod camera;
pub mod vector3d;
pub mod ray;
use std::ops::{Add, Div, Mul, Sub};
use std::prelude::v1::Vec;
use std::time::SystemTime;

use pixel_canvas::{Canvas, Color, input::MouseState};
use rand::prelude::*;


fn unit_vector(v: &vector3d::Vector3d) -> vector3d::Vector3d {
    return *v / v.length();
}


#[derive(Copy, Clone)]
pub struct FloatColor {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

#[derive(Copy, Clone)]
struct HitRecord {
    p: vector3d::Vector3d,
    t: f64,
    normal: vector3d::Vector3d,
    front_face: bool,
}

impl Mul<f64> for FloatColor {
    type Output = FloatColor;
    fn mul(self, f: f64) -> FloatColor {
        return FloatColor { r: self.r * f, g: self.g * f, b: self.b * f };
    }
}

impl Div<f64> for FloatColor {
    type Output = FloatColor;
    fn div(self, f: f64) -> FloatColor {
        return FloatColor { r: self.r / f, g: self.g / f, b: self.b / f };
    }
}

impl Add<&FloatColor> for FloatColor {
    type Output = FloatColor;
    fn add(self, other: &FloatColor) -> FloatColor {
        return FloatColor { r: self.r + other.r, g: self.g + other.g, b: self.b + other.b };
    }
}

struct Sphere {
    center: vector3d::Vector3d,
    radius: f64,
}

struct HittableSpheres {
    spheres: Vec<Sphere>
}

impl HittableSpheres {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        let mut rec: Option<HitRecord> = None;
        for sphere in &self.spheres {
            match hit_sphere(&sphere.center, sphere.radius, r, t_min, closest_so_far)
            {
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

pub struct FloatImage {
    data: Vec<FloatColor>,
    width: usize,
    height: usize,
}

impl FloatImage {
    pub fn new(width: usize, height: usize) -> FloatImage {
        FloatImage {
            data: vec![FloatColor { r: 0.0, g: 0.0, b: 0.0 }; width * height],
            width,
            height,
        }
    }
    pub fn get(&self, x: usize, y: usize) -> &FloatColor {
        return &self.data[y * self.width + x];
    }
    pub fn set(&mut self, x: usize, y: usize, c: FloatColor) {
        self.data[y * self.width + x] = c;
    }
}







fn ray_color(r: &ray::Ray, spheres: &HittableSpheres) -> FloatColor {
    let t_min = 0.001;
    let t_max = 99999999.9;
    match spheres.hit(r, t_min, t_max) {
        Some(rec) => {
            return FloatColor { r: rec.normal.x + 1.0, g: rec.normal.y + 1.0, b: rec.normal.z + 1.0 } * 0.5;
        }
        None => {}
    }
    let unit_direction = unit_vector(&r.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    let col1 = FloatColor { r: 1.0, g: 1.0, b: 1.0 };
    let col2 = FloatColor { r: 0.5, g: 0.7, b: 1.0 };
    return col1 * (1.0 - t) + &(col2 * t);
}

fn face_normal(r: &ray::Ray, outward_normal: &vector3d::Vector3d) -> (bool, vector3d::Vector3d) {
    let front_face = vector3d::dot(&r.direction, outward_normal) < 0.0;
    let normal = if front_face {
        *outward_normal
    } else {
        (vector3d::Vector3d { x: 0.0, y: 0.0, z: 0.0 } - &outward_normal)
    };
    return (front_face, normal);
}

fn hit_sphere(center: &vector3d::Vector3d, radius: f64, r: &ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let oc = r.origin - center;
    let a = r.direction.length_squared();
    let half_b = vector3d::dot(&oc, &r.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant > 0.0 {
        let root = discriminant.sqrt();
        let temp = (-half_b - root) / a;
        if temp < t_max && temp > t_min {
            let p = r.at(temp);
            let outward_normal = (p - center) / radius;
            let (front_face, normal) = face_normal(r, &outward_normal);
            return Some(HitRecord {
                p,
                t: temp,
                normal,
                front_face,
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
            });
        }
    }
    return None;
}

pub fn render(mut rng: ThreadRng, width: usize, height: usize) -> FloatImage {
    let mut image = FloatImage::new(width, height);

    let aspect_ratio = width as f64 / height as f64;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = vector3d::Vector3d { x: 0.0, y: 0.0, z: 0.0 };
    let horizontal = vector3d::Vector3d { x: viewport_width, y: 0.0, z: 0.0 };
    let vertical = vector3d::Vector3d { x: 0.0, y: viewport_height, z: 0.0 };
    let lower_left_corner = origin - &(horizontal / 2.0) - &(vertical / 2.0) - &vector3d::Vector3d { x: 0.0, y: 0.0, z: focal_length };

    let world = HittableSpheres {
        spheres: vec![
            Sphere { center: vector3d::Vector3d { x: 0.0, y: 0.0, z: -1.0 }, radius: 0.5 },
            Sphere { center: vector3d::Vector3d { x: 0.0, y: -100.5, z: -1.0 }, radius: 100.0 }]
    };

    let cam = camera::Camera::new(16.0 / 9.0, 2.0, 1.0);

    let samples_per_pixel = 100;

    for y in 0..height {
        for x in 0..width {
            let mut pixel_color = FloatColor { r: 0.0, g: 0.0, b: 0.0 };
            for s in 0..samples_per_pixel {
                let u = (x as f64 + rng.gen::<f64>()) / (width as f64 - 1.0);
                let v = (y as f64 + rng.gen::<f64>()) / (height as f64 - 1.0);
                let r = cam.get_ray(u, v);
                pixel_color = pixel_color + &ray_color(&r, &world);
            }
            image.set(x, y, pixel_color / samples_per_pixel as f64);
        }
    }
    return image;
}

