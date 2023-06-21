use std::f64::INFINITY;

use crate::hittable::{Hittable, HitRecord};
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::image_writer::Color32;
use crate::vector::{Point3, Color};
use crate::utility::clamp;


fn intersects_sphere(center: Point3, radius: f64, ray: &Ray) -> f64  {
    let oc: Vec3 = ray.origin - center;
    let a = ray.direction.dot(&ray.direction);
    let b = 2.0 * oc.dot(&ray.direction);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = (b * b) - (4.0 * a * c);
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt()) / (2.0 * a);
    }

}

pub fn make_ray_color<T>(ray: Ray, world: &T, depth: u32) -> Color 
where T: Hittable
{
    let mut rec = HitRecord::default();
    
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if world.hit(&ray, 0.001, INFINITY, &mut rec) {
        let target: Point3 = rec.p + Vec3::random_in_hemisphere(&rec.normal);
        return 0.5 * make_ray_color(Ray::new(rec.p, target - rec.p), world, depth - 1);
    }

    let unit_direction: Vec3 = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    ((1.0 - t) * Color::new(1.0, 1.0, 1.0)) + (t * Color::new(0.5, 0.7, 1.0))
}

pub fn make_color(point: Point3, samples_per_pixel: u32) -> Color32 {
    let scale = 1.0 / samples_per_pixel as f64;

    let r = (256.0 * clamp((point.x * scale).sqrt(), 0.0, 0.999)) as i64;
    let g = (256.0 * clamp((point.y * scale).sqrt(), 0.0, 0.999)) as i64;
    let b = (256.0 * clamp((point.z * scale).sqrt(), 0.0, 0.999)) as i64;


    (b << (8 * 2) | g << (8 * 1) | r << (8 * 0)) as Color32
}
