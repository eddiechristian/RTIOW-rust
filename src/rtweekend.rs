use nalgebra::Vector3;

pub type Vec3 = Vector3<f64>;
pub type Point3 = nalgebra::Point3<f64>;

// Returns a random real in [0,1).
pub fn random_double() -> f64 {
    rand::random::<f64>()
}

// Returns a random real in [min,max).
#[allow(dead_code)] // used in later chapters
pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}
