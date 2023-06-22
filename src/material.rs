use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::Color;
use crate::vector::Vec3;

#[derive(Copy, Clone)]
pub enum MaterialType {
    Lambertian(Color),

    /// Metal(color, fuzziness)
    Metal(Color, f64)
}

impl MaterialType {
    fn new(self: Self) -> Self {
        if let MaterialType::Metal(c, f) = self {
            let fuzz = match f < 1.0 {
                true => f,
                false => 1.0,
            };

            MaterialType::Metal(c, f)
        } else {
            self
        }
    }
}


#[derive(Copy, Clone)]
pub struct Material {
    mat_type: MaterialType
}

impl Material {
    pub fn new(mat_type: MaterialType) -> Self {
        Material { mat_type }
    }

    pub fn scatter(self: Self, r_in: &Ray, rec: &HitRecord, attentuation: &mut Color, scattered: &mut Ray) -> bool {
        match self.mat_type {
            MaterialType::Lambertian(c) => Material::scatter_lambertian(c, r_in, rec, attentuation, scattered),
            MaterialType::Metal(c, f) => Material::scatter_metal(c, f, r_in, rec, attentuation, scattered),
        }
    }

    fn scatter_lambertian(c: Color, r_in: &Ray, rec: &HitRecord, attentuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_direction);
        *attentuation = c;
        true
    }

    fn scatter_metal(c: Color, fuzz: f64, r_in: &Ray, rec: &HitRecord, attentuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = Vec3::reflect(r_in.direction.unit_vector(), rec.normal);
        *scattered = Ray::new(rec.p, reflected + fuzz * Vec3::random_in_unit_sphere());
        *attentuation = c;
        scattered.direction.dot(&rec.normal) > 0.0
    }
}

