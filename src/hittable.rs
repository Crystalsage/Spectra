use crate::material::{Material, MaterialType};
use crate::vector::{Vec3, Point3, Color};
use crate::ray::Ray;

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Material,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn default() -> Self {
        HitRecord { 
            p: Point3::default(), 
            normal: Vec3::default(), 
            material: Material::new(MaterialType::Lambertian(Color::default())),
            t: 0.0, 
            front_face: false 
        }
    }

    pub fn set_face_normal(self: &mut Self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = match self.front_face {
            true => *outward_normal,
            false => -*outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(self: &Self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}


/// List of Hittable objects
pub struct Hittables<T> where T: Hittable {
    objects: Vec<T>,
}

impl<T: Hittable> Hittables<T> {
    pub fn new(object: T) -> Self {
        Hittables { 
            objects: vec![object],
        }
    }

    pub fn add(self: &mut Self, object: T) {
        self.objects.push(object);
    }

    pub fn clear(self: &mut Self) {
        self.objects.clear()
    }
}

impl<T: Hittable> Hittable for Hittables<T> {
    fn hit(self: &Self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_once: bool = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_once = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        return hit_once;
    }
}
