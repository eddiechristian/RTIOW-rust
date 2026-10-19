use crate::rtweekend::{Point3, Vec3};

pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(orig: &Point3, dir: &Vec3) -> Self {
        Self {
            orig: *orig,
            dir: *dir,
        }
    }

    // Returns the point along the ray at parameter t: P(t) = A + t*B
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}
