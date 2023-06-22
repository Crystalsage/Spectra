use crate::vector::{Point3, Vec3};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn default() -> Self {
        Ray {
            origin: Point3::default(),
            direction: Vec3::default(),
        }
    }

    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { 
            origin, 
            direction
        }
    }

    pub fn at(self: &Self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
