use std::sync::Arc;

use crate::color::Color;
use crate::hittable::HitRecord;
use crate::pdf::{CosinePdf, Pdf};
use crate::ray::Ray;
use crate::rtweekend::{random_double, random_unit_vector, reflect, refract};
use crate::texture::{SolidColorTexture, Texture};

// How a material scatters an incoming ray (the book's scatter_record).
pub struct ScatterRecord {
    pub attenuation: Color,
    pub scatter: Scatter,
}

pub enum Scatter {
    // Diffuse-like: the camera picks the direction, mixing this PDF with
    // light sampling, and weights the result by scattering_pdf / pdf.
    Pdf(Box<dyn Pdf>),
    // Specular (metal, glass): one exact direction, nothing to importance-sample
    // (the book's skip_pdf + skip_pdf_ray).
    SkipPdf(Ray),
}

// Send + Sync so materials can be shared across rayon render threads.
pub trait Material: Send + Sync {
    // Returns how the ray scatters, or None if it is absorbed.
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    // Light given off at the hit point; only lights override this.
    fn emitted(&self, _r_in: &Ray, _rec: &HitRecord) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    // The material's actual density of scattering into `scattered`.
    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        0.0
    }
}

pub struct Lambertian {
    tex: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self::from_texture(Arc::new(SolidColorTexture::new(albedo)))
    }

    pub fn from_texture(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord {
            attenuation: self.tex.value(rec.u, rec.v, &rec.p),
            scatter: Scatter::Pdf(Box::new(CosinePdf::new(&rec.normal))),
        })
    }

    // Ideal diffuse reflection: density cos(theta) / pi, zero below the surface.
    fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cos_theta = rec.normal.dot(&scattered.dir.normalize());
        (cos_theta / std::f64::consts::PI).max(0.0)
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = reflect(&r_in.dir, &rec.normal);
        let reflected = reflected.normalize() + (self.fuzz * random_unit_vector());
        let scattered = Ray::new(&rec.p, &reflected);

        // Fuzz can push the ray below the surface; absorb it if so.
        if scattered.dir.dot(&rec.normal) > 0.0 {
            Some(ScatterRecord {
                attenuation: self.albedo,
                scatter: Scatter::SkipPdf(scattered),
            })
        } else {
            None
        }
    }
}

pub struct Dielectric {
    // Refractive index in vacuum or air, or the ratio of the material's refractive index over
    // the refractive index of the enclosing media
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    // Use Schlick's approximation for reflectance.
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = r_in.dir.normalize();
        let cos_theta = (-unit_direction).dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        // Total internal reflection when Snell's law has no solution.
        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract || Self::reflectance(cos_theta, ri) > random_double() {
            reflect(&unit_direction, &rec.normal)
        } else {
            refract(&unit_direction, &rec.normal, ri)
        };

        Some(ScatterRecord {
            attenuation,
            scatter: Scatter::SkipPdf(Ray::new(&rec.p, &direction)),
        })
    }
}

pub struct DiffuseLight {
    tex: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(emit: Color) -> Self {
        Self::from_texture(Arc::new(SolidColorTexture::new(emit)))
    }

    pub fn from_texture(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }
}

// A light only emits; it never scatters (the default scatter returns None).
impl Material for DiffuseLight {
    // One-sided: only the front face glows, so an add_quad light shines along u × v.
    fn emitted(&self, _r_in: &Ray, rec: &HitRecord) -> Color {
        if !rec.front_face {
            return Color::new(0.0, 0.0, 0.0);
        }
        self.tex.value(rec.u, rec.v, &rec.p)
    }
}
