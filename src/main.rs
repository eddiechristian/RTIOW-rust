mod color;
mod rtweekend;

use std::fs::File;
use std::io::{self};
use std::io::{BufWriter, Write}; // Required for flushing stdout
use std::thread::sleep;
use std::time::Duration;

use crate::color::Color;
use crate::color::write_color;

fn main() -> std::io::Result<()> {
    // 1. Image dimensions
    let image_width = 256;
    let image_height = 256;

    // 2. Open file and wrap in a buffer (Rust equivalent to std::cout redirection)
    let file = File::create("gradient.ppm")?;
    let mut writer: BufWriter<File> = BufWriter::new(file);

    // 3. Render Header
    writeln!(writer, "P3")?;
    writeln!(writer, "{} {}", image_width, image_height)?;
    writeln!(writer, "255")?;

    // 4. Render Pixels
    for j in 0..image_height {
        let rem = image_height - j;
        print!("\rScanlines remaining: {}   ", rem);
        io::stdout().flush().unwrap();
        for i in 0..image_width {
            // Map loop indices to 0.0 -> 1.0 bounds
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.0;

            let pixel_color = color::Color::new(r, g, b);

            write_color(&mut writer, pixel_color);
        }
    }

    // Explicitly flush to save safely to disk
    writer.flush()?;
    println!("\nPPM image successfully saved to gradient.ppm!");
    Ok(())
}
