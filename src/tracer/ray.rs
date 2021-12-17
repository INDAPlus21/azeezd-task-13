use crate::utils::{Vector3, Colour, INFINITY, ORIGIN};
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
    pub fn colour(ray: &Ray, world: &World, depth: usize) -> Colour {
        let mut hit_rec = HitRecord::new_empty();

        if depth <= 0 {
            return ORIGIN; // black colour
        }

        if world.hit(ray, 0.001, INFINITY, &mut hit_rec) {
            let target = hit_rec.origin + hit_rec.normal + Vector3::random_in_unit_sphere();
            return 0.5 * Ray::colour(&Ray::new(hit_rec.origin, target - hit_rec.origin), &world, depth - 1);
        }

        let unit_dir = ray.direction.unit();
        let t = 0.5 * (unit_dir.y + 1.0);
        (1.0 - t)*Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
    }
}