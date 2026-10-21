use std::sync::Arc;

use crate::aabb::AABB;
use crate::hittable::{BBox, HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::onb::Onb;
use crate::ray::Ray;
use crate::rtweekend::{Point3, Vec3, random_double};

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

    // Uniform over the cone of directions from origin that hit the sphere.
    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        if self
            .hit(
                &Ray::new(origin, direction),
                Interval::new(0.001, f64::INFINITY),
            )
            .is_none()
        {
            return 0.0;
        }

        let dist_squared = (self.center - origin).norm_squared();
        let cos_theta_max = (1.0 - self.radius * self.radius / dist_squared)
            .max(0.0)
            .sqrt();
        let solid_angle = 2.0 * std::f64::consts::PI * (1.0 - cos_theta_max);
        1.0 / solid_angle
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        let direction = self.center - origin;
        let uvw = Onb::new(&direction);
        uvw.transform(&random_to_sphere(self.radius, direction.norm_squared()))
    }
}

impl BBox for Sphere {
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

// Random direction around +z inside the cone that a sphere of this radius
// subtends from distance sqrt(distance_squared).
fn random_to_sphere(radius: f64, distance_squared: f64) -> Vec3 {
    let r1 = random_double();
    let r2 = random_double();
    // Clamp so a point on (or, from rounding, just inside) the sphere doesn't give NaN.
    let cos_theta_max = (1.0 - radius * radius / distance_squared).max(0.0).sqrt();
    let z = 1.0 + r2 * (cos_theta_max - 1.0);

    let phi = 2.0 * std::f64::consts::PI * r1;
    let sin_theta = (1.0 - z * z).sqrt();

    Vec3::new(phi.cos() * sin_theta, phi.sin() * sin_theta, z)
}
