use crate::utils::{Vector3, Image, IMG_WIDTH, IMG_HEIGHT, BOUNCE_AMOUNT, Colour, ORIGIN, SAMPLES, SAMPLE_SCALE, random_f32, FOCAL_LENGTH, Y_UNIT};
use super::ray::Ray;
use super::objects::{World, Sphere};
extern crate minifb;
use minifb::{Key, Window, WindowOptions};
/// # `Camera`
/// Structure that stores and handles the perspective from which the scene is rendered
pub struct Camera {
    origin: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
    lower_left_corner: Vector3,
    image: Image
}

impl Camera {
    /// # `new`
    /// Creates a new Camera by taking its origin as `Vector3`, width and height of the viewport, both as `f32` and the focal length as `f32`
    pub fn new(from: Vector3, at: Vector3, vertical_fov: f32, aspect_ratio: f32) -> Camera {
        let theta = vertical_fov.to_radians();
        let height = (theta * 0.5).tan();
        let viewport_height = 2.0 * height;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (from - at).unit();
        let u = Y_UNIT.cross(w).unit(); // Y_UNIT is viewport's up direction
        let v = w.cross(u);

        let horizontal =  viewport_width * u;
        let vertical =  viewport_height * v;

        Camera {
            origin: from,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: from - horizontal / 2.0 - vertical / 2.0 - w,
            image: Image::new(IMG_WIDTH, IMG_HEIGHT)
        }
    }

    /// # `render`
    /// Renders the scene and saves the output as a .png file
    pub fn render(&mut self, world: &mut World, name: String) -> std::io::Result<()> {
        for j in 0..IMG_HEIGHT {
            for i in 0..IMG_WIDTH {
                let mut colour: Colour = ORIGIN; // black
                for _ in 0..SAMPLES {
                    let u = (i as f32 + random_f32()) / (IMG_WIDTH as f32 - 1.0);
                    let v = (j as f32 + random_f32()) / (IMG_HEIGHT as f32 - 1.0);
                    let r = &self.get_ray(u, v);
                    colour += Ray::colour(r, &world, BOUNCE_AMOUNT);
                }
                
                colour = Colour::clamp(&Colour::new((colour.x * SAMPLE_SCALE as f32).sqrt(), (colour.y * SAMPLE_SCALE as f32).sqrt(), (colour.z * SAMPLE_SCALE as f32).sqrt()));
                let pix = self.image.at(i, j);
                *pix = colour;
            }
        }

        self.image.save(format!("images/{}", name).to_string())
    }

    /// # `update_buffer`
    /// Updates the minifb screen buffer using the given `World` to render and the buffer as `Vec<u32>`
    pub fn update_buffer(&mut self, world: &mut World, buffer: &mut Vec<u32>) {
        for j in 0..IMG_HEIGHT {
            for i in 0..IMG_WIDTH {
                let mut colour: Colour = ORIGIN; // black
                for _ in 0..SAMPLES {
                    let u = (i as f32 + random_f32()) / (IMG_WIDTH as f32 - 1.0);
                    let v = (j as f32 + random_f32()) / (IMG_HEIGHT as f32 - 1.0);
                    let r = &self.get_ray(u, v);
                    colour += Ray::colour(r, &world, BOUNCE_AMOUNT);
                }
                
                colour = Colour::clamp(&Colour::new((colour.x * SAMPLE_SCALE as f32).sqrt(), (colour.y * SAMPLE_SCALE as f32).sqrt(), (colour.z * SAMPLE_SCALE as f32).sqrt()));

                buffer[(IMG_HEIGHT - j - 1) * IMG_WIDTH + i] = colour.to_u32();
            }
        }
    }

    /// # `get_ray`
    /// Returns the `Ray` object at the given pixel coordinates u and v as `f32`
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
    }
}
