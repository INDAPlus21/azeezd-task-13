use crate::utils::Vector3;
use super::{Object, Ray, HitRecord, Material};

/// # `Sphere`
/// A good ol' Sphere with a center and a radius
pub struct Sphere {
    pub center: Vector3,
    pub radius: f32,
    pub material: Material
}

impl Sphere {
    /// # `new`
    /// Creates a new sphere using the given center as `Vector3` and radius as `f32`
    pub fn new(center: Vector3, radius: f32, material: Material) -> Box<dyn Object> {
        Box::new ( Sphere {
            center: center,
            radius: radius,
            material: material
        } )
    }
}

impl Object for Sphere {

    /// # `hit`
    /// Returns true if the ray `Ray` hit the object within the given parameter boundries t_min, t_max as `f32` and the hit_record of the ray `HitRecord`
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

        if root < t_min || t_max < root  {
            root = (-b_half + sqrt_disc) / a;
            if root < t_min || t_max < root   {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.origin = ray.at(hit_record.t);
        let normal = (hit_record.origin - self.center) / self.radius;
        hit_record.face_normal(ray, &normal);
        hit_record.material = Some(self.material);

        return true;
    }
}