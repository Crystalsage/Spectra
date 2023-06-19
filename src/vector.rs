use std::ops;
use std::fmt;


type Point3 = Vec3;
type Color = Vec3;

#[derive(Debug, PartialEq)]
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

    fn dot(self: Self, other: Self) -> f64  {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn cross(self: Self, other: Self) -> Self {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    fn unit_vector(self: Self) -> Self {
        let length = self.length();
        self / length
    }

    fn length(self: &Self) -> f64 {
        self.length_squared().sqrt()
    }

    fn length_squared(self: &Self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
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
