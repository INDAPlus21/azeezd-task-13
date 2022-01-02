use super::{Ray, Material};
use crate::utils::{Vector3, ORIGIN};

#[derive(Copy, Clone)]
/// # `HitRecord`
/// Structure that holds the informatiom about hit object, its normal and orientation
pub struct HitRecord {
    pub origin: Vector3,
    pub normal: Vector3,
    pub material: Option<Material>,
    pub t: f32,
    pub front_face: bool
}

impl HitRecord {
    /// # `new_empty`
    /// Creates a new empty HitRecord
    pub fn new_empty() -> HitRecord {
        let empty_vec = ORIGIN;
        HitRecord {
            origin: empty_vec,
            normal: empty_vec,
            material: None,
            t: 0.0,
            front_face: false
        }
    }

    /// # `face_normal`
    /// Changes the HitRecord's information about if the ray hit from outside the object or inside
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
mod rectangle;

pub use self::{
    sphere::Sphere,
    world::World,
    rectangle::{
        Axis,
        Rectangle
    }
};
