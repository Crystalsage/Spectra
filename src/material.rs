use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::Color;
use crate::utility::random_f64;
use crate::vector::Vec3;

#[derive(Copy, Clone)]
pub enum MaterialType {
    Lambertian(Color),

    /// Metal(color, fuzziness)
    Metal(Color, f64),

    // Dielectric(refraction_index)
    Dielectric(f64)
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
            MaterialType::Dielectric(ir) => Material::scatter_dielectric(ir, r_in, rec, attentuation, scattered),
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

    fn scatter_dielectric(ir: f64, r_in: &Ray, rec: &HitRecord, attentuation: &mut Color, scattered: &mut Ray) -> bool {
        *attentuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio: f64 = match rec.front_face {
            true => 1.0/ir,
            false => ir
        };

        let unit_direction = r_in.direction.unit_vector();
        let cos_theta = (-unit_direction).dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - (cos_theta * cos_theta)).sqrt();

        let cannot_reflect = (refraction_ratio * sin_theta) > 1.0;

        let direction: Vec3 = match cannot_reflect || Self::reflectance(cos_theta, refraction_ratio) > random_f64(None, None) {
            true => Vec3::reflect(unit_direction, rec.normal),
            false => Vec3::refract(&unit_direction, &rec.normal, refraction_ratio),
        };

        *scattered = Ray::new(rec.p, direction);

        true
    }


    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}
