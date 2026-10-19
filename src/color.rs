use crate::rtweekend::Vec3;
use std::fs::File;
use std::io::{BufWriter, Write};

pub type Color = Vec3;

pub fn write_color(writer: &mut BufWriter<File>, pixel_color: Color) {
    let r = pixel_color.x;
    let g = pixel_color.y;
    let b = pixel_color.z;

    // Scale to 0..255 range and cast to integer
    let ir = (255.999 * r) as i32;
    let ig = (255.999 * g) as i32;
    let ib = (255.999 * b) as i32;
    match writeln!(writer, "{} {} {}", ir, ig, ib) {
        Ok(_) => {} // Do nothing on success
        Err(e) => {
            eprintln!("\nFailed to write to ppm file: {}", e);
        }
    }
}
