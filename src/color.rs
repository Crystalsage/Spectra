use crate::vector::Vec3;
use crate::ray::Ray;
use crate::image_writer::Color32;
use crate::vector::{Point3, Color};


fn intersects_sphere(center: Point3, radius: f64, ray: &Ray) -> bool  {
    let oc: Vec3 = ray.origin - center;
    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * oc.dot(ray.direction);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = (b * b) - (4.0 * a * c);
    discriminant > 0.0
}

pub fn make_ray_color(ray: Ray) -> Color {
    if intersects_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, &ray) {
        return Color::new(1.0, 0.0, 0.0);
    }

    let unit_direction: Vec3 = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    ((1.0 - t) * Color::new(1.0, 1.0, 1.0)) + (t * Color::new(0.5, 0.7, 1.0))
}

pub fn make_color(point: Point3) -> Color32 {
    let r = (255.999 * point.x) as i64 ;
    let g = (255.999 * point.y) as i64;
    let b = (255.999 * point.z) as i64;

    (b << (8 * 2) | g << (8 * 1) | r << (8 * 0)) as Color32
}
