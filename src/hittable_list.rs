use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    #[allow(dead_code)] // part of the book's hittable_list API; not called yet
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.max;
        let mut hit_anything = None;

        // Shrink the search range after each hit so only the nearest one survives.
        for object in &self.objects {
            if let Some(rec) = object.hit(ray, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = rec.t;
                hit_anything = Some(rec);
            }
        }

        hit_anything
    }
}
