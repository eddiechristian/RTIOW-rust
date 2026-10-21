use crate::interval::Interval;
use crate::ray::Ray;
use crate::rtweekend::Point3;

#[allow(clippy::upper_case_acronyms)] // keep the book's aabb name
#[derive(Clone, Copy, Debug)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

// UNIVERSE is part of the book's aabb API; not called yet.
#[allow(dead_code)]
impl AABB {
    pub const EMPTY: AABB = AABB {
        x: Interval::EMPTY,
        y: Interval::EMPTY,
        z: Interval::EMPTY,
    };
    pub const UNIVERSE: AABB = AABB {
        x: Interval::UNIVERSE,
        y: Interval::UNIVERSE,
        z: Interval::UNIVERSE,
    };

    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_two_bboxes(box0: &AABB, box1: &AABB) -> Self {
        let x = Interval::from_two_intervals(&box0.x, &box1.x);
        let y = Interval::from_two_intervals(&box0.y, &box1.y);
        let z = Interval::from_two_intervals(&box0.z, &box1.z);
        AABB { x, y, z }
    }

    pub fn from_points(a: Point3, b: Point3) -> Self {
        let mut bbox = Self {
            x: if a[0] <= b[0] {
                Interval::new(a[0], b[0])
            } else {
                Interval::new(b[0], a[0])
            },
            y: if a[1] <= b[1] {
                Interval::new(a[1], b[1])
            } else {
                Interval::new(b[1], a[1])
            },
            z: if a[2] <= b[2] {
                Interval::new(a[2], b[2])
            } else {
                Interval::new(b[2], a[2])
            },
        };
        bbox.pad_to_minimums();
        bbox
    }

    // Widen any side thinner than delta, so flat shapes (an axis-aligned
    // triangle has zero thickness) never get a degenerate box.
    fn pad_to_minimums(&mut self) {
        let delta = 0.0001;
        if self.x.size() < delta {
            self.x = self.x.expand(delta);
        }
        if self.y.size() < delta {
            self.y = self.y.expand(delta);
        }
        if self.z.size() < delta {
            self.z = self.z.expand(delta);
        }
    }

    pub fn axis_interval(&self, n: usize) -> &Interval {
        match n {
            1 => &self.y,
            2 => &self.z,
            _ => &self.x, // 0 or any fallback defaults to x
        }
    }

    // Returns the index of the longest axis of the bounding box.
    pub fn longest_axis(&self) -> usize {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() { 0 } else { 2 }
        } else if self.y.size() > self.z.size() {
            1
        } else {
            2
        }
    }

    pub fn hit(&self, r: &Ray, mut ray_t: Interval) -> bool {
        let ray_orig = r.orig;
        let ray_dir = r.dir;

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir[axis];

            let t0 = (ax.min - ray_orig[axis]) * adinv;
            let t1 = (ax.max - ray_orig[axis]) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                }
                if t1 < ray_t.max {
                    ray_t.max = t1;
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                }
                if t0 < ray_t.max {
                    ray_t.max = t0;
                }
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }
}

impl Default for AABB {
    fn default() -> Self {
        AABB::EMPTY
    }
}
