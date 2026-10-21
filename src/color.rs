use crate::interval::Interval;
use crate::rtweekend::Vec3;
use std::fs::File;
use std::io::{BufWriter, Write};

pub type Color = Vec3;

// Convert a linear color component to gamma 2 space.
pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    0.0
}

pub fn write_color(writer: &mut BufWriter<File>, pixel_color: Color) {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    // Replace NaN components with zero, so a bad sample can't poison the output.
    if r.is_nan() {
        r = 0.0;
    }
    if g.is_nan() {
        g = 0.0;
    }
    if b.is_nan() {
        b = 0.0;
    }

    // Apply a linear to gamma transform for gamma 2
    let r = linear_to_gamma(r);
    let g = linear_to_gamma(g);
    let b = linear_to_gamma(b);

    // Translate the [0,1] component values to the byte range [0,255].
    let intensity = Interval::new(0.000, 0.999);
    let ir = (256.0 * intensity.clamp(r)) as i32;
    let ig = (256.0 * intensity.clamp(g)) as i32;
    let ib = (256.0 * intensity.clamp(b)) as i32;
    match writeln!(writer, "{} {} {}", ir, ig, ib) {
        Ok(_) => {} // Do nothing on success
        Err(e) => {
            eprintln!("\nFailed to write to ppm file: {}", e);
        }
    }
}
