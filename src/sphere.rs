use crate::hittable::{HitRecord, Hittable};
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
            radius: radius,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
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
        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return None;
            }
        }
        let t = root;
        let p = ray.at(root);
        let normal = (p - self.center) / self.radius;

        let mut hit_rec = HitRecord {
            t: t,
            p: p,
            normal: normal,
            front_face: true,
        };
        hit_rec.set_face_normal(&ray, &normal);

        Some(hit_rec)
    }
}
