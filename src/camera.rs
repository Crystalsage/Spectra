use crate::{Vec3, Point3, Ray};
use crate::utility;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Point3, lookat: Point3, vup: Vec3, vfov: f64, aspect_ratio: f64) -> Self {
        let theta = utility::degrees_to_radians(vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let focal_length = 1.0;

        let w = (lookfrom - lookat).unit_vector();
        let u = (vup.cross(w)).unit_vector();
        let v = w.cross(u);

        let origin: Point3 = lookfrom;
        let horizontal: Vec3 = viewport_width * u;
        let vertical: Vec3 = viewport_height * v;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - w;

        Camera { 
            origin,
            lower_left_corner,
            horizontal, 
            vertical 
        }
    }

    pub fn get_ray(self: &Self, s: f64, t: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin)
    }
}
