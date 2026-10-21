use std::f64::consts::PI;

use crate::hittable::Hittable;
use crate::onb::Onb;
use crate::rtweekend::{Point3, Vec3, random_cosine_direction, random_double, random_unit_vector};

// A probability density over directions: `generate` picks a random direction
// and `value` reports how likely that direction was to be picked.
pub trait Pdf {
    fn value(&self, direction: &Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

// Uniform over all directions.
#[allow(dead_code)] // the book uses it for the isotropic (volume) material, not implemented yet
pub struct SpherePdf;

impl Pdf for SpherePdf {
    fn value(&self, _direction: &Vec3) -> f64 {
        1.0 / (4.0 * PI)
    }

    fn generate(&self) -> Vec3 {
        random_unit_vector()
    }
}

// Proportional to cos(theta) around a surface normal: the ideal diffuse distribution.
pub struct CosinePdf {
    uvw: Onb,
}

impl CosinePdf {
    pub fn new(w: &Vec3) -> Self {
        Self { uvw: Onb::new(w) }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        let cosine_theta = direction.normalize().dot(self.uvw.w());
        (cosine_theta / PI).max(0.0)
    }

    fn generate(&self) -> Vec3 {
        self.uvw.transform(&random_cosine_direction())
    }
}

// Directions from `origin` toward an object (usually the lights).
pub struct HittablePdf<'a> {
    objects: &'a dyn Hittable,
    origin: Point3,
}

impl<'a> HittablePdf<'a> {
    pub fn new(objects: &'a dyn Hittable, origin: Point3) -> Self {
        Self { objects, origin }
    }
}

impl Pdf for HittablePdf<'_> {
    fn value(&self, direction: &Vec3) -> f64 {
        self.objects.pdf_value(&self.origin, direction)
    }

    fn generate(&self) -> Vec3 {
        self.objects.random(&self.origin)
    }
}

// 50/50 blend of two PDFs: sample one or the other, average their densities.
pub struct MixturePdf<'a> {
    p: [&'a dyn Pdf; 2],
}

impl<'a> MixturePdf<'a> {
    pub fn new(p0: &'a dyn Pdf, p1: &'a dyn Pdf) -> Self {
        Self { p: [p0, p1] }
    }
}

impl Pdf for MixturePdf<'_> {
    fn value(&self, direction: &Vec3) -> f64 {
        0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
    }

    fn generate(&self) -> Vec3 {
        if random_double() < 0.5 {
            self.p[0].generate()
        } else {
            self.p[1].generate()
        }
    }
}
