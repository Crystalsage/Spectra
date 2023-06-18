use std::error;
use std::fs::File;
use std::io::Write;

const FILE_PATH: &str = "render.ppm";

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;
type Color32 = i64;
type Pixels = Vec<Vec<Color32>>;

struct Image {
    width: usize,
    height: usize,
    pixels: Pixels,
}

impl Image {
    fn default() -> Self {
        Image { 
            width: 0,
            height: 0,
            pixels: vec![vec![]] 
        }
    }

    fn new(width: usize, height: usize, pixels: Pixels) -> Self {
        Image { 
            width,
            height,
            pixels
        }
    }

    fn write_to_file(self: Self) -> Result<()> {
       let mut file = File::create(FILE_PATH)?;
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

    #[test]
    fn test_image_write() {
        let width: usize = 256;
        let height: usize = 256;

        let mut pixels: Pixels = vec![vec![0_i64; width as usize]; height as usize];

        for y in 0..height {
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
        assert!(matches!(image.write_to_file(), Ok(())));
    }
}
