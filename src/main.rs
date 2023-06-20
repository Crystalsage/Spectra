mod vector;
mod color;
mod image_writer;
mod ray;

use crate::image_writer::{Image, Pixels};
use crate::color::{make_ray_color, make_color};
use crate::vector::{Point3, Vec3};
use crate::ray::Ray;

fn main() {
    let file_path: Option<&str> = Some("render.ppm");

    let aspect_ratio = 16.0 / 9.0;
    let width: usize = 1920;
    let height: usize = (width as f64 / aspect_ratio) as usize;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin: Point3 = Point3::new(0.0, 0.0, 0.0);
    let horizontal: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);

    let mut pixels: Pixels = vec![vec![0_i64; width as usize]; height as usize];

    // And.... render....!
    for y in 0..height {
        // println!("Scan lines remaining: {}", height-y);
        for x in 0..width {
            let u: f64 = x as f64/ (width - 1) as f64;
            let v: f64 = (height - y) as f64/ (height - 1) as f64;

            let r: Ray = Ray::new(origin, lower_left + (u * horizontal) + (v * vertical) - origin);
            pixels[y][x] = make_color(make_ray_color(r));
        }
    }

    let image = Image::new(width, height, pixels);
    assert!(matches!(image.write_to_file(file_path), Ok(())));
}
