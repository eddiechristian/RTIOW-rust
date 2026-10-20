use std::cmp::Ordering;
use std::sync::Arc;

use crate::aabb::AABB;
use crate::hittable::{BBox, HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;

// Children are Arc (the book's shared_ptr) so a single-object node can point
// both left and right at the same object.
pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: AABB,
}

impl BVHNode {
    pub fn from_list(list: HittableList) -> Self {
        Self::new(list.objects.into_iter().map(Arc::from).collect())
    }

    pub fn new(mut objects: Vec<Arc<dyn Hittable>>) -> Self {
        // Build the bounding box of the span of source objects.
        let bbox = objects.iter().fold(AABB::EMPTY, |bbox, object| {
            AABB::from_two_bboxes(&bbox, &object.bounding_box())
        });

        let axis = bbox.longest_axis();

        let (left, right): (Arc<dyn Hittable>, Arc<dyn Hittable>) = match objects.len() {
            0 => panic!("BVHNode::new called with no objects"),
            1 => {
                let object = objects.pop().unwrap();
                (Arc::clone(&object), object)
            }
            2 => {
                let right = objects.pop().unwrap();
                let left = objects.pop().unwrap();
                (left, right)
            }
            n => {
                objects.sort_by(|a, b| Self::box_compare(a, b, axis));
                let right_objects = objects.split_off(n / 2);
                (
                    Arc::new(BVHNode::new(objects)),
                    Arc::new(BVHNode::new(right_objects)),
                )
            }
        };

        Self { left, right, bbox }
    }

    fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: usize) -> Ordering {
        let a_axis_interval = *a.bounding_box().axis_interval(axis);
        let b_axis_interval = *b.bounding_box().axis_interval(axis);
        a_axis_interval.min.total_cmp(&b_axis_interval.min)
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        if !self.bbox.hit(ray, ray_t) {
            return None;
        }

        let hit_left = self.left.hit(ray, ray_t);
        // Only accept a right-side hit if it is closer than the left one.
        let right_max = hit_left.as_ref().map_or(ray_t.max, |rec| rec.t);
        let hit_right = self.right.hit(ray, Interval::new(ray_t.min, right_max));

        hit_right.or(hit_left)
    }
}

impl BBox for BVHNode {
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
