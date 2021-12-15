use super::Colour;
use std::path::Path;
extern crate image;

/// # `Image`
/// Struct to save pixels stored as `Colour`s and the width and height of an image. Has an function to save the image to png
pub struct Image {
    pub pixels: Vec<Colour>,
    width: usize,
    height: usize
}

impl Image {
    /// # `new`
    /// Creates an new empty (all black pixels) image with the size as the given width and height as `usize`
    pub fn new(width: usize, height: usize) -> Image {
        Image {
            pixels: vec![Colour::new(0.0, 0.0, 0.0); width * height],
            width: width,
            height: height
        }
    }

    pub fn at(&mut self, i: usize, j: usize) -> &mut Colour {
        &mut self.pixels[j * self.width + i]
    }

    /// # `save`
    /// Saves the image as a .png file (using the Image crate) with given name as `String`
    pub fn save(&self, output_name: String) -> std::io::Result<()> {
        // Handle lack of extension
        let output_name = if output_name.ends_with(".png") {output_name} else {output_name + ".png"};

        // Store colour as bytes of RGB, hence *3 at the end
        let mut byte_array : Vec<u8> = Vec::with_capacity(self.width * self.height * 3);

        // Store pixels from top to bottom while converting the colours from the range (0.0, 1.0) to bytes in (0,255)
        for j in (0..self.height).rev() {
            for i in 0..self.width {
                let byte = self.pixels[j * self.width + i];
                byte_array.push((255.99 * byte.x) as u8);
                byte_array.push((255.99 * byte.y) as u8); 
                byte_array.push((255.99 * byte.z) as u8);
            }
        }

        // Save using the Image crate
        image::save_buffer(&Path::new(output_name.as_str()), &byte_array, self.width as u32, self.height as u32, image::ColorType::Rgb8).ok();

        Ok(())
    }
}