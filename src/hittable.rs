use crate::interval::Interval;
use crate::ray::Ray;
use crate::rtweekend::{Point3, Vec3};

pub struct HitRecord {
    #[allow(dead_code)] // TODO: remove once something reads p
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.dir.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
}
