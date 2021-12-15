use crate::utils::{Vector3, Colour, INFINITY};
use super::objects::{Object, World, HitRecord};

/// # `Ray`
/// Structure of the ray that is cast and traced
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3
}

impl Ray {
    /// # `new`
    /// Creates a new Ray using given `Vector3` origin (where it starts) and its direction `Vector3`
    pub fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray {
            origin: origin,
            direction: direction
        }
    }

    /// # `at`
    /// Returns the `Vector3` that starts at the origin of the Ray and scaled by a given parameter t as `f32`
    pub fn at(&self, t: f32) -> Vector3 {
        return self.origin + t * self.direction;
    }

    /// # `colour`
    /// Returns the colour of the ray based on the way it was traced
    pub fn colour(&self, world: &World) -> Colour {
        let mut hit_rec = HitRecord::new_empty();

        if world.hit(&self, 0.0, INFINITY, &mut hit_rec) {
            return 0.5 * (hit_rec.normal + Vector3::new(1.0, 1.0, 1.0))
        }

        let unit_dir = self.direction.unit();
        let t = 0.5 * (unit_dir.y + 1.0);
        (1.0 - t)*Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
    }
}