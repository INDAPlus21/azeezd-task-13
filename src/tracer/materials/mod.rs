use super::*;
use crate::utils::{Colour, Vector3, random_f32, ORIGIN};

#[derive(Copy, Clone)]
pub enum MaterialType {
    Lambertian,
    Metal(f32),
    Dielectric(f32),
    DiffuseLight
}

#[derive(Copy, Clone)]
pub struct Material {
    pub colour: Colour,
    pub material_type: MaterialType
}

impl Material {
    pub fn new(colour: Colour, material_type: MaterialType) -> Material {
        Material {
            colour: colour,
            material_type: material_type
        }
    }

    pub fn scatter(&self, ray: &Ray, hit_record: &HitRecord, attenuation: &mut Colour, scattered: &mut Ray) -> bool {
        match self.material_type {
            MaterialType::Lambertian => self.lambertian(ray, hit_record, attenuation, scattered),
            MaterialType::Metal(fuzz) => self.metal(ray, hit_record, attenuation, fuzz, scattered),
            MaterialType::Dielectric(refrac_idx) => self.dielectric(ray, hit_record, attenuation, refrac_idx, scattered),
            MaterialType::DiffuseLight => {false}
        }
    }

    pub fn emit(&self) -> Colour {
        match self.material_type {
            MaterialType::DiffuseLight => self.colour,
            _ => ORIGIN // Black
        }
    }

    fn lambertian(&self, ray: &Ray, hit_record: &HitRecord, attenuation: &mut Colour, scattered: &mut Ray) -> bool {
        let mut scatter_direction = hit_record.normal + Vector3::random_in_hemisphere(&hit_record.normal).unit();

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        *scattered = Ray::new(hit_record.origin, scatter_direction);
        *attenuation = self.colour.clone();
        true
    }

    fn metal(&self, ray: &Ray, hit_record: &HitRecord, attenuation: &mut Colour, fuzz: f32, scattered: &mut Ray) -> bool {
        let reflected = ray.direction.unit().reflect(hit_record.normal);
        *scattered = Ray::new(hit_record.origin, reflected + fuzz * Vector3::random_in_unit_sphere());
        *attenuation = self.colour.clone();

        scattered.direction.dot(hit_record.normal) > 0.0
    }

    fn dielectric(&self, ray: &Ray, hit_record: &HitRecord, attenuation: &mut Colour, refraction_index: f32, scattered: &mut Ray) -> bool {
        *attenuation = Colour::new(1.0, 1.0, 1.0);
        let refraction_index = if hit_record.front_face {1.0 / refraction_index} else {refraction_index};

        let unit_direction = ray.direction.unit();

        let cos_theta = (hit_record.normal.dot(-unit_direction)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let direction;

        if refraction_index * sin_theta > 1.0 
        || Material::reflectance(cos_theta, refraction_index) > random_f32() {
            direction = unit_direction.reflect(hit_record.normal);
        }
        else {
            direction = unit_direction.refract(hit_record.normal, refraction_index);
        }

        *scattered = Ray::new(hit_record.origin, direction);

        true
    }

    fn reflectance(cosine: f32, refrac_idx: f32) -> f32 {
        let mut r0 = (1.0 - refrac_idx) / (1.0 + refrac_idx);
        r0 = r0 * r0;
        
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

