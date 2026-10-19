use nalgebra::Vector3;

pub type Vec3 = Vector3<f64>;
pub type Point3 = nalgebra::Point3<f64>;

// Returns a random real in [0,1).
pub fn random_double() -> f64 {
    rand::random::<f64>()
}

// Returns a random real in [min,max).
pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

#[allow(dead_code)] // part of the book's vec3 API; not called yet
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
