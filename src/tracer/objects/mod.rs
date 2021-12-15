use super::Ray;
use crate::utils::{Vector3};

#[derive(Copy, Clone)]
/// # `HitRecord`
/// Structure that holds the 
pub struct HitRecord {
    pub origin: Vector3,
    pub normal: Vector3,
    pub t: f32,
    pub front_face: bool
}

impl HitRecord {
    pub fn new_empty() -> HitRecord {
        let empty_vec = Vector3::new(0.0, 0.0, 0.0);
        HitRecord {
            origin: empty_vec,
            normal: empty_vec,
            t: 0.0,
            front_face: false
        }
    }

    pub fn face_normal(&mut self, ray: &Ray, out_normal: &Vector3) {
        self.front_face = out_normal.dot(ray.direction) < 0.0;
        self.normal = if self.front_face {*out_normal} else {-*out_normal};
    }
}

/// # `Object`
/// Trait used for objects that are hittable by a ray
pub trait Object {
    /// # `hit`
    /// Returns true if the given `Ray` hits the object
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool;
}

mod sphere;
mod world;

pub use self::{
    sphere::Sphere,
    world::World
};
