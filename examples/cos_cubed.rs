// Book 3, chapter 6: integrate cos³(theta) over the hemisphere (exactly π/2)
// twice: once with directions spread uniformly over the hemisphere, and once
// with cosine-weighted directions, the distribution the renderer's Lambertian
// material now uses. The cosine-weighted estimate is noticeably less noisy.
//
// Run with: cargo run --release --example cos_cubed

use std::f64::consts::PI;

fn main() {
    let n = 1_000_000;
    let exact = PI / 2.0;
    println!("integral of cos³θ over the hemisphere, exact {exact:.10}, {n} samples");

    // Uniform over the hemisphere: z = cos(theta) is uniform in [0,1], p = 1/(2π).
    let mut sum = 0.0;
    for _ in 0..n {
        let z = rand::random::<f64>();
        sum += z * z * z / (1.0 / (2.0 * PI));
    }
    let estimate = sum / n as f64;
    println!(
        "  uniform hemisphere: {estimate:.10}  (error {:.2e})",
        (estimate - exact).abs()
    );

    // Cosine-weighted: z = sqrt(1 - r2), p = cos(theta)/π.
    let mut sum = 0.0;
    for _ in 0..n {
        let z = (1.0 - rand::random::<f64>()).sqrt();
        sum += z * z * z / (z / PI);
    }
    let estimate = sum / n as f64;
    println!(
        "  cosine-weighted:    {estimate:.10}  (error {:.2e})",
        (estimate - exact).abs()
    );
}
