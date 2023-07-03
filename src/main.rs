mod vector;
mod color;
mod image_writer;
mod ray;
mod hittable; 
mod sphere;
mod utility;
mod camera;
mod material;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

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


fn random_world() -> Hittables<Sphere>{
    let ground_material = Material::new(MaterialType::Lambertian(Color::new(0.5, 0.5, 0.5)));

    let mut world: Hittables<Sphere> = Hittables::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64(None, None);
            let center: Point3 = Point3::new(a as f64 + 0.9 * random_f64(None, None), 0.2, b as f64 + 0.9 * random_f64(None, None));

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Material;

                if choose_mat < 0.5 {
                    let albedo  = Color::random() * Color::random();
                    sphere_material = Material::new(MaterialType::Lambertian(albedo));
                    world.add(Sphere::new(center, 0.2, sphere_material));
                } else if choose_mat < 0.8 {
                    let albedo = Color::random();
                    let fuzz = random_f64(Some(0.0), Some(0.5));
                    sphere_material = Material::new(MaterialType::Metal(albedo, fuzz));
                    world.add(Sphere::new(center, 0.2, sphere_material));
                } else {
                    sphere_material = Material::new(MaterialType::Dielectric(1.5));
                    world.add(Sphere::new(center, 0.2, sphere_material));
                }
            }
        }
    }

    let material1 = Material::new(MaterialType::Dielectric(1.5));
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Material::new(MaterialType::Lambertian(Color::new(0.4, 0.2, 0.1)));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Material::new(MaterialType::Metal(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3));

    world
}

fn main() {
    let file_path: Option<&str> = Some("render.ppm");

    let aspect_ratio = 16.0 / 9.0;
    let width: usize = 1920;
    let height: usize = (width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 500;
    let max_depth = 50;

    
    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Arc::new(Camera::new(lookfrom, lookat, vup, 20.0, aspect_ratio, aperture, dist_to_focus));

    // World
    let world: Arc<Hittables<Sphere>> = Arc::new(random_world());

    // Pixel data
    let pixels: Arc<Mutex<Pixels>> = Arc::new(Mutex::new(vec![vec![0_i64; width as usize]; height as usize]));

    let worker_count = 12;
    let mut results = vec![];

    let st = Instant::now();

    for i in 0..worker_count {
        let inner_pixels = pixels.clone();
        let inner_world = world.clone();
        let inner_cam = cam.clone();

        // And.... render....!
        results.push(thread::spawn(move || {
            for y in 0..height {
                // println!("Scan lines remaining: {}", height-y);
                for x in (i * width/worker_count)..((i+1) * width/worker_count) {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                    for _ in 0..samples_per_pixel {
                        let u: f64 = (x as f64 + random_f64(None, None)) / (width - 1) as f64;
                        let v: f64 = ((height - y) as f64 + random_f64(None, None)) / (height - 1) as f64;
                        let ray: Ray = inner_cam.get_ray(u, v);

                        pixel_color += make_ray_color(ray, &*inner_world, max_depth);
                    }

                    let mut inner_pixels = inner_pixels.lock().unwrap();
                    inner_pixels[y][x] = make_color(pixel_color, samples_per_pixel);
                }
            }
        }));
    }

    for handle in results {
        handle.join().unwrap();
    }

    let pixels = Arc::try_unwrap(pixels).unwrap().into_inner().unwrap();

    let duration = st.elapsed();
    let seconds = duration.as_secs() % 60;
    let minutes = (duration.as_secs() / 60) % 60;
    let hours = (duration.as_secs() / 60) / 60;
    println!("Done calculating pixels in {}h:{}m:{}s", hours, minutes, seconds);

    let st = Instant::now();

    let image = Image::new(width, height, pixels);
    assert!(matches!(image.write_to_file(file_path), Ok(())));

    let duration = st.elapsed();
    let seconds = duration.as_secs() % 60;
    let minutes = (duration.as_secs() / 60) % 60;
    let hours = (duration.as_secs() / 60) / 60;
    println!("Wrote to image in {}:{}:{}", hours, minutes, seconds);
}
