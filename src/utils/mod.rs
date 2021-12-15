mod vector;
mod image;

pub const ASPECT_RATIO : f32 = 16.0 / 9.0;
pub const IMG_WIDTH : usize = 400;
pub const IMG_HEIGHT : usize = (IMG_WIDTH as f32 / ASPECT_RATIO) as usize;
pub const VIEWPORT_HEIGHT : f32 = 2.0;
pub const VIEWPORT_WIDTH : f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
pub const FOCAL_LENGTH : f32 = 1.0;

pub const INFINITY: f32 = std::f32::INFINITY;

pub use self::{
    vector::Vector3,
    image::Image
};

/// # `Colour`
/// Alias for `Vector3`
/// 
/// The x, y, z components should be interperted as the Red, Green, Blue values of the colour 
pub type Colour = Vector3;