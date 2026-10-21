use crate::rtweekend::{Point3, Vec3, random_int, random_vec3_min_max};

const POINT_COUNT: usize = 256;

pub struct Perlin {
    randvec: [Vec3; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        Self {
            randvec: std::array::from_fn(|_| random_vec3_min_max(-1.0, 1.0).normalize()),
            perm_x: Self::generate_perm(),
            perm_y: Self::generate_perm(),
            perm_z: Self::generate_perm(),
        }
    }

    // Smooth noise in roughly [-1,1].
    pub fn noise(&self, p: &Point3) -> f64 {
        let (fx, fy, fz) = (p.x.floor(), p.y.floor(), p.z.floor());
        let (u, v, w) = (p.x - fx, p.y - fy, p.z - fz);
        let (i, j, k) = (fx as i64, fy as i64, fz as i64);

        // Random gradient vectors at the 8 lattice corners around p. `& 255`
        // wraps negative coordinates too, since i64 is two's complement.
        let c: [[[Vec3; 2]; 2]; 2] = std::array::from_fn(|di| {
            std::array::from_fn(|dj| {
                std::array::from_fn(|dk| {
                    self.randvec[self.perm_x[((i + di as i64) & 255) as usize]
                        ^ self.perm_y[((j + dj as i64) & 255) as usize]
                        ^ self.perm_z[((k + dk as i64) & 255) as usize]]
                })
            })
        });

        Self::perlin_interp(&c, u, v, w)
    }

    // Sum of noise at doubling frequencies and halving weights.
    pub fn turb(&self, p: &Point3, depth: usize) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }

    fn generate_perm() -> [usize; POINT_COUNT] {
        let mut p: [usize; POINT_COUNT] = std::array::from_fn(|i| i);
        // Fisher-Yates shuffle.
        for i in (1..POINT_COUNT).rev() {
            let target = random_int(0, i as i32) as usize;
            p.swap(i, target);
        }
        p
    }

    fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        // Hermite smoothing hides the lattice grid.
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        for (i, plane) in c.iter().enumerate() {
            for (j, row) in plane.iter().enumerate() {
                for (k, grad) in row.iter().enumerate() {
                    let (fi, fj, fk) = (i as f64, j as f64, k as f64);
                    let weight_v = Vec3::new(u - fi, v - fj, w - fk);
                    accum += (fi * uu + (1.0 - fi) * (1.0 - uu))
                        * (fj * vv + (1.0 - fj) * (1.0 - vv))
                        * (fk * ww + (1.0 - fk) * (1.0 - ww))
                        * grad.dot(&weight_v);
                }
            }
        }

        accum
    }
}
