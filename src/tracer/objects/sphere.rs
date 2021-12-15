use crate::utils::Vector3;
use super::{Object, Ray, HitRecord};

pub struct Sphere {
    center: Vector3,
    radius: f32
}

impl Sphere {
    pub fn new(center: Vector3, radius: f32) -> Box<dyn Object> {
        Box::new ( Sphere {
            center: center,
            radius: radius
        } )
    }
}

impl Object for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let orig_center = ray.origin - self.center;
        let a = ray.direction.norm_squared();
        let b_half = orig_center.dot(ray.direction);
        let c = orig_center.norm_squared() - self.radius * self.radius;

        let discriminant = b_half * b_half - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrt_disc = discriminant.sqrt();
        let mut root = (-b_half - sqrt_disc) / a;

        if root < t_min || root > t_max {
            root = (-b_half + sqrt_disc) / a;
            if root < t_min || root > t_max  {
                return false;
            }
        }


        hit_record.t = root;
        hit_record.origin = ray.at(root);
        let normal = (hit_record.origin - self.center) / self.radius;
        hit_record.face_normal(ray, &normal);

        return true;
    }
}