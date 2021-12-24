use rand::{Rng};

mod vector;
mod image;

pub const ASPECT_RATIO : f32 = 16.0 / 9.0;
pub const IMG_WIDTH : usize = 800;
pub const IMG_HEIGHT : usize = (IMG_WIDTH as f32 / ASPECT_RATIO) as usize;
pub const VIEWPORT_HEIGHT : f32 = 2.0;
pub const VIEWPORT_WIDTH : f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
pub const FOCAL_LENGTH : f32 = 1.0;
pub const BOUNCE_AMOUNT : usize = 10;
pub const SAMPLES : usize = 100;
pub const SAMPLE_SCALE : f32 = 1.0 / SAMPLES as f32;

pub const INFINITY: f32 = std::f32::INFINITY;
pub const ORIGIN: Vector3 = Vector3 {x: 0.0, y: 0.0, z: 0.0};
pub const X_UNIT: Vector3 = Vector3 {x: 1.0, y: 0.0, z: 0.0};
pub const Y_UNIT: Vector3 = Vector3 {x: 0.0, y: 1.0, z: 0.0};
pub const Z_UNIT: Vector3 = Vector3 {x: 0.0, y: 0.0, z: 1.0};

// small value
pub const EPSILON: f32 = 1.0e-8;


pub use self::{
    vector::Vector3,
    image::Image
};

/// # `Colour`
/// Alias for `Vector3`
/// 
/// The x, y, z components should be interperted as the Red, Green, Blue values of the colour 
pub type Colour = Vector3;

impl Colour {
    /// # `to_u32`
    /// Returns the `u32` representation (0RGB in range[0,255]) 
    pub fn to_u32(&self) -> u32 {
       ((255.99 * self.x) as u32) << 16|
       ((255.99 * self.y) as u32) << 8 |
       ((255.99 * self.z) as u32)
    }

    /// # `clamp`
    /// Clamps the values of the colours to be in between the range [0, 1.0]
    pub fn clamp(colour: &Colour) -> Colour {
        Colour {
            x: colour.x.max(0.0).min(1.0),
            y: colour.y.max(0.0).min(1.0),
            z: colour.z.max(0.0).min(1.0)
        }
    } 
}

/// # `random_f32`
/// returns a random value between in the range [0, 1.0)
pub fn random_f32() -> f32 {
    let mut rng = rand::thread_rng();

    rng.gen_range(0.0..1.0)
}