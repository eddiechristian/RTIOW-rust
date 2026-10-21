use std::sync::Arc;

use crate::aabb::AABB;
use crate::hittable::{BBox, HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::rtweekend::{Point3, Vec3};

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Arc<dyn Material>,
    bbox: AABB,
}

impl Sphere {
    pub fn new(center: &Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        let radius = radius.max(0.0);
        let rvec = Vec3::new(radius, radius, radius);
        let bbox = AABB::from_points(center - rvec, center + rvec);
        Sphere {
            center: *center,
            radius,
            mat,
            bbox,
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
            mat: Arc::clone(&self.mat),
            front_face: true,
            // Real sphere UVs come in a later section; solid textures only use p.
            u: 0.0,
            v: 0.0,
        };
        hit_rec.set_face_normal(ray, &normal);

        Some(hit_rec)
    }
}

impl BBox for Sphere {
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
