use super::{Object, HitRecord, Ray};

pub struct World {
    pub objects: Vec<Box<dyn Object>>,
}

impl World {
    pub fn new_empty() -> World {
        World {
            objects: Vec::new()
        }
    }
}

impl Object for World {
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