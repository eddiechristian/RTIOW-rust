// Book 3, chapter 2: estimate pi by throwing random points at a square and
// counting how many land inside the inscribed circle, then compare plain
// random sampling with stratified (jittered) sampling.
//
// Run with: cargo run --release --example pi

fn random_range(min: f64, max: f64) -> f64 {
    min + (max - min) * rand::random::<f64>()
}

fn main() {
    // Circle of radius 1 inside the square [-1,1]²: area ratio is pi/4.
    let inside = |x: f64, y: f64| x * x + y * y < 1.0;

    let sqrt_n = 1000;
    let n = sqrt_n * sqrt_n;

    // Plain: n independent random points.
    let mut hits_plain = 0;
    for _ in 0..n {
        if inside(random_range(-1.0, 1.0), random_range(-1.0, 1.0)) {
            hits_plain += 1;
        }
    }

    // Stratified: one random point in each cell of a sqrt_n × sqrt_n grid.
    let mut hits_strat = 0;
    for i in 0..sqrt_n {
        for j in 0..sqrt_n {
            let x = 2.0 * ((i as f64 + rand::random::<f64>()) / sqrt_n as f64) - 1.0;
            let y = 2.0 * ((j as f64 + rand::random::<f64>()) / sqrt_n as f64) - 1.0;
            if inside(x, y) {
                hits_strat += 1;
            }
        }
    }

    let estimate = |hits: i32| 4.0 * hits as f64 / n as f64;
    let pi = std::f64::consts::PI;
    println!("{n} samples each");
    println!(
        "  plain:      {:.8}  (error {:.2e})",
        estimate(hits_plain),
        (estimate(hits_plain) - pi).abs()
    );
    println!(
        "  stratified: {:.8}  (error {:.2e})",
        estimate(hits_strat),
        (estimate(hits_strat) - pi).abs()
    );
}
