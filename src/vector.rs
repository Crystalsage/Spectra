use std::num::NonZeroU8;
use std::ops;
use std::fmt;
use std::ops::ControlFlow;

use crate::random_f64;

pub type Point3 = Vec3;
pub type Color = Vec3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    /// Initialize a `Vec3` at origin.
    pub fn default() -> Self {
        Vec3 { 
            x: 0.0, 
            y: 0.0,  
            z: 0.0 
        }
    }

    /// Create a new `Vec3` at given co-ordinates.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { 
            x, 
            y,
            z
        }
    }

    pub fn random() -> Self {
        Vec3 {
            x: random_f64(None, None),
            y: random_f64(None, None),
            z: random_f64(None, None),
        }
    }

    pub fn random_with_range(min: f64, max: f64) -> Self {
        Vec3 {
            x: random_f64(Some(min), Some(max)),
            y: random_f64(Some(min), Some(max)),
            z: random_f64(Some(min), Some(max)),
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let v: Vec3 = Vec3::random();
            if v.length_squared() >= 1.0 { continue; }
            return v;
        }
    }

    pub fn random_unit_vector() -> Self {
        Vec3::random_in_unit_sphere().unit_vector()
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            return in_unit_sphere;
        } else {
            return -in_unit_sphere;
        }
    }

    pub fn dot(self: &Self, other: &Self) -> f64  {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self: Self, other: Self) -> Self {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit_vector(self: Self) -> Self {
        let length = self.length();
        self / length
    }

    pub fn length(self: &Self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self: &Self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    
    pub fn near_zero(self: Self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Self {
        v - 2.0 * v.dot(&n) * n
    }

    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Self {
        let cos_theta = (-*uv).dot(n).min(1.0);
        let r_perpendicular = etai_over_etat * (*uv + (*n * cos_theta));
        let r_parallel = -(((1.0 - r_perpendicular.length_squared()).abs()).sqrt()) * *n;
        r_perpendicular + r_parallel
    }
}


impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}


// The methods below overload some operators so that we can use them with our
// `Vec3`.


impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}


/// Allows us to negate the `Vec3`
/// e.g.
/// ```rust
///   let mut vec: Vec3 = Vec3::new(1.0, 2.0, 3.0);
///   vec = -vec;
///   // `vec` is now (-1.0, -2.0, -3.0)
/// ```
impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Can't index into Vec3 with index: {}", index),
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
        self.z = self.z * rhs.z;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x = self.x * (1.0/rhs);
        self.y = self.y * (1.0/rhs);
        self.z = self.z * (1.0/rhs);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_vec3() {
        let vec3 = Vec3::new(2.0, 3.0, 4.0);
        assert_eq!(vec3.x + vec3.y + vec3.z, 9.0);
    }

    #[test]
    fn test_vec3_addition() {
        let vec3 = Vec3::new(1.0, 2.0, 3.0);
        let another = Vec3::new(2.0, 4.0, 6.0);

        assert_eq!(vec3 + another, Vec3::new(3.0, 6.0, 9.0));
    }
}
