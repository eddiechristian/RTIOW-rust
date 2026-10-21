// Book 3, chapter 6: print points on the unit sphere generated from two
// uniform random numbers (phi = 2π r1, z = 1 - 2 r2), one "x y z" per line,
// for plotting with any 3D scatter tool (e.g. gnuplot's splot).
//
// Run with: cargo run --release --example sphere_plot > sphere.txt

use std::f64::consts::PI;

fn main() {
    for _ in 0..200 {
        let r1 = rand::random::<f64>();
        let r2 = rand::random::<f64>();

        let phi = 2.0 * PI * r1;
        let z = 1.0 - 2.0 * r2;
        let r = (1.0 - z * z).sqrt();

        println!("{} {} {}", phi.cos() * r, phi.sin() * r, z);
    }
}
