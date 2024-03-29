use std::error;
use std::fs::File;
use std::io::Write;

/// The default file path, if not provided.
const DEFAULT_FILE_PATH: &str = "render.ppm";

/// Let's Box any errors!
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Represent colors with this type.
pub type Color32 = i64;

/// A 2-D collection of pixels. Index as `pixels[y][x]`.
pub type Pixels = Vec<Vec<Color32>>;


/// The collection associated with a particular image. 
pub struct Image {
    /// Width of the image.
    width: usize,

    /// Height of the image.
    height: usize,

    /// Collection of pixels.
    /// This is supposed to be populated by the time we write the image.
    pixels: Pixels,
}

impl Image {
    pub fn default() -> Self {
        Image { 
            width: 0,
            height: 0,
            pixels: vec![vec![]] 
        }
    }

    pub fn new(width: usize, height: usize, pixels: Pixels) -> Self {
        Image { 
            width,
            height,
            pixels
        }
    }

    /// Write the `Image` data to a `.ppm` file. 
    /// Note that the order is (B, G, R)
    pub fn write_to_file(self: Self, file_path: Option<&str>) -> Result<()> {
       let mut file = File::create("renders/".to_owned() + file_path.unwrap_or(DEFAULT_FILE_PATH))?;
       file.write_all(format!("P6\n{} {} 255\n", self.width, self.height).as_bytes())?;

       let mut all_bytes: Vec<u8> = Vec::new();

       for y in 0..self.height as usize {
           for x in 0..self.width as usize {
               let pixel: i64 = self.pixels[y][x];

               // Extract red component
               all_bytes.push(((pixel&0x0000FF) >> 8*0) as u8);

               // Extract green component
               all_bytes.push(((pixel&0x00FF00) >> 8*1) as u8);

               // Extract blue component
               all_bytes.push(((pixel&0xFF0000) >> 8*2) as u8);
           }
       }

       file.write_all(&all_bytes).unwrap();

       Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    fn test_image_write() {
        let file_path: Option<&str> = Some("test_render.ppm");

        let width: usize = 512;
        let height: usize = 512;

        let mut pixels: Pixels = vec![vec![0_i64; width as usize]; height as usize];

        for y in 0..height {
            println!("Scan lines remaining: {}", height-y);
            for x in 0..width {
                let r = (255.999 * ((x as f32) / ((width - 1) as f32))) as i64 ;
                let g = (255.999 * (((height-y-1) as f32) / ((height - 1) as f32))) as i64;
                let b = (255.999 * 0.25) as i64;

                pixels[y][x] = (
                    b << (8 * 2) |
                    g << (8 * 1) |
                    r << (8 * 0)
                ) as Color32;
            }
        }

        let image = Image::new(width, height, pixels);
        assert!(matches!(image.write_to_file(file_path), Ok(())));
    }
}
