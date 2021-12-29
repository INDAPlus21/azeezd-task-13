use super::{Object, HitRecord, Ray};
use crate::utils::{Colour, ORIGIN};

/// # `World`
/// The entire world that is rendered. Holds a vector of objects that are hit by the rays
pub struct World {
    pub objects: Vec<Box<dyn Object>>,
    pub background: Colour
}

impl World {
    /// # `new_empty`
    /// Creates an empty world (duh)
    pub fn new_empty() -> World {
        World {
            objects: Vec::new(),
            background: ORIGIN // black background
        }
    }
}

impl Object for World {

    /// # `hit`
    /// Returns true if the ray `Ray` hit the objects in the world within the given parameter boundries t_min, t_max as `f32` and the hit_record of the ray `HitRecord`
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let mut temp: HitRecord = HitRecord::new_empty();
        let mut hit_anything = false;
        let mut closest = t_max;

        for object in self.objects.iter() {
            if object.hit(ray, t_min, closest, &mut temp) {
                hit_anything = true;
                closest = temp.t;
                *hit_record = temp;
            }
        }

        hit_anything
    }
}