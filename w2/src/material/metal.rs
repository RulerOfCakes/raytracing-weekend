use crate::{
    hittable::HitRecord,
    primitive::{color::Color, ray::Ray, vec3::Vec3},
};

use super::Material;

#[derive(Debug)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected: Vec3 = r_in.direction().reflect(&hit_record.normal);
        reflected = reflected.unit() + Vec3::random_unit() * self.fuzz;
        *scattered = Ray::new(hit_record.p, reflected, r_in.time());
        *attenuation = self.albedo;

        scattered.direction().dot(&hit_record.normal) > 0.0
    }
}
