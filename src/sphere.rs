use crate::material::Material;
use crate::{Vec3, Point3};
use crate::hittable::{HitRecord, Hittable};
use crate::Ray;


pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Material) -> Self {
        Sphere { 
            center, 
            radius,
            material
        }
    }
}

impl Hittable for Sphere {
    fn hit(self: &Self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = (half_b * half_b) - (a * c);

        if discriminant < 0.0 { return false; }

        let dis_sqrt = discriminant.sqrt();
        let mut root = (-half_b - dis_sqrt) / a;
        if root < t_min || root > t_max {
            root = (-half_b - dis_sqrt) / a;
            if root < t_min || root > t_max { return false; }
        }

        rec.t = root;
        rec.p = ray.at(root);

        let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);
        rec.material = self.material;

        return true;
    }
}
