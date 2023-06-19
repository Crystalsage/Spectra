use std::ops;

struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    /// Initialize a `Vec3` at origin.
    fn default() -> Self {
        Vec3 { 
            x: 0.0, 
            y: 0.0,  
            z: 0.0 
        }
    }

    /// Create a new `Vec3` at given co-ordinates.
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { 
            x, 
            y,
            z
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        
    }
}
