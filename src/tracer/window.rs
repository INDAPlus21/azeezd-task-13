extern crate minifb;
use minifb::{Key, Window as MFBWindow, WindowOptions};
use super::{objects::World, Camera};
use crate::utils::{IMG_WIDTH, IMG_HEIGHT, Vector3, VIEWPORT_WIDTH, VIEWPORT_HEIGHT, FOCAL_LENGTH, ORIGIN};

/// # `window`
/// Holds the minifb buffer `Vec<u32>`, minifb window `Window` and camera `Camera`. Used to display the ray tracing on a window
pub struct Window {
    buffer: Vec<u32>,
    window: MFBWindow,
    camera: Camera
}

impl Window {

    /// # `new`
    /// Creates a new empty window using the default settings (under `utils` mod)
    pub fn new() -> Window {
        Window {
            buffer: vec![0; IMG_WIDTH * IMG_HEIGHT],
            window: MFBWindow::new("Ray Tracer", IMG_WIDTH, IMG_HEIGHT, WindowOptions::default()).unwrap(),
            camera: Camera::new(ORIGIN, VIEWPORT_WIDTH, VIEWPORT_HEIGHT, FOCAL_LENGTH)
        }
    }

    /// # `update`
    /// Updates the window by calculating the ray tracing again in the given `World`
    pub fn update(&mut self, world: &mut World) {
        while self.window.is_open() {
            self.camera.update_buffer(world, &mut self.buffer);
            self.window.update_with_buffer(&mut self.buffer, IMG_WIDTH, IMG_HEIGHT);
        }
    }
}