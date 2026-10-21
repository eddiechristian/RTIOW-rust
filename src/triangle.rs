use std::sync::Arc;

use crate::aabb::AABB;
use crate::hittable::{BBox, HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::rtweekend::{Point3, Vec3};

// Replaces the book's quad: every flat shape is built from triangles.
pub struct Triangle {
    a: Point3,
    e1: Vec3, // b - a
    e2: Vec3, // c - a
    normal: Vec3,
    mat: Arc<dyn Material>,
    bbox: AABB,
}

impl Triangle {
    // The front face is the side from which a, b, c appear counter-clockwise.
    pub fn new(a: Point3, b: Point3, c: Point3, mat: Arc<dyn Material>) -> Self {
        let e1 = b - a;
        let e2 = c - a;
        let bbox = AABB::from_two_bboxes(&AABB::from_points(a, b), &AABB::from_points(a, c));
        Self {
            a,
            e1,
            e2,
            normal: e1.cross(&e2).normalize(),
            mat,
            bbox,
        }
    }
}

impl Hittable for Triangle {
    // Möller–Trumbore: solve a + beta*e1 + gamma*e2 = orig + t*dir directly.
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let pvec = ray.dir.cross(&self.e2);
        let det = self.e1.dot(&pvec);

        // Ray parallel to the triangle's plane.
        if det.abs() < 1e-8 {
            return None;
        }
        let inv_det = 1.0 / det;

        let tvec = ray.orig - self.a;
        let beta = tvec.dot(&pvec) * inv_det;
        if !(0.0..=1.0).contains(&beta) {
            return None;
        }

        let qvec = tvec.cross(&self.e1);
        let gamma = ray.dir.dot(&qvec) * inv_det;
        if gamma < 0.0 || beta + gamma > 1.0 {
            return None;
        }

        let t = self.e2.dot(&qvec) * inv_det;
        if !ray_t.contains(t) {
            return None;
        }

        let mut rec = HitRecord {
            t,
            p: ray.at(t),
            normal: self.normal,
            mat: Arc::clone(&self.mat),
            front_face: true,
            // Barycentric coordinates double as texture coordinates.
            u: beta,
            v: gamma,
        };
        rec.set_face_normal(ray, &self.normal);
        Some(rec)
    }
}

impl BBox for Triangle {
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

// Adds the parallelogram with corner q and edges u, v as two triangles.
// The front face is the side u × v points to, as with the book's quad.
pub fn add_quad(list: &mut HittableList, q: Point3, u: Vec3, v: Vec3, mat: Arc<dyn Material>) {
    list.add(Box::new(Triangle::new(q, q + u, q + v, Arc::clone(&mat))));
    list.add(Box::new(Triangle::new(q + u + v, q + v, q + u, mat)));
}

// The box with opposite corners a and b, as 12 triangles (6 quads).
pub fn make_box(a: Point3, b: Point3, mat: Arc<dyn Material>) -> HittableList {
    let mut sides = HittableList::new();

    let min = Point3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
    let max = Point3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

    let dx = Vec3::new(max.x - min.x, 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y - min.y, 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z - min.z);

    // Each face's edges are ordered so its normal points out of the box.
    let p = |x, y, z| Point3::new(x, y, z);
    add_quad(&mut sides, p(min.x, min.y, max.z), dx, dy, Arc::clone(&mat)); // front
    add_quad(
        &mut sides,
        p(max.x, min.y, max.z),
        -dz,
        dy,
        Arc::clone(&mat),
    ); // right
    add_quad(
        &mut sides,
        p(max.x, min.y, min.z),
        -dx,
        dy,
        Arc::clone(&mat),
    ); // back
    add_quad(&mut sides, p(min.x, min.y, min.z), dz, dy, Arc::clone(&mat)); // left
    add_quad(
        &mut sides,
        p(min.x, max.y, max.z),
        dx,
        -dz,
        Arc::clone(&mat),
    ); // top
    add_quad(&mut sides, p(min.x, min.y, min.z), dx, dz, mat); // bottom

    sides
}

// Square pyramid: base of side `base` centered on `base_center` in the xz
// plane, apex `height` above it. 6 triangles (2 base, 4 sides), normals outward.
pub fn make_pyramid(
    base_center: Point3,
    base: f64,
    height: f64,
    mat: Arc<dyn Material>,
) -> HittableList {
    let mut faces = HittableList::new();

    let a = base / 2.0;
    let corner = |dx, dz| base_center + Vec3::new(dx, 0.0, dz);
    let (b0, b1, b2, b3) = (corner(-a, -a), corner(a, -a), corner(a, a), corner(-a, a));
    let apex = base_center + Vec3::new(0.0, height, 0.0);

    // Base faces down.
    faces.add(Box::new(Triangle::new(b0, b1, b2, Arc::clone(&mat))));
    faces.add(Box::new(Triangle::new(b0, b2, b3, Arc::clone(&mat))));

    // Sides: base edge counter-clockwise as seen from outside, then the apex.
    faces.add(Box::new(Triangle::new(b3, b2, apex, Arc::clone(&mat)))); // +z
    faces.add(Box::new(Triangle::new(b2, b1, apex, Arc::clone(&mat)))); // +x
    faces.add(Box::new(Triangle::new(b1, b0, apex, Arc::clone(&mat)))); // -z
    faces.add(Box::new(Triangle::new(b0, b3, apex, mat))); // -x

    faces
}

// Triangular prism centered on `center`: an isosceles triangle `width` wide
// and `height` tall in the xy plane, extruded `depth` along z.
// 8 triangles (2 end caps, 3 rectangular sides), normals outward.
pub fn make_prism(
    center: Point3,
    width: f64,
    height: f64,
    depth: f64,
    mat: Arc<dyn Material>,
) -> HittableList {
    let mut faces = HittableList::new();

    let (w, h, d) = (width / 2.0, height / 2.0, depth / 2.0);
    let p = |x, y, z| center + Vec3::new(x, y, z);
    let extrude = Vec3::new(0.0, 0.0, 2.0 * d);

    // End caps: front faces +z, back faces -z.
    faces.add(Box::new(Triangle::new(
        p(-w, -h, d),
        p(w, -h, d),
        p(0.0, h, d),
        Arc::clone(&mat),
    )));
    faces.add(Box::new(Triangle::new(
        p(-w, -h, -d),
        p(0.0, h, -d),
        p(w, -h, -d),
        Arc::clone(&mat),
    )));

    // Sides, each a quad from the back cap's edge extruded forward.
    add_quad(
        &mut faces,
        p(-w, -h, -d),
        Vec3::new(2.0 * w, 0.0, 0.0),
        extrude,
        Arc::clone(&mat),
    ); // bottom
    add_quad(
        &mut faces,
        p(w, -h, -d),
        Vec3::new(-w, 2.0 * h, 0.0),
        extrude,
        Arc::clone(&mat),
    ); // right slope
    add_quad(
        &mut faces,
        p(0.0, h, -d),
        Vec3::new(-w, -2.0 * h, 0.0),
        extrude,
        mat,
    ); // left slope

    faces
}
