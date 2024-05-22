use rand::random;

use crate::{
    hittable::HitRecord,
    primitive::{color::Color, ray::Ray},
};

use super::Material;

// Glass material
#[derive(Debug)]
pub struct Dielectric {
    // For now, this implies the material's refractive index
    // over the refractive index of the enclosing media.
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        // Use Schlick's approximation for reflectance
        let r0 = ((1. - refraction_index) / (1. + refraction_index)).powi(2);
        r0 + (1. - r0) * (1. - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        // White: no color attenuation!
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if hit_record.front_face {
            1. / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = r_in.direction().unit();
        let cos_theta = (-unit_direction).dot(&hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0; // breaking snell's law

        let direction = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > random::<f64>()
        {
            // reflected
            unit_direction.reflect(&hit_record.normal)
        } else {
            // refracted
            unit_direction.refract(&hit_record.normal, refraction_ratio)
        };
        *scattered = Ray::new(hit_record.p, direction);
        true
    }
}
