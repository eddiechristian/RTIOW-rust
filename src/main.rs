mod color;
mod ray;
mod rtweekend;

use std::fs::File;
use std::io::{self};
use std::io::{BufWriter, Write}; // Required for flushing stdout

use crate::color::Color;
use crate::color::write_color;
use crate::ray::Ray;
use crate::rtweekend::{Point3, Vec3};

pub fn ray_color(r: &Ray) -> Color {
    if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Color::new(1.0, 0.0, 0.0);
    }

    // 1. Get the unit vector of the ray's direction (.normalize() in nalgebra)
    let unit_direction: Vec3 = r.dir.normalize();

    // 2. Map the y component from [-1.0, 1.0] to [0.0, 1.0]
    let a = 0.5 * (unit_direction.y + 1.0);

    // 3. Linear blend (lerp) between white and light blue
    let white = Color::new(1.0, 1.0, 1.0);
    let blue = Color::new(0.5, 0.7, 1.0);

    (1.0 - a) * white + a * blue
}

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> bool {
    let oc: Vec3 = center - ray.orig;
    let a: f64 = ray.dir.dot(&ray.dir);
    let b: f64 = -2.0 * ray.dir.dot(&oc);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant >= 0.0
}

fn main() -> std::io::Result<()> {
    //Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: i32 = 1280;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;

    //Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u: Vec3 = viewport_u / image_width as f64;
    let pixel_delta_v: Vec3 = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render
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
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r: Ray = Ray::new(&camera_center, &ray_direction);

            let pixel_color: Color = ray_color(&r);

            write_color(&mut writer, pixel_color);
        }
    }

    // Explicitly flush to save safely to disk
    writer.flush()?;
    println!("\nPPM image successfully saved to gradient.ppm!");
    Ok(())
}
