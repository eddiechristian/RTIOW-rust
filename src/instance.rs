use std::sync::Arc;

use nalgebra::{Rotation3, Unit};

use crate::aabb::AABB;
use crate::hittable::{BBox, HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::rtweekend::{Point3, Vec3, degrees_to_radians};

// Instances move an object without changing its geometry: the ray is moved
// into the object's own space instead, and the hit is moved back out.

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3,
    bbox: AABB,
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vec3) -> Self {
        let b = object.bounding_box();
        let bbox = AABB::new(
            Interval::new(b.x.min + offset.x, b.x.max + offset.x),
            Interval::new(b.y.min + offset.y, b.y.max + offset.y),
            Interval::new(b.z.min + offset.z, b.z.max + offset.z),
        );
        Self {
            object,
            offset,
            bbox,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        // Move the ray backwards by the offset, hit, then move the hit point forwards.
        let offset_r = Ray::new(&(ray.orig - self.offset), &ray.dir);
        let mut rec = self.object.hit(&offset_r, ray_t)?;
        rec.p += self.offset;
        Some(rec)
    }
}

impl BBox for Translate {
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

// Rotation about any axis through the origin. The book only has rotate_y.
pub struct Rotate {
    object: Arc<dyn Hittable>,
    rotation: Rotation3<f64>,
    bbox: AABB,
}

impl Rotate {
    pub fn new(object: Arc<dyn Hittable>, axis: Vec3, angle_degrees: f64) -> Self {
        let rotation = Rotation3::from_axis_angle(
            &Unit::new_normalize(axis),
            degrees_to_radians(angle_degrees),
        );

        // The rotated box is the box around the 8 rotated corners of the original.
        let b = object.bounding_box();
        let mut min = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
        for x in [b.x.min, b.x.max] {
            for y in [b.y.min, b.y.max] {
                for z in [b.z.min, b.z.max] {
                    let corner = rotation * Point3::new(x, y, z);
                    min = min.inf(&corner);
                    max = max.sup(&corner);
                }
            }
        }

        Self {
            object,
            rotation,
            bbox: AABB::from_points(min, max),
        }
    }
}

impl Hittable for Rotate {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        // Rotate the ray into object space with the inverse rotation.
        let inverse = self.rotation.inverse();
        let rotated_r = Ray::new(&(inverse * ray.orig), &(inverse * ray.dir));

        let mut rec = self.object.hit(&rotated_r, ray_t)?;

        // Rotate the hit back to world space. Rotations keep lengths and angles,
        // so the normal stays a unit vector and front_face stays correct.
        rec.p = self.rotation * rec.p;
        rec.normal = self.rotation * rec.normal;
        Some(rec)
    }
}

impl BBox for Rotate {
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
