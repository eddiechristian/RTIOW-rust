// Book 3, chapter 5: Monte Carlo over the sphere of directions. Integrates
// cos²(theta) over all directions (exactly 4π/3) using uniformly random unit
// vectors, whose density is 1/(4π) everywhere.
//
// Run with: cargo run --release --example sphere_importance

use std::f64::consts::PI;

// Uniform random direction: rejection-sample the unit ball, then normalize.
fn random_unit_vector() -> [f64; 3] {
    loop {
        let p: [f64; 3] = std::array::from_fn(|_| 2.0 * rand::random::<f64>() - 1.0);
        let len_sq = p.iter().map(|c| c * c).sum::<f64>();
        if 1e-160 < len_sq && len_sq <= 1.0 {
            let len = len_sq.sqrt();
            return p.map(|c| c / len);
        }
    }
}

fn main() {
    let n = 1_000_000;
    let pdf = 1.0 / (4.0 * PI);

    let mut sum = 0.0;
    for _ in 0..n {
        let d = random_unit_vector();
        let cos_theta = d[2]; // angle from +z
        sum += cos_theta * cos_theta / pdf;
    }

    let exact = 4.0 * PI / 3.0;
    let estimate = sum / n as f64;
    println!("integral of cos²θ over the sphere, {n} samples");
    println!(
        "  estimate {estimate:.10}, exact {exact:.10} (error {:.2e})",
        (estimate - exact).abs()
    );
}
