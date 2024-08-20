use std::rc::Rc;

use crate::{
    hittable::HitRecord,
    primitive::{color::Color, ray::Ray, vec3::Vec3},
    texture::{solid_color::SolidColor, Texture},
};

use super::Material;

#[derive(Debug)]
pub struct Isotropic {
    texture: Rc<dyn Texture>,
}

impl Isotropic {
    pub fn new(color: Color) -> Self {
        Self {
            texture: Rc::new(SolidColor::new(color)),
        }
    }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *scattered = Ray::new(hit_record.p, Vec3::random_unit(), r_in.time());
        *attenuation = self
            .texture
            .value(hit_record.u, hit_record.v, &hit_record.p);
        true
    }
}
