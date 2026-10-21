use nalgebra::Vector3;

pub type Vec3 = Vector3<f64>;
pub type Point3 = nalgebra::Point3<f64>;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

// Returns a random real in [0,1).
pub fn random_double() -> f64 {
    rand::random::<f64>()
}

// Returns a random real in [min,max).
pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

// Returns a random integer in [min,max].
pub fn random_int(min: i32, max: i32) -> i32 {
    random_double_range(min as f64, (max + 1) as f64) as i32
}

pub fn random_vec3() -> Vec3 {
    let mut vec = Vec3::default();
    vec.x = random_double();
    vec.y = random_double();
    vec.z = random_double();
    vec
}

pub fn random_vec3_min_max(min: f64, max: f64) -> Vec3 {
    let mut vec = Vec3::default();
    vec.x = random_double_range(min, max);
    vec.y = random_double_range(min, max);
    vec.z = random_double_range(min, max);
    vec
}

// Returns a random unit vector, uniformly distributed over the unit sphere.
pub fn random_unit_vector() -> Vec3 {
    loop {
        // Rejection-sample a point inside the unit sphere, then normalize it.
        let p = random_vec3_min_max(-1.0, 1.0);
        let lensq = p.norm_squared();
        // Skip tiny vectors whose squared length could underflow to zero.
        if 1e-160 < lensq && lensq <= 1.0 {
            return p / lensq.sqrt();
        }
    }
}

// Returns a random direction on the +z hemisphere, with density cos(theta)/pi.
pub fn random_cosine_direction() -> Vec3 {
    let r1 = random_double();
    let r2 = random_double();

    let phi = 2.0 * std::f64::consts::PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();
    let z = (1.0 - r2).sqrt();

    Vec3::new(x, y, z)
}

// Returns a random point inside the unit disk in the xy plane.
pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(
            random_double_range(-1.0, 1.0),
            random_double_range(-1.0, 1.0),
            0.0,
        );
        if p.norm_squared() < 1.0 {
            return p;
        }
    }
}

// Return true if the vector is close to zero in all dimensions.
#[allow(dead_code)] // Lambertian used it until cosine-PDF sampling (Book 3); kept as in the book
pub fn near_zero(v: &Vec3) -> bool {
    let s = 1e-8;
    v.x.abs() < s && v.y.abs() < s && v.z.abs() < s
}

// Reflect v about the unit normal n.
pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

// Refract unit vector uv through the surface with unit normal n (Snell's law).
pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = (-uv).dot(n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -((1.0 - r_out_perp.norm_squared()).abs().sqrt()) * n;
    r_out_perp + r_out_parallel
}

// Returns a random unit vector on the hemisphere around the given normal.
#[allow(dead_code)] // replaced by Lambertian scattering in 9.4; kept as in the book
pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if on_unit_sphere.dot(normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}
