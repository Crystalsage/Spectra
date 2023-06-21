// This file includes some constants and utility functions

use rand::{thread_rng, Rng};

const INFINITY: f64 = f64::INFINITY;
const PI: f64 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return (degrees * PI) / 180.0;
}

pub fn random_f64(min: Option<f64>, max: Option<f64>) -> f64 {
    if min.is_none() && max.is_none() {
        let mut rng = thread_rng();
        rng.gen_range(0.0..1.0)
    } else {
        let min = min.unwrap();
        let max = max.unwrap();

        min + (max - min ) * random_f64(None, None)
    }
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { return min; }
    if x > max { return max; }
    return x;
}
