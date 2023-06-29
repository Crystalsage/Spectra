mod vector;
mod color;
mod image_writer;
mod ray;
mod hittable; 
mod sphere;
mod utility;
mod camera;
mod material;

use crate::camera::Camera;
use crate::hittable::Hittables;
use crate::material::Material;
use crate::image_writer::{Image, Pixels};
use crate::color::{make_ray_color, make_color};
use crate::material::MaterialType;
use crate::sphere::Sphere;
use crate::vector::{Point3, Vec3, Color};
use crate::ray::Ray;
use crate::utility::random_f64;


fn main() {
    let file_path: Option<&str> = Some("render.ppm");

    let aspect_ratio = 16.0 / 9.0;
    let width: usize = 1920;
    let height: usize = (width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // Materials
    let material_ground = Material::new(MaterialType::Lambertian(Color::new(0.8, 0.8, 0.0)));
    let material_center = Material::new(MaterialType::Lambertian(Color::new(0.1, 0.2, 0.5)));

    let material_left = Material::new(MaterialType::Dielectric(1.5));
    let material_right = Material::new(MaterialType::Metal(Color::new(0.8, 0.6, 0.2), 0.0));

    // World
    let mut world: Hittables<Sphere> = Hittables::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground));
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center));
    world.add(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left));
    world.add(Sphere::new(Point3::new(-1.0, 0.0, -1.0), -0.4, material_left));
    world.add(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right));

    // Camera
    let cam = Camera::default();

    let mut pixels: Pixels = vec![vec![0_i64; width as usize]; height as usize];

    // And.... render....!
    for y in 0..height {
        // println!("Scan lines remaining: {}", height-y);
        for x in 0..width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u: f64 = (x as f64 + random_f64(None, None)) / (width - 1) as f64;
                let v: f64 = ((height - y) as f64 + random_f64(None, None)) / (height - 1) as f64;
                let ray: Ray = cam.get_ray(u, v);
                pixel_color += make_ray_color(ray, &world, max_depth);

            }
            pixels[y][x] = make_color(pixel_color, samples_per_pixel);
        }
    }

    let image = Image::new(width, height, pixels);
    assert!(matches!(image.write_to_file(file_path), Ok(())));
}
