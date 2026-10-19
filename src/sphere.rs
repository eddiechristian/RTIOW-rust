use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::rtweekend::{Point3, Vec3};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: &Point3, radius: f64) -> Self {
        Sphere {
            center: *center,
            radius,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc: Vec3 = self.center - ray.orig;
        let a: f64 = ray.dir.dot(&ray.dir);
        let h = ray.dir.dot(&oc);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }
        let t = root;
        let p = ray.at(root);
        let normal = (p - self.center) / self.radius;

        let mut hit_rec = HitRecord {
            t,
            p,
            normal,
            front_face: true,
        };
        hit_rec.set_face_normal(ray, &normal);

        Some(hit_rec)
    }
}
