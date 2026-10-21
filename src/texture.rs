use std::sync::Arc;

use crate::{color::Color, perlin::Perlin, rtweekend::Point3};

pub struct SolidColorTexture {
    albedo: Color,
}

pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

// Send + Sync so textures can be shared across rayon render threads.
pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

impl SolidColorTexture {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Texture for SolidColorTexture {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.albedo
    }
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        CheckerTexture {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub fn from_colors(scale: f64, c1: Color, c2: Color) -> Self {
        Self::new(
            scale,
            Arc::new(SolidColorTexture::new(c1)),
            Arc::new(SolidColorTexture::new(c2)),
        )
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        // Which cell of the 3D grid p falls in, one integer per axis.
        let x = (self.inv_scale * p.x).floor() as i32;
        let y = (self.inv_scale * p.y).floor() as i32;
        let z = (self.inv_scale * p.z).floor() as i32;

        // Neighboring cells alternate, so the parity of the sum picks the texture.
        if (x + y + z) % 2 == 0 {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

// Which look a NoiseTexture renders; the book switches between these by editing value().
pub enum NoiseStyle {
    // Smooth Perlin noise (book Image 13).
    Smooth,
    // Turbulence used directly: camouflage netting (book Image 14).
    Turbulence,
    // Stripes along z whose phase is shifted by turbulence (book Image 15).
    Marble,
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
    style: NoiseStyle,
}

impl NoiseTexture {
    pub fn new(scale: f64, style: NoiseStyle) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
            style,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        let scaled = Point3::from(self.scale * p.coords);
        let gray = match self.style {
            // Noise is in [-1,1]; map it to [0,1] so colors stay non-negative.
            NoiseStyle::Smooth => 0.5 * (1.0 + self.noise.noise(&scaled)),
            NoiseStyle::Turbulence => self.noise.turb(&scaled, 7),
            // Scale sets the stripe frequency; turbulence makes the stripes wavy.
            NoiseStyle::Marble => {
                0.5 * (1.0 + (self.scale * p.z + 10.0 * self.noise.turb(p, 7)).sin())
            }
        };
        Color::new(1.0, 1.0, 1.0) * gray
    }
}
