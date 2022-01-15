use super::{Object, Material, Ray, HitRecord};
use crate::utils::{X_UNIT, Y_UNIT, Z_UNIT};

/// # `Axis`
/// Enum type used to specify the axis which a rectangle is aligned to
pub enum Axis {
    XY, XZ, YZ
}

/// # `Rectangle`
/// Struct that handles an axis-aligned rectangle
pub struct Rectangle {
    pub axis: Axis,
    pub axis_0: (f32, f32),
    pub axis_1: (f32, f32),
    pub k: f32,
    pub material: Material
}

impl Object for Rectangle {
    /// `hit`
    /// Returns true if a ray hits the rectangle using the given boundries 
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        match self.axis { // Calcualte hit based on axis 
            Axis::XY => {return self.hit_xy(ray, t_min, t_max, hit_record);},
            Axis::XZ => {return self.hit_xz(ray, t_min, t_max, hit_record);},
            Axis::YZ => {return self.hit_yz(ray, t_min, t_max, hit_record);}
        }
    }
}

impl Rectangle {
    /// # `new`
    /// Creates a new rectangle and returns it using the given `Axis` of alignment, the boundries in the first and second axis as `(f32, f32)` then the depth in the third as `f32`.
    /// Takes the rectangle's `Material` as well
    pub fn new(axis: Axis, axis_0: (f32, f32), axis_1: (f32, f32), k: f32, material: Material) -> Box<dyn Object> {
        Box::new (Rectangle {
            axis: axis,
            axis_0: axis_0,
            axis_1: axis_1,
            k: k,
            material: material,
        })
    }

    fn hit_xy(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let t = (self.k - ray.origin.z) / ray.direction.z;
        if t < t_min || t > t_max {return false;}

        let x = ray.origin.x + t * ray.direction.x;
        let y = ray.origin.y + t * ray.direction.y;
        if x < self.axis_0.0 || x > self.axis_0.1 || y < self.axis_1.0 || y > self.axis_1.1 {return false;}
        
        hit_record.t = t;
        hit_record.face_normal(ray, &Z_UNIT);
        hit_record.material = Some(self.material);
        hit_record.origin = ray.at(t);
        
        true
    }

    fn hit_xz(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let t = (self.k - ray.origin.y) / ray.direction.y;
        if t < t_min || t > t_max {return false;}

        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;
        if x < self.axis_0.0 || x > self.axis_0.1 || z < self.axis_1.0 || z > self.axis_1.1 {return false;}
        
        hit_record.t = t;
        hit_record.face_normal(ray, &Y_UNIT);
        hit_record.material = Some(self.material);
        hit_record.origin = ray.at(t);
        
        true
    }

    fn hit_yz(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let t = (self.k - ray.origin.x) / ray.direction.x;
        if t < t_min || t > t_max {return false;}

        let y = ray.origin.y + t * ray.direction.y;
        let z = ray.origin.z + t * ray.direction.z;
        if y < self.axis_0.0 || y > self.axis_0.1 || z < self.axis_1.0 || z > self.axis_1.1 {return false;}
        
        hit_record.t = t;
        hit_record.face_normal(ray, &X_UNIT);
        hit_record.material = Some(self.material);
        hit_record.origin = ray.at(t);
        
        true
    }
}