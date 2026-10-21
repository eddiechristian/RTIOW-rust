// Book 3, chapters 3-4: estimate the integral of x² over [0,2] (exactly 8/3)
// with Monte Carlo, sampling x from three different PDFs. The closer the PDF's
// shape is to the integrand, the less noisy the estimate; a PDF proportional
// to x² gives the exact answer from every single sample.
//
// Run with: cargo run --release --example integrate_x_sq

struct Sampler {
    name: &'static str,
    // Turns a uniform random number in [0,1) into a sample x...
    sample: fn(f64) -> f64,
    // ...and gives the density p(x) of producing that x.
    pdf: fn(f64) -> f64,
}

fn main() {
    let samplers = [
        // Uniform on [0,2]: p(x) = 1/2.
        Sampler {
            name: "uniform  p(x) = 1/2   ",
            sample: |r| 2.0 * r,
            pdf: |_| 0.5,
        },
        // Linear: p(x) = x/2, sampled by inverting its CDF x²/4.
        Sampler {
            name: "linear   p(x) = x/2   ",
            sample: |r| (4.0 * r).sqrt(),
            pdf: |x| x / 2.0,
        },
        // Matches the integrand: p(x) = 3x²/8, inverse CDF of x³/8.
        Sampler {
            name: "matching p(x) = 3x²/8 ",
            sample: |r| (8.0 * r).cbrt(),
            pdf: |x| 3.0 * x * x / 8.0,
        },
    ];

    let n = 1_000_000;
    let exact = 8.0 / 3.0;
    println!("integral of x² over [0,2], exact {exact:.12}, {n} samples");

    for s in &samplers {
        let mut sum = 0.0;
        for _ in 0..n {
            // 1 - r is in (0,1], which keeps x away from 0 where some pdfs are 0.
            let x = (s.sample)(1.0 - rand::random::<f64>());
            sum += x * x / (s.pdf)(x);
        }
        let estimate = sum / n as f64;
        println!(
            "  {}: {estimate:.12}  (error {:.2e})",
            s.name,
            (estimate - exact).abs()
        );
    }
}
