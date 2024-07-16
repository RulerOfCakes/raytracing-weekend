use std::rc::Rc;

use crate::{
    hittable::HitRecord,
    primitive::{color::Color, ray::Ray, vec3::Vec3},
    texture::{solid_color::SolidColor, Texture},
};

use super::Material;

#[derive(Debug)]
pub struct Lambertian {
    texture: Rc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self {
            texture: Rc::new(SolidColor::new(albedo)),
        }
    }
}

impl From<Rc<dyn Texture>> for Lambertian {
    fn from(texture: Rc<dyn Texture>) -> Self {
        Self { texture }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        // Normal is unit length
        // we choose an arbitrary scatter direction with probability
        // proportional to the cosine of the angle between the normal
        let mut scatter_direction = hit_record.normal + Vec3::random_unit();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        *scattered = Ray::new(hit_record.p, scatter_direction, r_in.time());
        *attenuation = self
            .texture
            .value(hit_record.u, hit_record.v, &hit_record.p);
        true
    }
}
