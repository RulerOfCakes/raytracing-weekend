use crate::{
    hittable::HitRecord,
    primitive::{color::Color, point3::Point3, ray::Ray},
};

pub trait Material: std::fmt::Debug {
    fn scatter(
        &self,
        _r_in: &Ray,
        _hit_record: &HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }

    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}

pub mod dielectric;
pub mod diffuse_light;
pub mod isotropic;
pub mod lambertian;
pub mod metal;
