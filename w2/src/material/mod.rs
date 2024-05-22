use crate::{
    hittable::HitRecord,
    primitive::{color::Color, ray::Ray},
};

pub trait Material: std::fmt::Debug {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

pub mod dielectric;
pub mod lambertian;
pub mod metal;
