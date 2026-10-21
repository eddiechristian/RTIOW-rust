use crate::rtweekend::Vec3;

// Orthonormal basis: three perpendicular unit vectors, with w along a given
// direction. Lets us generate directions around +z and then point them along w.
pub struct Onb {
    axis: [Vec3; 3],
}

impl Onb {
    pub fn new(n: &Vec3) -> Self {
        let w = n.normalize();
        // Any vector not parallel to w works as a helper for the cross products.
        let a = if w.x.abs() > 0.9 {
            Vec3::y()
        } else {
            Vec3::x()
        };
        let v = w.cross(&a).normalize();
        let u = w.cross(&v);
        Self { axis: [u, v, w] }
    }

    pub fn w(&self) -> &Vec3 {
        &self.axis[2]
    }

    // Map a vector given in this basis's coordinates to world coordinates.
    pub fn transform(&self, v: &Vec3) -> Vec3 {
        v.x * self.axis[0] + v.y * self.axis[1] + v.z * self.axis[2]
    }
}
