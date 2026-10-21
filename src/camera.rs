use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::sync::atomic::{AtomicI32, Ordering};

use rayon::prelude::*;

use crate::color::{Color, write_color};
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::rtweekend::{Point3, Vec3, degrees_to_radians, random_double, random_in_unit_disk};

pub struct Camera {
    pub aspect_ratio: f64,         // Ratio of image width over height
    pub image_width: i32,          // Rendered image width in pixel count
    pub samples_per_pixel: i32,    // Count of random samples for each pixel
    pub max_depth: i32,            // Maximum number of ray bounces into scene
    pub background: Option<Color>, // Scene background color; None draws the sky gradient

    pub vfov: f64,        // Vertical view angle (field of view)
    pub lookfrom: Point3, // Point camera is looking from
    pub lookat: Point3,   // Point camera is looking at
    pub vup: Vec3,        // Camera-relative "up" direction

    pub defocus_angle: f64, // Variation angle of rays through each pixel
    pub focus_dist: f64,    // Distance from camera lookfrom point to plane of perfect focus

    image_height: i32,        // Rendered image height
    pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples
    center: Point3,           // Camera center
    pixel00_loc: Point3,      // Location of pixel 0, 0
    pixel_delta_u: Vec3,      // Offset to pixel to the right
    pixel_delta_v: Vec3,      // Offset to pixel below
    u: Vec3,                  // Camera frame basis vectors
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3, // Defocus disk horizontal radius
    defocus_disk_v: Vec3, // Defocus disk vertical radius
}

impl Camera {
    pub fn new() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            background: None,
            vfov: 90.0,
            lookfrom: Point3::new(0.0, 0.0, 0.0),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            image_height: 0,
            pixel_samples_scale: 0.0,
            center: Point3::origin(),
            pixel00_loc: Point3::origin(),
            pixel_delta_u: Vec3::zeros(),
            pixel_delta_v: Vec3::zeros(),
            u: Vec3::zeros(),
            v: Vec3::zeros(),
            w: Vec3::zeros(),
            defocus_disk_u: Vec3::zeros(),
            defocus_disk_v: Vec3::zeros(),
        }
    }

    pub fn render(&mut self, world: &dyn Hittable) -> io::Result<()> {
        self.initialize();

        let file = File::create("gradient.ppm")?;
        let mut writer: BufWriter<File> = BufWriter::new(file);

        // Header
        writeln!(writer, "P3")?;
        writeln!(writer, "{} {}", self.image_width, self.image_height)?;
        writeln!(writer, "255")?;

        // Pixels: render scanlines in parallel (rayon), then write them out in order.
        let remaining = AtomicI32::new(self.image_height);
        let rows: Vec<Vec<Color>> = (0..self.image_height)
            .into_par_iter()
            .map(|j| {
                let row = (0..self.image_width)
                    .map(|i| {
                        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                        for _ in 0..self.samples_per_pixel {
                            let r = self.get_ray(i, j);
                            pixel_color += self.ray_color(&r, self.max_depth, world);
                        }
                        self.pixel_samples_scale * pixel_color
                    })
                    .collect();

                let rem = remaining.fetch_sub(1, Ordering::Relaxed) - 1;
                print!("\rScanlines remaining: {}   ", rem);
                io::stdout().flush().unwrap();
                row
            })
            .collect();

        for pixel_color in rows.into_iter().flatten() {
            write_color(&mut writer, pixel_color);
        }

        // Explicitly flush to save safely to disk
        writer.flush()?;
        println!("\nPPM image successfully saved to gradient.ppm!");
        Ok(())
    }

    fn initialize(&mut self) {
        self.image_height = ((self.image_width as f64 / self.aspect_ratio) as i32).max(1);

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = self.lookfrom;

        // Determine viewport dimensions.
        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        self.w = (self.lookfrom - self.lookat).normalize();
        self.u = self.vup.cross(&self.w).normalize();
        self.v = self.w.cross(&self.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * self.u; // Vector across viewport horizontal edge
        let viewport_v = viewport_height * -self.v; // Vector down viewport vertical edge

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = self.focus_dist * degrees_to_radians(self.defocus_angle / 2.0).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    // Construct a camera ray originating from the defocus disk and directed at a randomly
    // sampled point around the pixel location i, j.
    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(&ray_origin, &ray_direction)
    }

    // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
    fn sample_square() -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    // Returns a random point in the camera defocus disk.
    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_in_unit_disk();
        self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }

    fn ray_color(&self, r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        // Start at 0.001 so floating-point error doesn't re-hit the surface (shadow acne).
        let Some(rec) = world.hit(r, Interval::new(0.001, f64::INFINITY)) else {
            return self.background.unwrap_or_else(|| Self::sky_color(r));
        };

        // Light the surface gives off, plus light it scatters from elsewhere.
        let color_from_emission = rec.mat.emitted(rec.u, rec.v, &rec.p);
        match rec.mat.scatter(r, &rec) {
            Some((attenuation, scattered)) => {
                color_from_emission
                    + attenuation.component_mul(&self.ray_color(&scattered, depth - 1, world))
            }
            None => color_from_emission,
        }
    }

    // Vertical white-to-blue gradient used when no background color is set.
    fn sky_color(r: &Ray) -> Color {
        // 1. Get the unit vector of the ray's direction (.normalize() in nalgebra)
        let unit_direction: Vec3 = r.dir.normalize();

        // 2. Map the y component from [-1.0, 1.0] to [0.0, 1.0]
        let a = 0.5 * (unit_direction.y + 1.0);

        // 3. Linear blend (lerp) between white and light blue
        let white = Color::new(1.0, 1.0, 1.0);
        let blue = Color::new(0.5, 0.7, 1.0);

        (1.0 - a) * white + a * blue
    }
}
