use crate::aabb::AABB;
use crate::hittable::{BBox, HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::rtweekend::{Point3, Vec3, random_int};

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
    pub bbox: AABB,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: AABB::default(),
        }
    }

    #[allow(dead_code)] // part of the book's hittable_list API; not called yet
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.bbox = AABB::from_two_bboxes(&self.bbox, &object.bounding_box());
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

    // Equal mixture of every member's density: pick a member at random, then sample it.
    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        if self.objects.is_empty() {
            return 0.0;
        }
        let weight = 1.0 / self.objects.len() as f64;
        self.objects
            .iter()
            .map(|object| weight * object.pdf_value(origin, direction))
            .sum()
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        if self.objects.is_empty() {
            return Vec3::new(1.0, 0.0, 0.0);
        }
        let index = random_int(0, self.objects.len() as i32 - 1) as usize;
        self.objects[index].random(origin)
    }
}

impl BBox for HittableList {
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
