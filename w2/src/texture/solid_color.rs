use crate::primitive::color::Color;

use super::Texture;

#[derive(Debug)]
pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Texture for SolidColor {
    fn value(
        &self,
        _u: f64,
        _v: f64,
        _p: &crate::primitive::point3::Point3,
    ) -> crate::primitive::color::Color {
        self.albedo
    }
}
